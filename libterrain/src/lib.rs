use std::{
    collections::HashMap,
    time::SystemTime,
};

mod chunk;
pub use chunk::{Biome, ChunkEntity, ChunkPos, ObjectType, TerrainChunk, ZLEVELS};

mod generator;
pub use generator::TerrainGenerator;

mod poisson;

use core::{log::info, Point3, WorldPos};
use libpath::find_path;

pub type Path = Vec<WorldPos>;

pub fn heuristic(a: &WorldPos, b: &WorldPos) -> usize {
    (a.x as i32 - b.x as i32).abs() as usize
        + (a.y as i32 - b.y as i32).abs() as usize
        + (a.z as i32 - b.z as i32).abs() as usize
}

// Loads terrain
#[derive(Clone)]
pub struct TerrainLoader {
    pub chunk_width: u32,
    pub chunk_height: u32,
    pub half_width: f32,
    pub half_height: f32,
    /// TODO: REMOVE THESE MAPS
    pub object_map: HashMap<WorldPos, u32>,
    /// Location map of all the workers.
    pub worker_map: HashMap<WorldPos, u32>,
    /// ENDTODO ----
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
            object_map: HashMap::new(),
            worker_map: HashMap::new(),
        }
    }

    pub fn find_path(&mut self, start: &WorldPos, end: &WorldPos) -> Path {
        let (_, path) = find_path(
            *start,
            *end,
            |node| heuristic(&node, start),
            |pt| self.neighbors(pt),
        );

        path
    }

    pub fn get_chunk(&mut self, x: i32, y: i32) -> TerrainChunk {
        if let Some(chunk) = self.chunks.get(&(x, y)) {
            return chunk.clone();
        }

        // TODO: Check file system
        // if let Some(chunk) = self.loader.loader(&(x, y)) {
        //     return chunk.clone();
        // }

        let now = SystemTime::now();
        let tergen = TerrainGenerator::new(self.chunk_width, self.chunk_height)
            .chunk_coord(x, y)
            .build();

        let chunk = tergen.get_terrain();
        self.chunks.insert((x, y), chunk.clone());
        info!("Terrain gen took: {}ms", now.elapsed().unwrap().as_millis());

        chunk.clone()
    }

    pub fn get(&mut self, pt: &WorldPos) -> Option<ChunkEntity> {
        // Grab the chunk this point would be in.
        let coord = self.to_chunk_coord(pt);
        // Transform world coordinate to chunk coordinate
        let chunk = self.get_chunk(coord.0, coord.1);
        let chunk_coord = self.world_to_chunk(pt);

        chunk.get(&chunk_coord)
    }

    pub fn get_topo(&self, x: i32, y: i32) -> Vec<Option<Biome>> {
        let tergen = TerrainGenerator::new(self.chunk_width, self.chunk_height)
            .chunk_coord(x, y)
            .build();

        tergen.topo
    }

    /// Is this point reachable?
    /// A space is passable if there is no block there and a block below.
    pub fn is_walkable(&mut self, pt: &WorldPos) -> bool {
        if pt.z == 0 {
            return self.get(pt).is_none();
        }

        let pt_below = WorldPos::new(pt.x, pt.y, pt.z - 1);
        self.get(pt).is_none() && self.get(&pt_below).is_some()
    }

    /// Determines whether the block @ (x, y, z) is visible.
    /// TODO: Store visibility as part of ChunkEntity
    pub fn is_visible(&mut self, pt: &WorldPos) -> bool {
        // Top level is always exposed.
        if pt.z == (ZLEVELS - 1) as i32 {
            return true;
        }

        let start_x = pt.x - 1;
        let start_y = pt.y - 1;
        let start_z = pt.z - 1;

        let end_x = pt.x + 1;
        let end_y = pt.y + 1;
        let end_z = (pt.z + 1).min((ZLEVELS - 1) as i32);

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

    /// Find neighboring points
    pub fn neighbors(&mut self, pt: &WorldPos) -> Vec<(WorldPos, usize)> {
        let mut results = Vec::new();

        if pt.z > 0 {
            self.neighbors_for_level(&mut results, pt, pt.z - 1);
        }

        self.neighbors_for_level(&mut results, pt, pt.z);

        if pt.z < ZLEVELS as i32 {
            self.neighbors_for_level(&mut results, pt, pt.z + 1);
        }

        results
            .into_iter()
            .filter(|pt| self.is_walkable(pt))
            .collect::<Vec<WorldPos>>()
            .into_iter()
            // TODO: Make difficult terrain have a higher cost.
            .map(|pt| (pt, 1 as usize))
            .collect()
    }

    fn neighbors_for_level(&self, neighbors: &mut Vec<WorldPos>, pt: &WorldPos, zlevel: i32) {
        let (x, y) = (pt.x, pt.y);

        neighbors.push(WorldPos::new(x, y - 1, zlevel));
        neighbors.push(WorldPos::new(x - 1, y, zlevel));
        neighbors.push(WorldPos::new(x + 1, y, zlevel));
        neighbors.push(WorldPos::new(x, y + 1, zlevel));
    }

    pub fn to_chunk_coord(&self, pt: &WorldPos) -> (i32, i32) {
        // world point to chunk coordinate
        let mut chunk_x = pt.x / self.chunk_width as i32;
        if (pt.x % self.chunk_width as i32) < 0 {
            chunk_x -= 1;
        }

        let mut chunk_y = pt.y / self.chunk_height as i32;
        if (pt.y % self.chunk_height as i32) < 0 {
            chunk_y -= 1;
        }

        (chunk_x, chunk_y)
    }

    pub fn world_to_chunk(&self, pt: &WorldPos) -> ChunkPos {
        let mut local_x = pt.x % self.chunk_width as i32;
        if local_x < 0 {
            local_x += self.chunk_width as i32;
        }

        let mut local_y = pt.y % self.chunk_height as i32;
        if local_y < 0 {
            local_y += self.chunk_height as i32;
        }

        ChunkPos::new(local_x as u32, local_y as u32, pt.z as u32)
    }

    pub fn set(&mut self, pt: &WorldPos, entity: Option<ChunkEntity>) {
        let coord = self.to_chunk_coord(pt);
        // Transform world coordinate to chunk coordinate
        let mut chunk = self.get_chunk(coord.0, coord.1);
        let chunk_coord = self.world_to_chunk(pt);

        chunk.set(&chunk_coord, entity);
    }

    pub fn move_worker(&mut self, entity: u32, old_pt: WorldPos, new_pt: WorldPos) {
        self.worker_map.remove(&old_pt);
        self.track_worker(entity, new_pt);
    }

    pub fn remove_object(&mut self, _entity: u32, pt: WorldPos) {
        self.object_map.remove(&pt);
    }

    pub fn track_object(&mut self, entity: u32, pt: WorldPos) {
        self.object_map.insert(pt, entity);
    }

    pub fn track_worker(&mut self, entity: u32, pt: WorldPos) {
        self.worker_map.insert(pt, entity);
    }
}
