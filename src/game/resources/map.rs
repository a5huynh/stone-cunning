// TODO: dynamic rendering as a system
use std::f32::consts::{FRAC_PI_2, PI};

use core::amethyst::{
    core::{math::Point3, transform::Transform},
    prelude::*,
};

use crate::game::{components::Direction, config::GameConfig};

/// Map resource used to convert screen coordinates into map coordinates, check for
/// collisions amongst objects, represent the current terrain.
pub struct MapRenderer {
    /// rotation - which direction along the map the camera is looking.
    pub rotation: Direction,
    pub rotate_cooldown: bool,

    pub tile_width: f32,
    pub tile_height: f32,
}

impl MapRenderer {
    pub fn initialize(world: &mut World) -> Self {
        let (tile_height, tile_width) = {
            let config = &world.read_resource::<GameConfig>();
            (config.tile_height, config.tile_width)
        };

        MapRenderer {
            rotation: Direction::NORTH,
            rotate_cooldown: false,
            tile_height: tile_height as f32,
            tile_width: tile_width as f32,
        }
    }

    pub fn rotate_left(&mut self) {
        let new_rotation = match self.rotation {
            Direction::NORTH => Direction::EAST,
            Direction::EAST => Direction::SOUTH,
            Direction::SOUTH => Direction::WEST,
            Direction::WEST => Direction::NORTH,
        };

        self.rotate_cooldown = true;
        self.rotation = new_rotation;
    }

    pub fn rotate_right(&mut self) {
        let new_rotation = match self.rotation {
            Direction::NORTH => Direction::WEST,
            Direction::WEST => Direction::SOUTH,
            Direction::SOUTH => Direction::EAST,
            Direction::EAST => Direction::NORTH,
        };

        self.rotate_cooldown = true;
        self.rotation = new_rotation;
    }

    /// Converts some point <x, y> into map coordinates.
    pub fn to_map_coords(&self, x: f32, y: f32) -> (i32, i32) {
        // Convert position to cartesian coordinates
        let tw = self.tile_width;
        let th = self.tile_height;

        let px = (x / tw) + (y / th);
        let py = (y / th) - (x / tw);

        // Determine how we should rotate the coordinates.
        // This should be the opposite of what we use to place
        // the points.
        let rotation: f32 = match self.rotation {
            Direction::NORTH => 0.0,
            Direction::EAST => -FRAC_PI_2,
            Direction::SOUTH => PI,
            Direction::WEST => FRAC_PI_2,
        };

        let cos_rot = rotation.cos();
        let sin_rot = rotation.sin();

        let mx = ((px - 32.0) * cos_rot) - ((py - 32.0) * sin_rot) + 32.0;
        let my = ((px - 32.0) * sin_rot) + ((py - 32.0) * cos_rot) + 32.0;

        (mx.trunc() as i32, my.trunc() as i32)
    }

    /// Creates a transform that would place an object on the map using
    /// map coordinates at <x, y, z> w/ zindex.
    ///
    /// Given a Direction, uses a rotation matrix to rotate the coordinates.
    ///
    /// The zoffset is a float, to allow for multiple objects coexisting
    /// on a single tile in a certain order.
    pub fn place(&self, pt: &Point3<i32>, zoffset: f32) -> Transform {
        let mut transform = Transform::default();

        let fx = pt.x as f32;
        let fy = pt.y as f32;
        let fz = pt.z as f32;

        // Determine how we should rotate the coordinates
        let rotation: f32 = match self.rotation {
            Direction::NORTH => 0.0,
            Direction::EAST => FRAC_PI_2,
            Direction::SOUTH => PI,
            Direction::WEST => -FRAC_PI_2,
        };

        let cos_rot = rotation.cos();
        let sin_rot = rotation.sin();

        // Center in window, rotate around new origin, and then translate back.
        // TODO: keep track of window
        let px = ((fx - 32.0) * cos_rot) - ((fy - 32.0) * sin_rot) + 32.0;
        let py = ((fx - 32.0) * sin_rot) + ((fy - 32.0) * cos_rot) + 32.0;

        // Scale and convert to iso coordinates
        let tx = (px - py) * self.tile_width / 2.0;
        let ty = (px + py) * self.tile_height / 2.0 + (fz * self.tile_height);
        let tz = -(px + py) + zoffset;

        // Set translation
        transform.set_translation_xyz(tx, ty, tz);
        transform
    }
}
