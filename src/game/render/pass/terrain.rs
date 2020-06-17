use core::amethyst::{
    assets::AssetStorage,
    core::{
        ecs::{Join, Read, ReadExpect, ReadStorage, SystemData, World, WriteExpect},
        transform::Transform,
    },
    renderer::{
        batch::{GroupIterator, OrderedOneLevelBatch},
        pipeline::{PipelineDescBuilder, PipelinesBuilder},
        rendy::{
            command::{QueueId, RenderPassEncoder},
            factory::Factory,
            graph::{
                render::{PrepareResult, RenderGroup, RenderGroupDesc},
                GraphContext, NodeBuffer, NodeImage,
            },
            hal::{self, device::Device, pso},
            mesh::AsVertex,
            shader::Shader,
        },
        sprite::SpriteSheet,
        submodules::{DynamicVertexBuffer, FlatEnvironmentSub, TextureId, TextureSub},
        types::{Backend, Texture},
        util, Camera,
    },
    window::ScreenDimensions,
};
use core::{Point3, Vector2};
use std::time::SystemTime;

use crate::game::{
    components::PassInfo,
    render::{
        pod::{SpriteArgs, TerrainArgs},
        SPRITE_FRAGMENT, SPRITE_VERTEX,
    },
    sprite::SpriteSheetStorage,
};
use libdwarf::components::{EntityInfo, Terrain};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct DrawTerrainDesc;
impl DrawTerrainDesc {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<B: Backend> RenderGroupDesc<B, World> for DrawTerrainDesc {
    fn build(
        self,
        _ctx: &GraphContext<B>,
        factory: &mut Factory<B>,
        _queue: QueueId,
        _world: &World,
        framebuffer_width: u32,
        framebuffer_height: u32,
        subpass: hal::pass::Subpass<'_, B>,
        _buffers: Vec<NodeBuffer>,
        _images: Vec<NodeImage>,
    ) -> Result<Box<dyn RenderGroup<B, World>>, failure::Error> {
        #[cfg(feature = "profiler")]
        profile_scope!("build_trans");

        let env = FlatEnvironmentSub::new(factory)?;
        let textures = TextureSub::new(factory)?;
        let vertex = DynamicVertexBuffer::new();

        let (pipeline, pipeline_layout) = build_terrain_pipeline(
            factory,
            subpass,
            framebuffer_width,
            framebuffer_height,
            true,
            vec![env.raw_layout(), textures.raw_layout()],
        )?;

        Ok(Box::new(DrawTerrain::<B> {
            pipeline,
            pipeline_layout,
            env,
            textures,
            vertex,
            sprites: Default::default(),
            change: Default::default(),
        }))
    }
}

/// Draws triangles to the screen.
#[derive(Debug)]
pub struct DrawTerrain<B: Backend> {
    pipeline: B::GraphicsPipeline,
    pipeline_layout: B::PipelineLayout,
    env: FlatEnvironmentSub<B>,
    textures: TextureSub<B>,
    vertex: DynamicVertexBuffer<B, TerrainArgs>,
    sprites: OrderedOneLevelBatch<TextureId, TerrainArgs>,
    change: util::ChangeDetection,
}

impl<B: Backend> RenderGroup<B, World> for DrawTerrain<B> {
    fn prepare(
        &mut self,
        factory: &Factory<B>,
        _queue: QueueId,
        index: usize,
        _subpass: hal::pass::Subpass<'_, B>,
        world: &World,
    ) -> PrepareResult {
        #[cfg(feature = "profiler")]
        profile_scope!("prepare terrain pass");

        let (
            sheet_storage,
            spritesheet,
            tex_storage,
            cameras,
            screen,
            terrain,
            entity_infos,
            transforms,
            passinfo,
        ) = <(
            Option<Read<'_, SpriteSheetStorage>>,
            Read<'_, AssetStorage<SpriteSheet>>,
            Read<'_, AssetStorage<Texture>>,
            ReadStorage<'_, Camera>,
            ReadExpect<'_, ScreenDimensions>,
            ReadStorage<'_, Terrain>,
            ReadStorage<'_, EntityInfo>,
            ReadStorage<'_, Transform>,
            Option<WriteExpect<'_, PassInfo>>,
        )>::fetch(world);

        // No camera? Skip render.
        let camera_info = (&cameras, &transforms).join().next().or(None);

        self.env.process(factory, index, world);
        self.sprites.swap_clear();
        let mut changed = false;

        let sprites_ref = &mut self.sprites;
        let textures_ref = &mut self.textures;

        if camera_info.is_some() && sheet_storage.is_some() {
            let now = SystemTime::now();

            #[cfg(feature = "profiler")]
            profile_scope!("gather_sprites_trans");

            let sheet_storage = sheet_storage.unwrap();
            let screen_dim = Vector2::new(screen.width(), screen.height());
            let (camera, cam_transform) = camera_info.unwrap();
            let top_left = camera.projection().screen_to_world_point(
                Point3::new(0.0, 0.0, cam_transform.translation().z),
                screen_dim,
                cam_transform,
            );

            let bottom_right = camera.projection().screen_to_world_point(
                Point3::new(
                    screen.width() as f32,
                    screen.height() as f32,
                    cam_transform.translation().z,
                ),
                screen_dim,
                cam_transform,
            );

            // TODO: Filter out tiles no in view
            let mut entities: Vec<(&Terrain, &EntityInfo, &Transform)> =
                (&terrain, &entity_infos, &transforms)
                    .join()
                    .into_iter()
                    .collect();

            // Depth sort tiles
            entities.sort_by(|(_, a, _), (_, b, _)| {
                let ta = a.pos;
                let tb = b.pos;

                (ta.x + ta.y + ta.z)
                    .partial_cmp(&(tb.x + tb.y + tb.z))
                    .unwrap()
                    .reverse()
            });

            entities
                .iter()
                .filter_map(|(terrain, _, transform)| {
                    if (transform.translation().x < top_left.x
                        || transform.translation().x > bottom_right.x)
                        && (transform.translation().y < top_left.y
                            || transform.translation().y > bottom_right.y)
                    {
                        return None;
                    }

                    if let Some((batch_data, texture)) = TerrainArgs::from_data(
                        &tex_storage,
                        spritesheet.get(&sheet_storage.terrain).unwrap(),
                        terrain,
                        transform,
                        None,
                    ) {
                        if let Some((tex_id, this_changed)) = textures_ref.insert(
                            factory,
                            world,
                            texture,
                            hal::image::Layout::ShaderReadOnlyOptimal,
                        ) {
                            changed = changed || this_changed;
                            return Some((tex_id, batch_data));
                        }
                    }
                    None
                })
                .for_each_group(|tex_id, batch_data| {
                    sprites_ref.insert(tex_id, batch_data.drain(..))
                });

            if let Some(mut passinfo) = passinfo {
                passinfo.num_entities = Some(entities.len());
                passinfo.walltime = Some(now.elapsed().unwrap().as_millis());
            }
        }

        self.textures.maintain(factory, world);
        changed = changed || self.sprites.changed();

        {
            #[cfg(feature = "profiler")]
            profile_scope!("write");

            self.vertex.write(
                factory,
                index,
                self.sprites.count() as u64,
                Some(self.sprites.data()),
            );
        }

        self.change.prepare_result(index, changed)
    }

