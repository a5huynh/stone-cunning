use std::f64::consts::PI;

use core::Point3;
use rand::{self, Rng};

pub struct PoissonDisk {
    min_dist: usize,
    width: usize,
    height: usize,
    grid_size: f64,
    grid_width: f64,
    grid_height: f64,

    grid: Vec<Option<Point3<i32>>>,
    active: Vec<Point3<i32>>,
    pub samples: Vec<Point3<i32>>,
}

fn distance(a: Point3<i32>, b: Point3<i32>) -> f64 {
    let dx = f64::from(a.x) - f64::from(b.x);
    let dy = f64::from(a.y) - f64::from(b.y);

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
        let point = Point3::new(
            rng.gen_range(0, width - 1) as i32,
            rng.gen_range(0, height - 1) as i32,
            0,
        );
        disk.insert_point(point);
        disk.active.push(point);

        disk
    }

    fn is_valid(&self, point: Point3<i32>) -> bool {
        let xidx = (f64::from(point.x) / self.grid_size).floor();
        let yidx = (f64::from(point.y) / self.grid_size).floor();

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

    fn insert_point(&mut self, point: Point3<i32>) {
        let cell_x = (f64::from(point.x) / self.grid_size).floor();
        let cell_y = (f64::from(point.y) / self.grid_size).floor();

        self.samples.push(point);

        let cell_idx = (cell_y * self.grid_width + cell_x) as usize;
        self.grid[cell_idx] = Some(point);
    }

    fn generate_around(&mut self, point: Point3<i32>) -> Point3<i32> {
        let mut rng = rand::thread_rng();
        // Random angle
        let angle = 2.0 * PI * rng.gen::<f64>();
        // Random radius between min_dist and 2 * min_dist
        let radius = self.min_dist as f64 * (rng.gen::<f64>() + 1.0);
        // The new point is generated around the point (x, y)
        let new_x = f64::from(point.x) + (radius * angle.cos());
        let new_y = f64::from(point.y) + (radius * angle.sin());

        Point3::new(
            new_x.max(0.0).min(self.width as f64 - 1.0) as i32,
            new_y.max(0.0).min(self.height as f64 - 1.0) as i32,
            0,
        )
    }

    pub fn generate(&mut self, new_points_count: usize) {
        let mut rng = rand::thread_rng();
        // Generate other points from points in queue
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
    }
}
