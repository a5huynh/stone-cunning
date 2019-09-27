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

    /// Is this point reachable?
    /// A space is passable if there is no block there and a block below.
    pub fn is_walkable(&self, point: &Point3<u32>) -> bool {
        if point.z == 0 {
            return self.get(point.x, point.y, point.z).is_none();
        }

        self.get(point.x, point.y, point.z).is_none()
            && self.get(point.x, point.y, point.z - 1).is_some()
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

    fn neighbors_for_level(&self, neighbors: &mut Vec<Point3<u32>>, pt: &Point3<u32>, zlevel: u32) {
        let (x, y) = (pt.x, pt.y);

        if y > 0 {
            neighbors.push(Point3::new(x, y - 1, zlevel));
        }

        if x > 0 {
            neighbors.push(Point3::new(x - 1, y, zlevel));
        }

        if x < self.width {
            neighbors.push(Point3::new(x + 1, y, zlevel));
        }

        if y < self.height {
            neighbors.push(Point3::new(x, y + 1, zlevel));
        }
    }

    /// Return the list of neighboring points for <pt>.
    pub fn neighbors(&self, pt: &Point3<u32>) -> Vec<Point3<u32>> {
        let mut results = Vec::new();

        if pt.z > 0 {
            self.neighbors_for_level(&mut results, pt, pt.z - 1);
        }

        self.neighbors_for_level(&mut results, pt, pt.z);

        if pt.z < ZLEVELS {
            self.neighbors_for_level(&mut results, pt, pt.z + 1);
        }

        results
            .into_iter()
            .filter(|pt| self.is_in_bounds(pt))
            .filter(|pt| self.is_walkable(pt))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{Biome, Point3, TerrainChunk};

    #[test]
    fn test_basic_neighbors() {
        let chunk = TerrainChunk::new(3, 3);
        let neighbors = chunk.neighbors(&Point3::new(1, 1, 0));

        // Since there are no blocks in this test chunk, we should only have 4
        // walkable neighbors at zlevel = 0. Everything else would be considered air.
        assert_eq!(neighbors.len(), 4);
    }

    #[test]
    fn test_blocked_neighbors() {
        let mut chunk = TerrainChunk::new(3, 3);

        // Test a single block high wall, should be passable.
        chunk.set((0, 1, 0), Some(Biome::ROCK));
        chunk.set((1, 1, 0), Some(Biome::ROCK));
        chunk.set((2, 1, 0), Some(Biome::ROCK));
        let neighbors = chunk.neighbors(&Point3::new(1, 0, 0));
        assert_eq!(neighbors[0], Point3::new(0, 0, 0));
        assert_eq!(neighbors[1], Point3::new(2, 0, 0));
        // On top of the wall, right in front.
        assert_eq!(neighbors[2], Point3::new(1, 1, 1));

        // Turning the wall two blocks high should be unpassable.
        chunk.set((0, 1, 1), Some(Biome::ROCK));
        chunk.set((1, 1, 1), Some(Biome::ROCK));
        chunk.set((2, 1, 1), Some(Biome::ROCK));

        // Since there is a wall in the way, we should only get these two points
        let neighbors = chunk.neighbors(&Point3::new(1, 0, 0));
        assert_eq!(neighbors[0], Point3::new(0, 0, 0));
        assert_eq!(neighbors[1], Point3::new(2, 0, 0));
    }
}