    fn draw_inline(
        &mut self,
        mut encoder: RenderPassEncoder<'_, B>,
        index: usize,
        _subpass: hal::pass::Subpass<'_, B>,
        _world: &World,
    ) {
        #[cfg(feature = "profiler")]
        profile_scope!("draw transparent");

        let layout = &self.pipeline_layout;
        encoder.bind_graphics_pipeline(&self.pipeline);
        self.env.bind(index, layout, 0, &mut encoder);
        self.vertex.bind(index, 0, 0, &mut encoder);
        for (&tex, range) in self.sprites.iter() {
            if self.textures.loaded(tex) {
                self.textures.bind(layout, 1, tex, &mut encoder);
                unsafe {
                    encoder.draw(0..4, range);
                }
            }
        }
    }

    fn dispose(self: Box<Self>, factory: &mut Factory<B>, _aux: &World) {
        unsafe {
            factory.device().destroy_graphics_pipeline(self.pipeline);
            factory
                .device()
                .destroy_pipeline_layout(self.pipeline_layout);
        }
    }
}

fn build_terrain_pipeline<B: Backend>(
    factory: &Factory<B>,
    subpass: hal::pass::Subpass<'_, B>,
    framebuffer_width: u32,
    framebuffer_height: u32,
    transparent: bool,
    layouts: Vec<&B::DescriptorSetLayout>,
) -> Result<(B::GraphicsPipeline, B::PipelineLayout), failure::Error> {
    let pipeline_layout = unsafe {
        factory
            .device()
            .create_pipeline_layout(layouts, None as Option<(_, _)>)
    }?;

    let shader_vertex = unsafe { SPRITE_VERTEX.module(factory).unwrap() };
    let shader_fragment = unsafe { SPRITE_FRAGMENT.module(factory).unwrap() };

    let pipes = PipelinesBuilder::new()
        .with_pipeline(
            PipelineDescBuilder::new()
                .with_vertex_desc(&[(SpriteArgs::vertex(), pso::VertexInputRate::Instance(1))])
                .with_input_assembler(pso::InputAssemblerDesc::new(hal::Primitive::TriangleStrip))
                .with_shaders(util::simple_shader_set(
                    &shader_vertex,
                    Some(&shader_fragment),
                ))
                .with_layout(&pipeline_layout)
                .with_subpass(subpass)
                .with_framebuffer_size(framebuffer_width, framebuffer_height)
                .with_blend_targets(vec![pso::ColorBlendDesc {
                    mask: pso::ColorMask::ALL,
                    blend: if transparent {
                        Some(pso::BlendState::PREMULTIPLIED_ALPHA)
                    } else {
                        None
                    },
                }])
                .with_depth_test(pso::DepthTest {
                    fun: pso::Comparison::Less,
                    write: !transparent,
                }),
        )
        .build(factory, None);

    unsafe {
        factory.destroy_shader_module(shader_vertex);
        factory.destroy_shader_module(shader_fragment);
    }

    match pipes {
        Err(e) => {
            unsafe {
                factory.device().destroy_pipeline_layout(pipeline_layout);
            }
            Err(e)
        }
        Ok(mut pipes) => Ok((pipes.remove(0), pipeline_layout)),
    }
}
