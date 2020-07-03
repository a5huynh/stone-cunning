use std::collections::{HashMap, HashSet};

use core::{amethyst::ecs, EntityId, Uuid, WorldPos};

use crate::Direction;
use libterrain::{ChunkEntity, ChunkId, TerrainLoader, ZLEVELS};

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

    /// Get the visible tile underneath the screen coordinate.
    pub fn visible_tile_at(&mut self, x: i32, y: i32, rotation: Direction) -> WorldPos {
        // From the view port, the tallest z level from lower (x,y) coordinates will
        // show up over ones from higher ones.
        let mut current_pt = WorldPos::new(x, y, 0);
        let mut above_pt = WorldPos::new(0, 0, 0);
        let mut valid_pt = None;

        // Start at the highest point
        for z in 0..ZLEVELS as i32 {
            current_pt.z = z;
            // Pointer to terrain above the current tile.
            above_pt.x = current_pt.x;
            above_pt.y = current_pt.y;
            above_pt.z = z + 1;

            // Loop until we find the first piece of visible terrain.
            let biome = self.terrain.get(&current_pt);
            // Are we at the top?
            if z == (ZLEVELS as i32 - 1) && biome.is_some() {
                valid_pt = Some(current_pt);
                break;
            }

            let above = self.terrain.get(&above_pt);

            if biome.is_some() && above.is_none() {
                if let Some(ChunkEntity::Terrain { .. }) = biome {
                    // Last valid point we've seen.
                    valid_pt = Some(current_pt);
                }
            }

            // Based on the current rotation, we'll want to search for the
            // correct z-level in different ways.
            match rotation {
                Direction::NORTH => {
                    current_pt.x -= 1;
                    current_pt.y -= 1;
                }
                Direction::EAST => {
                    current_pt.x -= 1;
                    current_pt.y += 1;
                }
                Direction::SOUTH => {
                    current_pt.x += 1;
                    current_pt.y += 1;
                }
                Direction::WEST => {
                    current_pt.x += 1;
                    current_pt.y -= 1;
                }
            }
        }

        if valid_pt.is_none() {
            current_pt
        } else {
            valid_pt.unwrap()
        }
    }
}
