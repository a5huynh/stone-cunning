use rand::{self, Rng};
use std::f64::consts::PI;

pub struct PoissonDisk {
    min_dist: usize,
    width: usize,
    height: usize,
    grid_size: f64,
    grid_width: f64,
    grid_height: f64,

    grid: Vec<Option<(usize, usize)>>,
    active: Vec<(usize, usize)>,
    pub samples: Vec<(usize, usize)>,
}

fn distance(a: (usize, usize), b: (usize, usize)) -> f64 {
    let dx = a.0 as f64 - b.0 as f64;
    let dy = a.1 as f64 - b.1 as f64;

    (dx * dx + dy * dy).sqrt()
}

impl PoissonDisk {
    pub fn new(width: usize, height: usize, min_dist: usize) -> Self {
        // dist / square root of the number of dimensions.
        let grid_size = min_dist as f64 / (2.0 as f64).sqrt();
        let grid_width = (width as f64 / grid_size).ceil() + 1.0;
        let grid_height = (height as f64 / grid_size).ceil() + 1.0;
        let grid = vec![None; (grid_width * grid_height) as usize];

        let mut disk = PoissonDisk {
            min_dist,
            width,
            height,
            grid_size,
            grid_height,
            grid_width,
            grid,
            active: Vec::new(),
            samples: Vec::new(),
        };

        // Generates a random point
        let mut rng = rand::thread_rng();
        let point = (
            rng.gen_range(0, width - 1) as usize,
            rng.gen_range(0, height - 1) as usize,
        );
        disk.insert_point(point);
        disk.active.push(point);

        disk
    }

    fn is_valid(&self, point: (usize, usize)) -> bool {
        let xidx = (point.0 as f64 / self.grid_size).floor();
        let yidx = (point.1 as f64 / self.grid_size).floor();

        // Get the neighborhood of the point in the grid.
        let start_x = (xidx - 2.0).max(0.0) as usize;
        let end_x = (xidx + 2.0).min(self.grid_width - 1.0) as usize;
        let start_y = (yidx - 2.0).max(0.0) as usize;
        let end_y = (yidx + 2.0).min(self.grid_height - 1.0) as usize;

        for y in start_y..end_y {
            for x in start_x..end_x {
                let idx = y * self.grid_width as usize + x;
                if let Some(cell) = self.grid[idx] {
                    if distance(cell, point) <= self.min_dist as f64 {
                        return false;
                    }
                }
            }
        }

        true
    }

    fn insert_point(&mut self, point: (usize, usize)) {
        let cell_x = (point.0 as f64 / self.grid_size).floor();
        let cell_y = (point.1 as f64 / self.grid_size).floor();

        self.samples.push(point);

        let cell_idx = (cell_y * self.grid_width + cell_x) as usize;
        self.grid[cell_idx] = Some(point);
    }

    fn generate_around(&mut self, point: (usize, usize)) -> (usize, usize) {
        let mut rng = rand::thread_rng();
        // Random angle
        let angle = 2.0 * PI * rng.gen::<f64>();
        // Random radius between min_dist and 2 * min_dist
        let radius = self.min_dist as f64 * (rng.gen::<f64>() + 1.0);
        // The new point is generated around the point (x, y)
        let new_x = point.0 as f64 + (radius * angle.cos());
        let new_y = point.1 as f64 + (radius * angle.sin());

        (
            new_x.max(0.0).min(self.width as f64 - 1.0) as usize,
            new_y.max(0.0).min(self.height as f64 - 1.0) as usize,
        )
    }

    pub fn generate(&mut self, new_points_count: usize) {
        let mut rng = rand::thread_rng();
        // Generate other points from points in queue
        println!("generating points");
        while !self.active.is_empty() {
            let idx = (rng.gen::<f64>() * (self.active.len() - 1) as f64) as usize;
            let point = self.active[idx];

            let mut found = false;
            for _ in 0..new_points_count {
                let new_point = self.generate_around(point);
                // Add the new point to the grid and active list if the point is
                // valid.
                if self.is_valid(new_point) {
                    self.insert_point(new_point);
                    self.active.push(new_point);
                    found = true;
                }
            }

            if !found {
                self.active.remove(idx);
            }
        }

        println!("finished generating points: ({})", self.samples.len());
    }
}