use libpath::{dijkstra};
use libterrain::{Biome, Point3, TerrainChunk};

const TEST_WIDTH: u32 = 30;
const TEST_HEIGHT: u32 = 15;
const TEST_DEPTH: u32 = 2;

pub fn init_terrain(terrain: &mut TerrainChunk) {
    terrain.set((21, 0, 0), Some(Biome::ROCK));
    terrain.set((21, 1, 0), Some(Biome::ROCK));
    terrain.set((21, 2, 0), Some(Biome::ROCK));
    terrain.set((21, 3, 0), Some(Biome::ROCK));
    terrain.set((21, 4, 0), Some(Biome::ROCK));
    terrain.set((21, 5, 0), Some(Biome::ROCK));
    terrain.set((22, 5, 0), Some(Biome::ROCK));
    terrain.set((23, 5, 0), Some(Biome::ROCK));
    terrain.set((24, 5, 0), Some(Biome::ROCK));
    terrain.set((25, 5, 0), Some(Biome::ROCK));
    terrain.set((26, 5, 0), Some(Biome::ROCK));
    terrain.set((27, 5, 0), Some(Biome::ROCK));
}

pub fn main() {
    // Create fake output from libterrain
    let mut terrain = TerrainChunk::new(TEST_WIDTH, TEST_HEIGHT);
    init_terrain(&mut terrain);

    let (parents, path) = dijkstra(&terrain, Point3::new(8, 7, 0), Point3::new(17, 2, 0));
    // Draw parents
    for z in (0..TEST_DEPTH).rev() {
        println!("z-level: {}", z);

        for y in 0..TEST_HEIGHT {
            for x in 0..TEST_WIDTH {
                let pt = Point3::new(x, y, z);
                if !terrain.is_passable(&pt) {
                    print!("#");
                } else {
                    if path.contains(&pt) {
                        print!("@");
                    } else {
                        if parents.contains_key(&pt) {
                            let came_from = parents[&pt];
                            print!("{}", direction((x, y), &came_from));
                        } else {
                            print!(".");
                        }
                    }
                }
                print!(" ");
            }

            print!("\n");
        }
    }
}

pub fn direction(pt: (u32, u32), start: &Point3<u32>) -> String {
    let xdir = pt.0 as i32 - start.x as i32;
    let ydir = pt.1 as i32 - start.y as i32;

    if ydir == 0 && xdir == 0 {
        return String::from("A");
    }

    // Up / down
    if xdir > 0 {
        return String::from("<");
    } else if xdir < 0 {
        return String::from(">");
    }

    // Left /right
    if ydir > 0 {
        return String::from("^");
    }

    return String::from("V");
}
