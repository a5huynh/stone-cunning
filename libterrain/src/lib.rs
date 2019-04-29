use noise::{NoiseFn, Perlin};

#[derive(Clone)]
pub struct TerrainGenerator {
    width: usize,
    height: usize,
    terrain: Vec<f64>,
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

impl TerrainGenerator {
    pub fn new(width: u32, height: u32) -> Self {
        TerrainGenerator {
            width: width as usize,
            height: height as usize,
            terrain: vec![0.0; (width * height) as usize],
        }
    }

    fn idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn build(mut self) -> Self {
        let noise = Perlin::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let nx = x as f64 / self.width as f64 - 0.5;
                let ny = y as f64 / self.height as f64 - 0.5;
                let idx = self.idx(x, y);
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

        self
    }

    pub fn get(&self, x: usize, y: usize) -> f64 {
        let idx = self.idx(x, y);
        self.terrain[idx]
    }

    pub fn get_biome(&self, x: usize, y: usize) -> Biome {
        let value = self.get(x, y);
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
}
