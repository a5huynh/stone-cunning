use core::Point3;
use std::collections::HashMap;

// TODO: Load from config file
#[derive(Clone, Debug, PartialEq)]
pub enum Biome {
    // Above ground biomes
    OCEAN,
    BEACH,
    GRASSLAND,
    TAIGA,
    TUNDRA,
    SNOW,
    // Underground biomes
    ROCK,
}

// TODO: Load from config file
#[derive(Clone, Debug)]
pub enum Object {
    TREE,
}

#[derive(Clone)]
pub struct TerrainChunk {
    /// TODO: Handle multiple objects per tile.
    pub objects: HashMap<Point3<i32>, Object>,
    grid: Vec<Option<Biome>>,
    pub height: u32,
    pub width: u32,
}

pub const ZLEVELS: u32 = 64;

impl TerrainChunk {
    pub fn new(width: u32, height: u32) -> TerrainChunk {
        TerrainChunk {
            width,
            height,
            grid: vec![None; (width * height * ZLEVELS) as usize],
            objects: HashMap::new(),
        }
    }

    fn idx(&self, x: u32, y: u32, z: u32) -> usize {
        (z * (self.width * self.height) as u32 + y * self.width as u32 + x) as usize
    }

    /// Determines whether the block @ (x, y, z) is visible.
    pub fn is_visible(&self, x: u32, y: u32, z: u32) -> bool {
        // Top level is always exposed.
        if z == ZLEVELS - 1 {
            return true;
        }

        let start_x = match x {
            0 => 0,
            _ => x - 1,
        };

        let start_y = match y {
            0 => 0,
            _ => y - 1,
        };

        let start_z = match z {
            0 => 0,
            _ => z - 1,
        };

        let end_x = (x + 1).min(self.width as u32 - 1);
        let end_y = (y + 1).min(self.height as u32 - 1);
        let end_z = (z + 1).min(ZLEVELS - 1);

        // If any side is exposed to air, the block is visible.
        for ix in start_x..=end_x {
            for iy in start_y..=end_y {
                for iz in start_z..=end_z {
                    if self.get(ix, iy, iz).is_none() {
                        return true;
                    }
                }
            }
        }

        false
    }

    /// Get chunk data at a specific position
    pub fn get(&self, x: u32, y: u32, z: u32) -> Option<Biome> {
        self.grid[self.idx(x, y, z)].clone()
    }

    /// Set chunk data at a specific position
    pub fn set(&mut self, pt: (u32, u32, u32), biome: Option<Biome>) {
        let idx = self.idx(pt.0, pt.1, pt.2);
        self.grid[idx] = biome;
    }

    pub fn set_object(&mut self, pt: &Point3<i32>, obj: Object) {
        self.objects.insert(*pt, obj);
    }
}
