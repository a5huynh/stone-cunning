mod chunk;
pub use chunk::{Biome, Object, TerrainChunk};

mod generator;
pub use generator::TerrainGenerator;

mod poisson;

use core::Point3;
pub type Path = Vec<Point3<u32>>;

// Loads terrain
pub struct TerrainLoader {
    chunk_width: u32,
    chunk_height: u32,
}

impl TerrainLoader {
    pub fn new(chunk_width: u32, chunk_height: u32) -> Self {
        TerrainLoader {
            chunk_width,
            chunk_height,
        }
    }

    pub fn get_chunk(&self, x: i32, y: i32) -> Vec<Option<Biome>> {
        // TODO: Have you loaded this chunk already? Load from memory / disk
        let tergen = TerrainGenerator::new(self.chunk_width, self.chunk_height)
            .chunk_coord(x, y)
            .build();
        tergen.topo.clone()
    }
}
