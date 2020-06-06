use core::amethyst::{
    core::transform::Transform,
    ecs::{Entities, Entity, Join, ReadExpect, ReadStorage, System, WriteStorage},
    renderer::{SpriteRender, Transparent},
};

use crate::game::{resources::MapRenderer, sprite::SpriteSheetStorage};
use libdwarf::components::{EntityInfo, Terrain};
use libterrain::Biome;

pub struct RenderTerrainSystem;
impl<'a> System<'a> for RenderTerrainSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, EntityInfo>,
        ReadStorage<'a, Terrain>,
        ReadExpect<'a, SpriteSheetStorage>,
        WriteStorage<'a, SpriteRender>,
        WriteStorage<'a, Transparent>,
        WriteStorage<'a, Transform>,
        ReadExpect<'a, MapRenderer>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut entity_info,
            terrain_storage,
            sheets,
            mut sprites,
            mut transparents,
            mut transforms,
            map_render,
        ): Self::SystemData,
    ) {
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
