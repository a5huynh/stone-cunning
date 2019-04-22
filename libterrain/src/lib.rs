use noise::{NoiseFn, Perlin};
pub struct TerrainGenerator {
    width: usize,
    height: usize,
    terrain: Vec<f64>,
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

    pub fn get_biome(&self, x: usize, y: usize) -> (usize, usize, usize) {
        let value = self.get(x, y);
        let mut red = 0;
        let mut green = (value * 255.0) as usize;
        let mut blue = 0;

        if value < 0.2 {
            red = 0;
            green = 0;
            blue = 255;
        } else if value < 0.25 {
            red = 231;
            green = 201;
            blue = 175;
        } else if value > 0.9 && value < 0.95 {
            red = 128;
            green = 128;
            blue = 128;
        } else if value > 0.95 {
            red = 255;
            green = 255;
            blue = 255;
        }

        (red, green, blue)
    }
}
