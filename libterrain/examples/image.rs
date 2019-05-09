/// Outputs the result of the terrain generation into a PPM file.
use std::fs::File;
use std::io::prelude::Write;

use libterrain::{Biome, TerrainGenerator};

fn main() -> Result<(), std::io::Error> {
    let width = 512;
    let height = 512;

    let tergen = TerrainGenerator::new(width, height).build();

    let mut outfile = File::create("terrain.ppm").expect("Could not write to file");
    // Write PPM header
    writeln!(outfile, "P3")?;
    writeln!(outfile, "{} {}", width, height)?;
    writeln!(outfile, "255")?;

    for y in 0..height {
        for x in 0..width {
            let mut color = match tergen.get_biome(x as usize, y as usize) {
                Biome::OCEAN => (66, 66, 125),
                Biome::BEACH => (211, 185, 135),
                Biome::GRASSLAND => (135, 171, 75),
                Biome::TAIGA => (136, 153, 116),
                Biome::TUNDRA => (128, 128, 128),
                Biome::SNOW => (255, 255, 255),
            };

            if tergen.has_tree(x as usize, y as usize) {
                color = (0, 0, 0);
            }

            write!(outfile, "{} {} {} ", color.0, color.1, color.2)?;
        }

        writeln!(outfile)?;
    }

    Ok(())
}
