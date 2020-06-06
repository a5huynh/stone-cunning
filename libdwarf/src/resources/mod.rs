use std::collections::HashMap;

use crate::components::{EntityInfo, Terrain};
use core::{
    amethyst::{
        ecs::{self, WorldExt},
        prelude::*,
    },
    EntityId, Uuid, WorldPos,
};
use libterrain::{ChunkEntity, TerrainLoader, ZLEVELS};

mod task_queue;
pub mod time;
pub use task_queue::*;

pub struct World {
    pub entity_map: HashMap<Uuid, EntityId>,
    pub terrain: TerrainLoader,
}

impl World {
    pub fn new(ecs: &mut ecs::World, terrain: TerrainLoader) -> World {
        let mut terrain = terrain.clone();
        // Add visible world into view
        for y in 0..terrain.chunk_height {
            for x in 0..terrain.chunk_width {
                for z in 32..ZLEVELS {
                    let pt = WorldPos::new(x as i32, y as i32, z as i32);

                    let entity = terrain.get(&pt);
                    match entity {
                        Some(ChunkEntity::Terrain(biome)) => {
                            if !terrain.is_visible(&pt) {
                                continue;
                            }

                            ecs.create_entity()
                                // Grid position
                                .with(EntityInfo {
                                    uuid: Uuid::new_v4(),
                                    pos: pt,
                                    z_offset: 0.0,
                                    needs_delete: false,
                                    needs_update: true,
                                })
                                .with(Terrain { biome })
                                .build();
                        }
                        Some(ChunkEntity::Object(_uuid, _object_type)) => {}
                        _ => {}
                    }
                }
            }
        }

        World {
            entity_map: HashMap::new(),
            terrain,
        }
    }

    pub fn add(&mut self, pos: &WorldPos, entity_ref: EntityId, uuid: Uuid, entity: ChunkEntity) {
        self.entity_map.insert(uuid, entity_ref);
        self.terrain.set(&pos, Some(entity));
    }

    pub fn destroy(&mut self, pos: &WorldPos, uuid: Uuid) {
        self.entity_map.remove(&uuid);
        self.terrain.set(&pos, None);
    }

    pub fn entity(&self, uuid: &Uuid) -> u32 {
        *self.entity_map.get(uuid).unwrap()
    }
}
