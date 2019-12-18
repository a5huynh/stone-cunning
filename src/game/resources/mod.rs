use crate::game::components::Direction;

mod map;
pub use map::*;

/// CameraWindow represents the current camera configuration.
///
/// rotation - which direction along the map the camera is looking.
pub struct CameraWindow {
    pub rotation: Direction,
    pub rotate_cooldown: bool,
}

impl CameraWindow {
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
}

impl Default for CameraWindow {
    fn default() -> CameraWindow {
        CameraWindow {
            rotation: Direction::NORTH,
            rotate_cooldown: false,
        }
    }
}
