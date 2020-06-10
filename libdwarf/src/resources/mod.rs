use std::collections::{HashMap, HashSet};

use core::{amethyst::ecs, EntityId, Uuid, WorldPos};
use libterrain::{ChunkEntity, ChunkId, TerrainLoader};

mod task_queue;
pub mod time;
pub use task_queue::*;

pub struct World {
    pub entity_map: HashMap<Uuid, EntityId>,
    pub terrain: TerrainLoader,
    pub visible_chunks: HashSet<ChunkId>,
}

impl World {
    pub fn new(_ecs: &mut ecs::World, terrain: TerrainLoader) -> World {
        World {
            entity_map: HashMap::new(),
            terrain,
            visible_chunks: HashSet::new(),
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
