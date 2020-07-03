use core::amethyst::{
    core::transform::Transform,
    ecs::{Entities, ReadExpect, System, WriteExpect, WriteStorage},
    renderer::SpriteRender,
};
use core::log::info;
use core::{Uuid, WorldPos};
use libdwarf::{
    components::{EntityInfo, Terrain},
    resources::World,
};
use libterrain::{Biome, ChunkEntity, ZLEVELS};

use crate::game::{
    resources::{MapRenderer, ViewShed},
    sprite::SpriteSheetStorage,
};

pub struct RenderTerrainSystem;
impl<'a> System<'a> for RenderTerrainSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, EntityInfo>,
        WriteStorage<'a, Terrain>,
        WriteStorage<'a, Transform>,
        WriteExpect<'a, World>,
        ReadExpect<'a, MapRenderer>,
        WriteExpect<'a, ViewShed>,
        WriteStorage<'a, SpriteRender>,
        ReadExpect<'a, SpriteSheetStorage>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut entity_info,
            mut terrain_storage,
            mut transforms,
            mut world,
            map_render,
            mut viewshed,
            mut sprites,
            sheets,
        ): Self::SystemData,
    ) {
        // Should we check for chunks to load / remove?
        if !viewshed.needs_chunking {
            return;
        }

        let tl_chunk = world
            .terrain
            .to_chunk_coord(&viewshed.top_left_world.unwrap());
        let br_chunk = world
            .terrain
            .to_chunk_coord(&viewshed.bottom_right_world.unwrap());

        for x in br_chunk.0..=tl_chunk.0 {
            for y in br_chunk.1..=tl_chunk.1 {
                // TODO:
                // - Compare to list of visible chunks, if not in list, load
                //   chunk and add entities for chunk & add chunk to visible chunk list
                // - if already loaded, don't do anything.
                if world.visible_chunks.contains(&(x, y)) {
                    continue;
                } else {
                    info!("Loading chunk: {:?}", (x, y));
                    // Track chunk as visible.
                    world.visible_chunks.insert((x, y));
                    let chunk = world.terrain.get_chunk(x, y);
                    let start_x = x * chunk.width as i32;
                    let start_y = y * chunk.height as i32;

                    for y in (start_y..start_y + chunk.height as i32).rev() {
                        for x in (start_x..start_x + chunk.width as i32).rev() {
                            for z in 32..ZLEVELS {
                                let pt = WorldPos::new(x as i32, y as i32, z as i32);
                                match chunk.get_world(&pt) {
                                    Some(ChunkEntity::Terrain {
                                        biome,
                                        visible_faces,
                                    }) => {
                                        if !visible_faces.iter().any(|b| *b) {
                                            continue;
                                        }

                                        // Determine which sprite to use
                                        let sprite_idx = match biome {
                                            Biome::TAIGA => 0,
                                            Biome::SNOW | Biome::TUNDRA => 1,
                                            Biome::GRASSLAND => 2,
                                            Biome::OCEAN => 3,
                                            Biome::BEACH => 4,
                                            Biome::ROCK => 5,
                                        };

                                        entities
                                            .build_entity()
                                            // Grid position
                                            .with(
                                                EntityInfo {
                                                    uuid: Uuid::new_v4(),
                                                    pos: pt,
                                                    z_offset: 0.0,
                                                    needs_delete: false,
                                                    needs_update: true,
                                                },
                                                &mut entity_info,
                                            )
                                            .with(
                                                SpriteRender {
                                                    sprite_sheet: sheets.terrain.clone(),
                                                    sprite_number: sprite_idx,
                                                },
                                                &mut sprites,
                                            )
                                            .with(Terrain { biome }, &mut terrain_storage)
                                            .with(map_render.place(&pt, 0.0), &mut transforms)
                                            .build();
                                    }
                                    Some(ChunkEntity::Object(_uuid, _object_type)) => {}
                                    _ => {}
                                }
                            }
                        }
                    }
                }

                // TODO: Clean up chunks that are no londer in view?
            }

            viewshed.needs_chunking = false;
        }
    }
}
