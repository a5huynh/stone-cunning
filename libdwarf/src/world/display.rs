/// Mostly used for debugging outside of a rendering context.
use std::fmt;

impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut cells = vec!['?'; (self.width * self.height) as usize];
        // Render terrain first
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                let idx = (x as u32, y as u32);
                let terrain = self.terrain.get(&idx);
                let tile = match terrain {
                    Some(Terrain::GRASS) => ',',
                    Some(Terrain::STONE) => '.',
                    Some(Terrain::MARBLE) => '.',
                    _ => '?',
                };

                cells[(y * self.width + x) as usize] = tile;
            }
        }

        // Add objects to cells
        for (_, object) in self.objects.iter() {
            let idx = (object.y * self.width + object.x) as usize;
            let tile = match object {
                MapObject { id: 1, .. } => 'T',
                MapObject { id: 10, .. } => 'w',
                _ => '?'
            };

            cells[idx] = tile;
        }

        // Add workers to cells
        for worker in self.workers.iter() {
            let idx = (worker.y * self.width + worker.x) as usize;
            cells[idx] = 'w';
        }

        // Output completed cells.
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                write!(f, "{}", cells[(y * self.width + x) as usize])?;
                if x < self.width - 1 {
                    write!(f, " ")?;
                }
            }
            write!(f, "\n\r")?;
        }

        Ok(())
    }
}