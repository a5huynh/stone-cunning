use core::{Point3, Uuid, WorldPos};

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

pub enum Faces {
    TOP = 0,
    BOTTOM = 1,
    WEST = 2,
    NORTH = 3,
    EAST = 4,
    SOUTH = 5,
}

#[derive(Clone, Debug)]
pub enum ChunkEntity {
    Terrain {
        biome: Biome,
        visible_faces: [bool; 6],
    },
    /// TODO: Handle multiple objects per tile.
    Object(Uuid, ObjectType),
    Worker(Uuid),
}

// (x,y) position of this chunk in the world.
pub type ChunkId = (i32, i32);
// Position inside
pub type ChunkPos = Point3<u32>;

#[derive(Clone)]
pub struct TerrainChunk {
    id: ChunkId,
    grid: Vec<Option<ChunkEntity>>,
    pub height: u32,
    pub width: u32,
}

pub const ZLEVELS: u32 = 64;

impl TerrainChunk {
    pub fn new(id: ChunkId, width: u32, height: u32) -> TerrainChunk {
        TerrainChunk {
            id,
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

    /// Get relative to world position
    pub fn get_world(&self, pt: &WorldPos) -> Option<ChunkEntity> {
        self.get(&self.world_to_local(pt))
    }

    /// Set chunk data at a specific position
    pub fn set(&mut self, pt: &ChunkPos, entity: Option<ChunkEntity>) {
        let idx = self.idx(&pt);
        self.grid[idx] = entity;
    }

    pub fn set_world(&mut self, pt: &WorldPos, entity: Option<ChunkEntity>) {
        self.set(&self.world_to_local(pt), entity);
    }

    pub fn world_to_local(&self, pt: &WorldPos) -> ChunkPos {
        let mut local_x = pt.x % self.width as i32;
        if local_x < 0 {
            local_x += self.width as i32;
        }

        let mut local_y = pt.y % self.height as i32;
        if local_y < 0 {
            local_y += self.height as i32;
        }

        ChunkPos::new(local_x as u32, local_y as u32, pt.z as u32)
    }
}
