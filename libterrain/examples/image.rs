/// Outputs the result of the terrain generation into a PPM file.
use std::fs::File;
use std::io::prelude::Write;

use libterrain::{Biome, TerrainLoader};

fn main() -> Result<(), std::io::Error> {
    let num_x_chunks = 5;
    let num_y_chunks = 5;

    let chunk_width = 64;
    let chunk_height = 64;

    let width = num_x_chunks * chunk_width;
    let height = num_y_chunks * chunk_height;

    let terloader = TerrainLoader::new(chunk_width, chunk_height);
    let mut colors = vec![(0, 0, 0); (width * height) as usize];

    let mut outfile = File::create("terrain.ppm").expect("Could not write to file");

    // Write PPM header
    writeln!(outfile, "P3")?;
    writeln!(outfile, "{} {}", width, height)?;
    writeln!(outfile, "255")?;

    for my in 0..num_y_chunks {
        for mx in 0..num_x_chunks {
            let chunk = terloader.get_topo(mx as i32, my as i32);

            for y in 0..chunk_height {
                for x in 0..chunk_width {
                    let color = match chunk[(y * chunk_width + x) as usize] {
                        Some(Biome::OCEAN) => (66, 66, 125),
                        Some(Biome::BEACH) => (211, 185, 135),
                        Some(Biome::GRASSLAND) => (135, 171, 75),
                        Some(Biome::TAIGA) => (136, 153, 116),
                        Some(Biome::TUNDRA) => (128, 128, 128),
                        Some(Biome::SNOW) => (255, 255, 255),
                        _ => (0, 0, 0),
                    };

                    // if tergen.has_tree(x as usize, y as usize) {
                    //     color = (0, 0, 0);
                    // }

                    let x_off = mx as u32 * chunk_width;
                    let y_off = my as u32 * chunk_height;
                    let idx = (y + y_off) * width as u32 + (x + x_off);
                    colors[idx as usize] = color;
                }
            }
        }
    }

    // Write out to file
    for y in 0..height {
        for x in 0..width {
            let color = colors[(y * width + x) as usize];
            write!(outfile, "{} {} {} ", color.0, color.1, color.2)?;
        }
        writeln!(outfile)?;
    }

    Ok(())
}
