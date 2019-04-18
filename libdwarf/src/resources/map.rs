use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Terrain {
    STONE = 0,
    MARBLE = 1,
    GRASS = 2,
    NONE = -1,
}

pub struct Map {
    // TODO: Support multiple objects per tile.
    pub collision_map: HashMap<(u32, u32), u32>,
    pub terrain: HashMap<(u32, u32), Terrain>,
    // World dimensions
    pub width: u32,
    pub height: u32,
}

impl Map {
    pub fn initialize(width: u32, height: u32) -> Self {
        let mut terrain = HashMap::new();
        for y in 0..height {
            for x in 0..width {
                let tile = ((x + y) % 3) as usize;
                let terrain_tile = match tile {
                    0 => Terrain::STONE,
                    1 => Terrain::MARBLE,
                    2 => Terrain::GRASS,
                    _ => Terrain::NONE,
                };

                terrain.insert((x as u32, y as u32), terrain_tile);
            }
        }

        Map {
            collision_map: HashMap::new(),
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
            if let Some(oid) = self.collision_map.get(idx) {
                results.push(oid);
            }
        }
        results
    }

    pub fn has_collision(&self, x: i32, y: i32) -> bool {
        if self.is_inside_map(x, y) {
            return self.collision_map.contains_key(&(x as u32, y as u32));
        }

        false
    }

    pub fn objects_at(&self, x: i32, y: i32) -> Option<u32> {
        if self.is_inside_map(x, y) {
            if let Some(id) = self.collision_map.get(&(x as u32, y as u32)) {
                return Some(*id);
            }
        }

        None
    }

    pub fn terrain_at(&self, x: i32, y: i32) -> Option<Terrain> {
        if self.is_inside_map(x, y) {
            if let Some(terrain) = self.terrain.get(&(x as u32, y as u32)) {
                return Some(terrain.clone());
            }
        }

        None
    }
}
