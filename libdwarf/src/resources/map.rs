use libterrain::{Biome, TerrainGenerator};
use std::collections::HashMap;

pub struct Map {
    // TODO: Support multiple objects per tile.
    pub object_map: HashMap<(u32, u32), u32>,
    /// Location map of all the workers.
    pub worker_map: HashMap<(u32, u32), u32>,
    pub terrain: TerrainGenerator,
    // World dimensions
    pub width: u32,
    pub height: u32,
}

impl Map {
    pub fn initialize(width: u32, height: u32) -> Self {
        let terrain = TerrainGenerator::new(width, height).build();

        Map {
            object_map: HashMap::new(),
            worker_map: HashMap::new(),
            terrain,
            width,
            height,
        }
    }

    pub fn is_inside_map(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32
    }

    /// Find the north, east, south, west neighboring objects for some
    /// point <x, y>.
    pub fn find_neighbors(&self, x: u32, y: u32) -> Vec<&u32> {
        // Generate the coordinates for the neighbors
        let mut neighbor_idx = Vec::with_capacity(4);
        neighbor_idx.push((x, y + 1));
        neighbor_idx.push((x + 1, y));
        if y > 0 {
            neighbor_idx.push((x, y - 1));
        }
        if x > 0 {
            neighbor_idx.push((x - 1, y));
        }

        // Find the neighbors and return the results
        let mut results = Vec::new();
        for idx in neighbor_idx.iter() {
            if let Some(oid) = self.object_map.get(idx) {
                results.push(oid);
            }
        }
        results
    }

    pub fn has_collision(&self, x: i32, y: i32) -> bool {
        if self.is_inside_map(x, y) {
            return self.object_map.contains_key(&(x as u32, y as u32))
                || self.worker_map.contains_key(&(x as u32, y as u32));
        }

        false
    }

    pub fn objects_at(&self, x: i32, y: i32) -> Option<u32> {
        if self.is_inside_map(x, y) {
            if let Some(id) = self.object_map.get(&(x as u32, y as u32)) {
                return Some(*id);
            }
        }

        None
    }

    pub fn terrain_at(&self, x: i32, y: i32) -> Option<Biome> {
        if self.is_inside_map(x, y) {
            Some(self.terrain.get_biome(x as usize, y as usize))
        } else {
            None
        }
    }

    pub fn worker_at(&self, x: i32, y: i32) -> Option<u32> {
        if self.is_inside_map(x, y) {
            if let Some(id) = self.worker_map.get(&(x as u32, y as u32)) {
                return Some(*id);
            }
        }

        None
    }

    pub fn move_worker(&mut self, entity: u32, old_x: u32, old_y: u32, new_x: u32, new_y: u32) {
        self.worker_map.remove(&(old_x, old_y));
        self.track_worker(entity, new_x, new_y);
    }

    pub fn remove_object(&mut self, _entity: u32, x: u32, y: u32) {
        self.object_map.remove(&(x, y));
    }

    pub fn track_object(&mut self, entity: u32, x: u32, y: u32) {
        self.object_map.insert((x, y), entity);
    }

    pub fn track_worker(&mut self, entity: u32, x: u32, y: u32) {
        self.worker_map.insert((x, y), entity);
    }
}
