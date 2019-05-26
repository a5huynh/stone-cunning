use noise::{NoiseFn, Perlin};
use std::collections::HashMap;

mod poisson;
pub use nalgebra::Point3;
use poisson::PoissonDisk;

#[derive(Clone)]
pub struct TerrainGenerator {
    width: usize,
    height: usize,
    terrain: Vec<Option<Biome>>,
    objects: HashMap<Point3<u32>, Object>,
}

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

#[derive(Clone, Debug)]
pub enum Object {
    TREE,
}

// TODO: Make this a variable?
const ZLEVELS: u32 = 64;
const GROUND_HEIGHT: u32 = 32;
const WATER_HEIGHT: u32 = GROUND_HEIGHT + (0.2 * GROUND_HEIGHT as f64) as u32;

impl TerrainGenerator {
    pub fn new(width: u32, height: u32) -> Self {
        TerrainGenerator {
            width: width as usize,
            height: height as usize,
            terrain: vec![None; (width * height * ZLEVELS) as usize],
            objects: HashMap::new(),
        }
    }

    fn idx(&self, x: u32, y: u32, z: u32) -> usize {
        (z * (self.width * self.height) as u32 + y * self.width as u32 + x) as usize
    }

    pub fn build(mut self) -> Self {
        let noise = Perlin::new();

        // Keep track of elevation for object placement.
        let mut heightmap = vec![None; self.height * self.width];
        for y in 0..self.height {
            for x in 0..self.width {
                let nx = x as f64 / self.width as f64 - 0.5;
                let ny = y as f64 / self.height as f64 - 0.5;
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
                    GROUND_HEIGHT + (GROUND_HEIGHT as f64 * elevation).floor() as u32;

                heightmap[y * self.width + x] = Some((terrain_height, biome.clone()));

                // TODO:
                //  * Less hilly?
                //  * place trees correctly on 3d map
                for z in 0..ZLEVELS {
                    let idx = self.idx(x as u32, y as u32, z as u32);
                    match biome {
                        // For water biomes, the height is always the same, but the
                        // depth of the water will change.
                        Biome::OCEAN => {
                            if z >= terrain_height && z <= WATER_HEIGHT {
                                self.terrain[idx] = Some(biome.clone());
                            } else if z < terrain_height {
                                self.terrain[idx] = Some(Biome::ROCK);
                            }
                        }
                        _ => {
                            if z == terrain_height {
                                self.terrain[idx] = Some(biome.clone());
                            } else if z < terrain_height {
                                self.terrain[idx] = Some(Biome::ROCK);
                            }
                        }
                    }
                }
            }
        }

        // Generate tree distribution.
        let mut poisson = PoissonDisk::new(self.width, self.height, 5);
        poisson.generate(5);

        for pt in poisson.samples.iter_mut() {
            // Get the terrain height at this location
            let idx = pt.y * self.width as u32 + pt.x;
            if let Some(data) = &heightmap[idx as usize] {
                if data.1 != Biome::OCEAN {
                    pt.z = data.0;
                    self.objects.insert(*pt, Object::TREE);
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

    /// Returns the Biome at (x, y).
    pub fn get_biome(&self, x: u32, y: u32, z: u32) -> Option<Biome> {
        let idx = self.idx(x, y, z);
        self.terrain[idx].clone()
    }

    /// Determines whether the block @ (x, y, z) is visible.
    pub fn is_visible(&self, x: u32, y: u32, z: u32) -> bool {
        // Top level is always exposed.
        if z == ZLEVELS - 1 {
            return true;
        }

        // If any side is exposed to air, the block is visible.
        let start_x = match x {
            0 => 0,
            _ => x - 1
        };

        let start_y = match y {
            0 => 0,
            _ => y - 1
        };

        let start_z = match z {
            0 => 0,
            _ => z - 1
        };

        let end_x = (x + 1).min(self.width as u32 - 1);
        let end_y = (y + 1).min(self.height as u32 - 1);
        let end_z = (z + 1).min(ZLEVELS - 1);

        for ix in start_x..=end_x {
            for iy in start_y..=end_y {
                for iz in start_z..=end_z {
                    let idx = self.idx(ix, iy, iz);
                    if self.terrain[idx].is_none() {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn objects(&self) -> HashMap<Point3<u32>, Object> {
        self.objects.clone()
    }

    pub fn has_tree(&self, x: usize, y: usize) -> bool {
        let object = self.objects.get(&Point3::new(x as u32, y as u32, 0));
        object.is_some()
    }
}
