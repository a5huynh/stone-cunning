use core::{Point3, Uuid};

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
    Object(Uuid, ObjectType),
    Worker(Uuid),
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

    /// Get chunk data at a specific position
    pub fn get(&self, pt: &ChunkPos) -> Option<ChunkEntity> {
        self.grid[self.idx(pt)].clone()
    }

    /// Set chunk data at a specific position
    pub fn set(&mut self, pt: &ChunkPos, entity: Option<ChunkEntity>) {
        let idx = self.idx(&pt);
        self.grid[idx] = entity;
    }
}
