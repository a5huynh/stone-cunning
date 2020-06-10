use core::amethyst::{
    core::transform::Transform,
    ecs::{Entities, Entity, Join, ReadExpect, System, WriteExpect, WriteStorage},
    renderer::{SpriteRender, Transparent},
};
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
        ReadExpect<'a, SpriteSheetStorage>,
        WriteStorage<'a, SpriteRender>,
        WriteStorage<'a, Transparent>,
        WriteStorage<'a, Transform>,
        WriteExpect<'a, World>,
        ReadExpect<'a, MapRenderer>,
        WriteExpect<'a, ViewShed>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut entity_info,
            mut terrain_storage,
            sheets,
            mut sprites,
            mut transparents,
            mut transforms,
            mut world,
            map_render,
            mut viewshed,
        ): Self::SystemData,
    ) {
        // Should we check for chunks to load / remove?
        if viewshed.dirty {
            let tl_chunk = world.terrain.to_chunk_coord(&viewshed.top_left.unwrap());
            let br_chunk = world
                .terrain
                .to_chunk_coord(&viewshed.bottom_right.unwrap());

            for x in br_chunk.0..=tl_chunk.0 {
                for y in br_chunk.1..=tl_chunk.1 {
                    // TODO:
                    // - Compare to list of visible chunks, if not in list, load
                    //   chunk and add entities for chunk & add chunk to visible chunk list
                    // - if already loaded, don't do anything.
                    if world.visible_chunks.contains(&(x, y)) {
                        continue;
                    } else {
                        println!("Loading chunk: {:?}", (x, y));
                        // Track chunk as visible.
                        world.visible_chunks.insert((x, y));
                        let chunk = world.terrain.get_chunk(x, y);
                        let start_x = x * chunk.width as i32;
                        let start_y = y * chunk.height as i32;

                        for y in start_y..start_y + chunk.height as i32 {
                            for x in start_x..start_x + chunk.width as i32 {
                                for z in 32..ZLEVELS {
                                    let pt = WorldPos::new(x as i32, y as i32, z as i32);
                                    match chunk.get_world(&pt) {
                                        Some(ChunkEntity::Terrain { biome, visible }) => {
                                            if !visible {
                                                continue;
                                            }

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
                                                .with(Terrain { biome }, &mut terrain_storage)
                                                .build();
                                        }
                                        Some(ChunkEntity::Object(_uuid, _object_type)) => {}
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                    // Things to think about, cleaning up chunks?
                }
            }

            viewshed.dirty = false;
        }

        // Find objects that don't have a sprite and give it one.
        let invisible: Vec<(Entity, &mut EntityInfo, &Terrain, ())> =
            (&*entities, &mut entity_info, &terrain_storage, !&sprites)
                .join()
                .collect();

        for (entity, info, terrain, _) in invisible {
            if !info.needs_update {
                continue;
            }

            // Apply transformation
            info.needs_update = false;
            let pos = info.pos;
            transforms
                .insert(entity, map_render.place(&pos, 0.0))
                .unwrap();

            // Determine which sprite to use
            let sprite_idx = match terrain.biome {
                Biome::TAIGA => 0,
                Biome::SNOW | Biome::TUNDRA => 1,
                Biome::GRASSLAND => 2,
                Biome::OCEAN => 3,
                Biome::BEACH => 4,
                Biome::ROCK => 5,
            };

            sprites
                .insert(
                    entity,
                    SpriteRender {
                        sprite_sheet: sheets.terrain.clone(),
                        sprite_number: sprite_idx,
                    },
                )
                .unwrap();
            transparents.insert(entity, Transparent).unwrap();
        }
    }
}
