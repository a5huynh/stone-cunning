use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug)]
pub enum Terrain {
    STONE = 0,
    MARBLE = 1,
    GRASS = 2,
    NONE = -1,
}

#[derive(Clone, Debug)]
pub struct MapObject {
    pub id: u32
}

#[derive(Clone)]
pub struct World {
    // TODO: Support multiple objects per tile.
    // TODO: Support multi-tile objects.
    pub width: u32,
    pub height: u32,
    pub objects: HashMap<(i32, i32), MapObject>,
    pub terrain: HashMap<(i32, i32), Terrain>,
}

impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let display_y = self.height - y - 1;
                let idx = (x as i32, display_y as i32);
                let terrain = self.terrain.get(&idx);
                match terrain {
                    Some(Terrain::GRASS) => write!(f, "G")?,
                    Some(Terrain::STONE) => write!(f, "S")?,
                    Some(Terrain::MARBLE) => write!(f, "M")?,
                    _ => write!(f, "?")?,
                };

                if x < self.width - 1 {
                    write!(f, ".")?;
                }
            }

            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl World {
    pub fn new(height: u32, width: u32) -> Self {
        let mut map_terrain = HashMap::new();
        // TODO: Actually generate terrain.
        for y in 0..height {
            for x in 0..width {
                let tile = ((x + y) % 3) as usize;
                let terrain = match tile {
                    0 => Terrain::STONE,
                    1 => Terrain::MARBLE,
                    2 => Terrain::GRASS,
                    _ => Terrain::NONE,
                };

                map_terrain.insert((x as i32, y as i32), terrain);
            }
        }

        World {
            height,
            width,
            objects: HashMap::new(),
            terrain: map_terrain
        }
    }

    pub fn add_object(&mut self, x: i32, y: i32, object: MapObject) {
        self.objects.insert((x, y), object);
    }
}