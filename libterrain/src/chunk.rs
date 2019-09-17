use super::Point3;

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

#[derive(Clone)]
pub struct TerrainChunk {
    grid: Vec<Option<Biome>>,
    height: u32,
    width: u32,
}

const ZLEVELS: u32 = 64;

impl TerrainChunk {
    pub fn new(width: u32, height: u32) -> TerrainChunk {
        TerrainChunk {
            width,
            height,
            grid: vec![None; (width * height * ZLEVELS) as usize],
        }
    }

    fn idx(&self, x: u32, y: u32, z: u32) -> usize {
        (z * (self.width * self.height) as u32 + y * self.width as u32 + x) as usize
    }

    /// Is <point> within the bounds of this chunk?
    fn is_in_bounds(&self, point: &Point3<u32>) -> bool {
        point.x < self.width && point.y < self.height && point.z < ZLEVELS
    }

    // Is this point reachable?
    pub fn is_passable(&self, point: &Point3<u32>) -> bool {
        return true;
    }

    /// Get chunk data at a specific position
    pub fn get(&self, x: u32, y: u32, z: u32) -> Option<Biome> {
        self.grid[self.idx(x, y, z)].clone()
    }

    /// Set chunk data at a specific position
    pub fn set(&mut self, pt: (u32, u32, u32), biome: Option<Biome>) {
        let idx = self.idx(pt.0, pt.1, pt.2);
        self.grid[idx] = biome;
    }

    /// Return the list of neighboring points for <pt>.
    pub fn neighbors(&self, pt: &Point3<u32>) -> Vec<Point3<u32>> {
        let (x, y, z) = (pt.x, pt.y, pt.z);
        let mut results = Vec::new();

        if y > 0 {
            results.push(Point3::new(x, y - 1, z));
        }

        if x > 0 {
            results.push(Point3::new(x - 1, y, z));
        }

        if z > 0 {
            results.push(Point3::new(x, y, z - 1));
        }

        if x < self.width {
            results.push(Point3::new(x + 1, y, z));
        }

        if y < self.height {
            results.push(Point3::new(x, y + 1, z));
        }

        if z < ZLEVELS {
            results.push(Point3::new(x, y, z + 1));
        }

        results
            .into_iter()
            .filter(|pt| self.is_in_bounds(pt))
            .filter(|pt| self.is_passable(pt))
            .collect()
    }
}