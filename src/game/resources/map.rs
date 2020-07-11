// TODO: dynamic rendering as a system
use std::f32::consts::{FRAC_PI_2, PI};

use core::amethyst::{
    core::{math::Point3, transform::Transform},
    prelude::*,
};
use core::WorldPos;

use crate::game::config::GameConfig;
use libdwarf::Direction;

/// Map resource used to convert screen coordinates into map coordinates, check for
/// collisions amongst objects, represent the current terrain.
pub struct MapResource {
    /// rotation - which direction along the map the camera is looking.
    pub rotation: Direction,
    pub prev_rotation: Direction,
    pub rotate_cooldown: bool,

    pub tile_width: f32,
    pub tile_height: f32,
}

impl MapResource {
    pub fn initialize(world: &mut World) -> Self {
        let (tile_height, tile_width) = {
            let config = &world.read_resource::<GameConfig>();
            (config.tile_height, config.tile_width)
        };

        MapResource {
            rotation: Direction::NORTH,
            prev_rotation: Direction::NORTH,
            rotate_cooldown: false,
            tile_height: tile_height as f32,
            tile_width: tile_width as f32,
        }
    }

    pub fn rotate_camera(&mut self, pt: &WorldPos) -> WorldPos {
        let t = 60;
        let dir = match self.prev_rotation {
            Direction::NORTH => WorldPos::new(-1, -1, 0),
            Direction::EAST => WorldPos::new(-1, 1, 0),
            Direction::SOUTH => WorldPos::new(1, 1, 0),
            Direction::WEST => WorldPos::new(1, -1, 0),
        };

        // Find true center
        let center = WorldPos::new(pt.x + t * dir.x, pt.y + t * dir.y, pt.z + t * dir.z);

        let dir = match self.rotation {
            Direction::NORTH => WorldPos::new(1, 1, 0),
            Direction::EAST => WorldPos::new(1, -1, 0),
            Direction::SOUTH => WorldPos::new(-1, -1, 0),
            Direction::WEST => WorldPos::new(-1, 1, 0),
        };
        // Extend out to new direction
        WorldPos::new(
            center.x + t * dir.x,
            center.y + t * dir.y,
            center.z + t * dir.z,
        )
    }

    pub fn rotate_cw(&mut self) {
        let new_rotation = match self.rotation {
            Direction::NORTH => Direction::WEST,
            Direction::WEST => Direction::SOUTH,
            Direction::SOUTH => Direction::EAST,
            Direction::EAST => Direction::NORTH,
        };

        self.rotate_cooldown = true;
        self.prev_rotation = self.rotation;
        self.rotation = new_rotation;
    }

    pub fn rotate_ccw(&mut self) {
        let new_rotation = match self.rotation {
            Direction::NORTH => Direction::EAST,
            Direction::EAST => Direction::SOUTH,
            Direction::SOUTH => Direction::WEST,
            Direction::WEST => Direction::NORTH,
        };

        self.rotate_cooldown = true;
        self.prev_rotation = self.rotation;
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

        let cx = 0.0; // self.center_offset.x as f32;
        let cy = 0.0; // self.center_offset.y as f32;
        let mx = cx + (px - cx) * cos_rot - (py - cy) * sin_rot;
        let my = cy + (px - cx) * sin_rot + (py - cy) * cos_rot;

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
        let cx = 0.0; // self.center_offset.x as f32;
        let cy = 0.0; // self.center_offset.y as f32;

        // Center in window, rotate around new origin, and then translate back.
        let px = cx + (fx - cx) * cos_rot - (fy - cy) * sin_rot;
        let py = cy + (fx - cx) * sin_rot + (fy - cy) * cos_rot;

        // Scale and convert to iso coordinates
        let tx = (px - py) * self.tile_width / 2.0;
        let ty = (px + py) * self.tile_height / 2.0 + (fz * self.tile_height);
        let tz = -(px + py) + zoffset;

        // Set translation
        transform.set_translation_xyz(tx, ty, tz);
        transform
    }
}

#[cfg(test)]
mod test {
    use crate::game::resources::MapResource;
    use core::WorldPos;
    use libdwarf::Direction;

    #[test]
    fn test_map_place() {
        let map = MapResource {
            rotation: Direction::NORTH,
            prev_rotation: Direction::NORTH,
            rotate_cooldown: false,
            tile_height: 16.0,
            tile_width: 32.0,
        };

        let pt_a = map.place(&WorldPos::new(50, 50, 0), 0.0);
        let pt_b = map.place(&WorldPos::new(5, 5, 45), 0.0);
        assert_eq!(pt_a.translation().xy(), pt_b.translation().xy());
    }

    #[test]
    fn test_map_rotate() {
        let mut map = MapResource {
            rotation: Direction::NORTH,
            prev_rotation: Direction::NORTH,
            rotate_cooldown: false,
            tile_height: 16.0,
            tile_width: 32.0,
        };

        map.rotate_cw();

        assert_eq!(map.prev_rotation, Direction::NORTH);
        assert_eq!(map.rotation, Direction::WEST);

        let pt_a = map.place(&WorldPos::new(5, 5, 45), 0.0);
        assert_eq!(pt_a.translation().x, 160.0);
        assert_eq!(pt_a.translation().y, 720.0);
        assert_eq!(pt_a.translation().z, 0.0);
    }

    #[test]
    fn test_rotate_camera() {
        let mut map = MapResource {
            rotation: Direction::NORTH,
            prev_rotation: Direction::NORTH,
            rotate_cooldown: false,
            tile_height: 16.0,
            tile_width: 32.0,
        };

        map.rotate_cw();
        let new_center = map.rotate_camera(&WorldPos::new(50, 50, 0));
        assert_eq!(new_center, WorldPos::new(0, 0, 0));
    }
}
