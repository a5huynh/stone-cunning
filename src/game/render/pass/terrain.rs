use core::amethyst::{
    assets::AssetStorage,
    core::{
        ecs::{Join, Read, ReadExpect, ReadStorage, SystemData, World},
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
        util,
    },
};

use crate::game::{
    render::{
        pod::{SpriteArgs, TerrainArgs},
        SPRITE_FRAGMENT, SPRITE_VERTEX,
    },
    sprite::SpriteSheetStorage,
};
use libdwarf::components::Terrain;

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

        let (sheet_storage, spritesheet, tex_storage, terrain, transforms) =
            <(
                Option<Read<'_, SpriteSheetStorage>>,
                Read<'_, AssetStorage<SpriteSheet>>,
                Read<'_, AssetStorage<Texture>>,
                ReadStorage<'_, Terrain>,
                ReadStorage<'_, Transform>,
            )>::fetch(world);

        self.env.process(factory, index, world);
        self.sprites.swap_clear();
        let mut changed = false;

        let sprites_ref = &mut self.sprites;
        let textures_ref = &mut self.textures;

        if let Some(sheet_storage) = sheet_storage {
            #[cfg(feature = "profiler")]
            profile_scope!("gather_sprites_trans");

            (&terrain, &transforms)
                .join()
                .into_iter()
                .filter_map(|(terrain, transform)| {
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
