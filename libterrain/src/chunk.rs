use core::Point3;

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
pub enum ObjectType {
    TREE,
}

#[derive(Clone, Debug)]
pub enum ChunkEntity {
    Terrain(Biome),
    /// TODO: Handle multiple objects per tile.
    Object(ObjectType),
}

// Position inside
pub type ChunkPos = Point3<u32>;

#[derive(Clone)]
pub struct TerrainChunk {
    grid: Vec<Option<ChunkEntity>>,
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
        }
    }

    fn idx(&self, pt: &ChunkPos) -> usize {
        (pt.z * (self.width * self.height) as u32 + pt.y * self.width as u32 + pt.x) as usize
    }

    /// Determines whether the block @ (x, y, z) is visible.
    /// TODO: Store visibility as part of ChunkEntity
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
                    let pt = Point3::new(ix, iy, iz);
                    if self.get(&pt).is_none() {
                        return true;
                    }
                }
            }
        }

        false
    }

    /// Get chunk data at a specific position
    pub fn get(&self, pt: &ChunkPos) -> Option<ChunkEntity> {
        self.grid[self.idx(pt)].clone()
    }

    /// Set chunk data at a specific position
    pub fn set(&mut self, pt: &ChunkPos, entity: ChunkEntity) {
        let idx = self.idx(&pt);
        self.grid[idx] = Some(entity);
    }
}
