use std::collections::HashMap;

mod chunk;
pub use chunk::{Biome, Object, TerrainChunk, ZLEVELS};

mod generator;
pub use generator::TerrainGenerator;

mod poisson;

use core::Point3;
pub type Path = Vec<Point3<u32>>;

// Loads terrain
pub struct TerrainLoader {
    chunk_width: u32,
    chunk_height: u32,
    half_width: f32,
    half_height: f32,
    /// Currently loaded chunks
    pub chunks: HashMap<(i32, i32), TerrainChunk>,
}

impl TerrainLoader {
    pub fn new(chunk_width: u32, chunk_height: u32) -> Self {
        TerrainLoader {
            chunk_width,
            chunk_height,
            half_width: chunk_width as f32 / 2.0,
            half_height: chunk_height as f32 / 2.0,
            chunks: HashMap::new(),
        }
    }

    pub fn get_chunk(&mut self, x: i32, y: i32) -> TerrainChunk {
        if let Some(chunk) = self.chunks.get(&(x, y)) {
            return chunk.clone();
        }

        // TODO: Check file system
        // if let Some(chunk) = self.loader.loader(&(x, y)) {
        //     return chunk.clone();
        // }

        let tergen = TerrainGenerator::new(self.chunk_width, self.chunk_height)
            .chunk_coord(x, y)
            .build();

        let chunk = tergen.get_terrain();
        self.chunks.insert((x, y), chunk.clone());

        chunk.clone()
    }

    pub fn get_terrain(&mut self, pt: &Point3<i32>) -> Option<Biome> {
        // Grab the chunk this point would be in.
        let coord = self.to_chunk_coord(pt);
        let chunk = self.get_chunk(coord.0, coord.1);

        // Transform world coordinate to chunk coordinate
        let chunk_coord: Point3<i32> = Point3::new(
            (pt.x as f32 + self.half_width) as i32,
            (pt.y as f32 + self.half_height) as i32,
            pt.z,
        );

        chunk.get(
            chunk_coord.x as u32,
            chunk_coord.y as u32,
            chunk_coord.z as u32,
        )
    }

    pub fn get_topo(&self, x: i32, y: i32) -> Vec<Option<Biome>> {
        let tergen = TerrainGenerator::new(self.chunk_width, self.chunk_height)
            .chunk_coord(x, y)
            .build();

        tergen.topo
    }

    /// Is this point reachable?
    /// A space is passable if there is no block there and a block below.
    pub fn is_walkable(&mut self, pt: &Point3<i32>) -> bool {
        if pt.z == 0 {
            return self.get_terrain(pt).is_none();
        }

        let pt_below = Point3::new(pt.x, pt.y, pt.z - 1);
        self.get_terrain(pt).is_none() && self.get_terrain(&pt_below).is_some()
    }
    /// Find neighboring points
    pub fn neighbors(&mut self, pt: &Point3<i32>) -> Vec<(Point3<i32>, usize)> {
        let mut results = Vec::new();

        if pt.z > 0 {
            self.neighbors_for_level(&mut results, pt, (pt.z - 1) as u32);
        }

        self.neighbors_for_level(&mut results, pt, pt.z as u32);

        if pt.z < ZLEVELS as i32 {
            self.neighbors_for_level(&mut results, pt, (pt.z + 1) as u32);
        }

        results
            .into_iter()
            .filter(|pt| self.is_walkable(pt))
            .collect::<Vec<Point3<i32>>>()
            .into_iter()
            // TODO: Make difficult terrain have a higher cost.
            .map(|pt| (pt, 1 as usize))
            .collect()
    }

    pub fn neighbors_for_level(&self, neighbors: &mut Vec<Point3<i32>>, pt: &Point3<i32>, zlevel: u32) {
        let (x, y) = (pt.x, pt.y);

        neighbors.push(Point3::new(x, y - 1, zlevel as i32));
        neighbors.push(Point3::new(x - 1, y, zlevel as i32));
        neighbors.push(Point3::new(x + 1, y, zlevel as i32));
        neighbors.push(Point3::new(x, y + 1, zlevel as i32));
    }

    pub fn to_chunk_coord(&self, pt: &Point3<i32>) -> (i32, i32) {
        // world point to chunk coordinate
        let chunk_x = (pt.x as f32 / self.half_width) as i32;
        let chunk_y = (pt.y as f32 / self.half_height) as i32;

        (chunk_x as i32, chunk_y as i32)
    }
}
