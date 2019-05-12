use noise::{NoiseFn, Perlin};
use std::collections::HashMap;

mod poisson;
pub use nalgebra::Point3;
use poisson::PoissonDisk;

#[derive(Clone)]
pub struct TerrainGenerator {
    width: usize,
    height: usize,
    terrain: Vec<f64>,
    objects: HashMap<Point3<u32>, Object>,
}

#[derive(Clone, Debug)]
pub enum Biome {
    OCEAN,
    BEACH,
    GRASSLAND,
    TAIGA,
    TUNDRA,
    SNOW,
}

#[derive(Clone, Debug)]
pub enum Object {
    TREE,
}

impl TerrainGenerator {
    pub fn new(width: u32, height: u32) -> Self {
        TerrainGenerator {
            width: width as usize,
            height: height as usize,
            terrain: vec![0.0; (width * height) as usize],
            objects: HashMap::new(),
        }
    }

    fn idx(&self, x: u32, y: u32) -> usize {
        (y * self.width as u32 + x) as usize
    }

    pub fn build(mut self) -> Self {
        let noise = Perlin::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let nx = x as f64 / self.width as f64 - 0.5;
                let ny = y as f64 / self.height as f64 - 0.5;
                let idx = self.idx(x as u32, y as u32);
                // Generate noise value & normalize to be between [0, 1]
                self.terrain[idx] = ((1.0
                    + noise.get([nx, ny])
                    + 0.50 * noise.get([2.0 * nx, 2.0 * ny])
                    + 0.25 * noise.get([4.0 * nx, 2.0 * ny]))
                    / 2.0)
                    // Clamp value at 1.0
                    .min(1.0);

                self.terrain[idx].powf(5.29);
            }
        }

        // Generate tree distribution.
        let mut poisson = PoissonDisk::new(self.width, self.height, 5);
        poisson.generate(5);

        for pt in poisson.samples.iter() {
            self.objects.insert(*pt, Object::TREE);
        }

        self
    }

    /// Returns the randomly generated value @ (x, y)
    pub fn get_value(&self, x: u32, y: u32) -> f64 {
        let idx = self.idx(x, y);
        self.terrain[idx]
    }

    /// Returns the Biome at (x, y).
    pub fn get_biome(&self, x: u32, y: u32) -> Biome {
        let value = self.get_value(x, y);
        // Ocean biome
        if value < 0.2 {
            return Biome::OCEAN;
        // beach biome
        } else if value < 0.3 {
            return Biome::BEACH;
        // grassland
        } else if value < 0.6 {
            return Biome::GRASSLAND;
        // taiga
        } else if value <= 0.9 {
            return Biome::TAIGA;
        // tundra
        } else if value > 0.9 && value < 0.95 {
            return Biome::TUNDRA;
        // snow
        } else if value > 0.95 {
            return Biome::SNOW;
        }

        Biome::GRASSLAND
    }

    pub fn objects(&self) -> HashMap<Point3<u32>, Object> {
        self.objects.clone()
    }

    pub fn has_tree(&self, x: usize, y: usize) -> bool {
        let object = self.objects.get(&Point3::new(x as u32, y as u32, 0));
        object.is_some()
    }
}
