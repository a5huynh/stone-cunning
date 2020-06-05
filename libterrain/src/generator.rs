use core::Uuid;
use noise::{NoiseFn, Perlin};
use std::cmp::Ordering;

use crate::chunk::{Biome, ChunkEntity, ChunkPos, ObjectType, TerrainChunk};
use crate::poisson::PoissonDisk;

#[derive(Clone)]
pub struct TerrainGenerator {
    // Chunk coordinate to generate.
    chunk_x: i32,
    chunk_y: i32,
    // Width & height of terrain chunk to generate.
    width: usize,
    height: usize,
    // TODO: Support multiple chunks.
    terrain: TerrainChunk,
    pub topo: Vec<Option<Biome>>,
}

// TODO: Make this a variable?
const ZLEVELS: u32 = 64;
const GROUND_HEIGHT: u32 = 32;
const WATER_HEIGHT: u32 = GROUND_HEIGHT + (0.2 * GROUND_HEIGHT as f64) as u32;

// NOTE:
// To current chunk, we need to have a constant chunk size used as the width / height for each
// chunk. Then as we move around in x, y
impl TerrainGenerator {
    pub fn new(width: u32, height: u32) -> Self {
        TerrainGenerator {
            chunk_x: 0,
            chunk_y: 0,
            width: width as usize,
            height: height as usize,
            terrain: TerrainChunk::new(width, height),
            topo: vec![None; (width * height) as usize],
        }
    }

    pub fn chunk_coord(mut self, x: i32, y: i32) -> Self {
        self.chunk_x = x;
        self.chunk_y = y;

        self
    }

    pub fn build(mut self) -> Self {
        let noise = Perlin::new();
        // Keep track of elevation for object placement.
        let mut heightmap = vec![None; self.height * self.width];

        let x_offset = self.chunk_x * self.width as i32;
        let y_offset = self.chunk_y * self.height as i32;

        for y in 0..self.height {
            for x in 0..self.width {
                let nx = (x as i32 + x_offset) as f64 / self.width as f64 - 0.5;
                let ny = (y as i32 + y_offset) as f64 / self.height as f64 - 0.5;
                // Generate noise value & normalize to be between [0, 1]
                let mut elevation = ((1.0
                    + noise.get([nx, ny])
                    + 0.50 * noise.get([2.0 * nx, 2.0 * ny])
                    + 0.25 * noise.get([4.0 * nx, 2.0 * ny]))
                    / 2.0)
                    // Clamp value at 1.0
                    .min(1.0);
                // Smooth things out. By raising the elevation values to a power,
                // we can make flat valleys.
                elevation = elevation.powf(1.00);

                // Fill in this chunk based on the elevation
                // Ground level is always at 32.
                let biome = self.determine_biome(elevation);
                let terrain_height =
                    (GROUND_HEIGHT as f64 + (f64::from(GROUND_HEIGHT) * elevation).floor()) as u32;

                self.topo[y * self.width + x] = Some(biome.clone());
                heightmap[y * self.width + x] = Some((terrain_height, biome.clone()));

                // TODO:
                //  * Less hilly?
                //  * place trees correctly on 3d map
                for z in 0..ZLEVELS {
                    let idx = ChunkPos::new(x as u32, y as u32, z as u32);
                    match biome {
                        // For water biomes, the height is always the same, but the
                        // depth of the water will change.
                        Biome::OCEAN => {
                            if z >= terrain_height && z <= WATER_HEIGHT {
                                self.terrain.set(&idx, Some(ChunkEntity::Terrain(biome.clone())));
                            } else if z < terrain_height {
                                self.terrain.set(&idx, Some(ChunkEntity::Terrain(Biome::ROCK)));
                            }
                        }
                        _ => match z.cmp(&terrain_height) {
                            Ordering::Equal => {
                                self.terrain.set(&idx, Some(ChunkEntity::Terrain(biome.clone())))
                            }
                            Ordering::Less => {
                                self.terrain.set(&idx, Some(ChunkEntity::Terrain(Biome::ROCK)))
                            }
                            _ => {}
                        },
                    }
                }
            }
        }

        // Generate tree distribution.
        let mut poisson = PoissonDisk::new(self.width, self.height, 5);
        poisson.generate(5);

        for pt in &mut poisson.samples {
            // Get the terrain height at this location
            let idx = pt.y * self.width as u32 + pt.x;
            if let Some(data) = &heightmap[idx as usize] {
                if data.1 != Biome::OCEAN && data.0 < (ZLEVELS - 1) {
                    pt.z = data.0 + 1;
                    self.terrain.set(pt, Some(ChunkEntity::Object(Uuid::new_v4(), ObjectType::TREE)));
                }
            }
        }

        self
    }

    pub fn determine_biome(&self, elevation: f64) -> Biome {
        // Ocean biome
        if elevation < 0.2 {
            return Biome::OCEAN;
        // beach biome
        } else if elevation < 0.3 {
            return Biome::BEACH;
        // grassland
        } else if elevation < 0.6 {
            return Biome::GRASSLAND;
        // taiga
        } else if elevation <= 0.9 {
            return Biome::TAIGA;
        // tundra
        } else if elevation > 0.9 && elevation < 0.95 {
            return Biome::TUNDRA;
        // snow
        } else if elevation > 0.95 {
            return Biome::SNOW;
        }

        Biome::GRASSLAND
    }

    pub fn get_terrain(&self) -> TerrainChunk {
        self.terrain.clone()
    }
}
