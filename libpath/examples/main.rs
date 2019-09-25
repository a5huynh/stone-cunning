use libpath::dijkstra;
use libterrain::{Biome, Point3, TerrainChunk};

const TEST_WIDTH: u32 = 30;
const TEST_HEIGHT: u32 = 15;
const TEST_DEPTH: u32 = 3;

pub fn biome_to_ascii(biome: Option<Biome>) -> char {
    match biome {
        Some(Biome::ROCK) => '\'',
        Some(Biome::GRASSLAND) => ',',
        _ => '.',
    }
}

pub fn init_terrain(terrain: &mut TerrainChunk) {
    // Fill the bottom level w/ rock
    for y in 0..TEST_HEIGHT {
        for x in 0..TEST_WIDTH {
            terrain.set((x, y, 0), Some(Biome::ROCK));
        }
    }

    // Add a 1 block high wall in the 2nd level.
    for y in 0..TEST_HEIGHT {
        terrain.set((15, y, 1), Some(Biome::ROCK));
    }
}

pub fn main() {
    // Create fake output from libterrain
    let mut terrain = TerrainChunk::new(TEST_WIDTH, TEST_HEIGHT);
    init_terrain(&mut terrain);

    let (parents, path) = dijkstra(&terrain, Point3::new(8, 7, 1), Point3::new(17, 2, 1));
    // Draw parents
    for z in (0..TEST_DEPTH).rev() {
        println!("z-level: {}", z);

        for y in 0..TEST_HEIGHT {
            for x in 0..TEST_WIDTH {
                let pt = Point3::new(x, y, z);
                if path.contains(&pt) {
                    print!("@");
                } else {
                    if parents.contains_key(&pt) {
                        let came_from = parents[&pt];
                        print!("{}", direction((x, y, z), &came_from));
                    } else {
                        print!("{}", biome_to_ascii(terrain.get(pt.x, pt.y, pt.z)));
                    }
                }
                print!(" ");
            }

            print!("\n");
        }
    }
}

pub fn direction(pt: (u32, u32, u32), start: &Point3<u32>) -> String {
    let xdir = pt.0 as i32 - start.x as i32;
    let ydir = pt.1 as i32 - start.y as i32;
    let zdir = pt.2 as i32 - start.z as i32;

    if ydir == 0 && xdir == 0 && zdir == 0 {
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

    // zlevels
    if zdir > 0 || zdir < 0 {
        return String::from("X");
    }

    return String::from("V");
}
