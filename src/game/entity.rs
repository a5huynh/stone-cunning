use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{
        SpriteSheetHandle,
        SpriteRender,
        Transparent,
    }
};


#[derive(Default)]
/// Used to move the camera and to follow around other entities
pub struct CameraFollow;
impl Component for CameraFollow {
    type Storage = DenseVecStorage<Self>;
}


#[derive(Default)]
pub struct Floor;
impl Component for Floor {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone, Debug, PartialEq)]
pub enum Direction {
    IDLE = 0,
    WEST = 1,
    NORTHWEST = 2,
    NORTH = 3,
    NORTHEAST = 4,
    EAST = 5,
    SOUTHEAST = 6,
    SOUTH = 7,
    SOUTHWEST = 8,
}

impl Default for Direction {
    fn default() -> Self { Direction::IDLE }
}

#[derive(Default)]
pub struct Player {
    // Direction the player is facing radians
    pub direction: Direction,
    // Used to keep track of the player's animation frame.
    pub ticks: f32,
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

impl Player {
    pub fn initialize(world: &mut World, player_sprite: SpriteSheetHandle) {
        let mut player_transform = Transform::default();
        player_transform.set_xyz(0.0, 0.0, 0.0);

        let sprite_render = SpriteRender {
            sprite_sheet: player_sprite.clone(),
            sprite_number: 0,
        };

        world.create_entity()
            .with(sprite_render.clone())
            .with(Player::default())
            .with(player_transform)
            .with(Transparent)
            .build();
    }

    /// Determine which way the player is facing based on the last
    /// movement transform.
    pub fn calculate_direction(x: f64, y: f64) -> Direction {

        if x < 0.0 {
            if y > 0.0 {
                return Direction::NORTHWEST;
            } else if y < 0.0 {
                return Direction::SOUTHWEST
            }
            return Direction::WEST;
        } else if x > 0.0 {
            if y > 0.0 {
                return Direction::NORTHEAST;
            } else if y < 0.0 {
                return Direction::SOUTHEAST;
            }
            return Direction::EAST;
        } else if x == 0.0 {
            if y > 0.0 {
                return Direction::NORTH;
            } else if y < 0.0 {
                return Direction::SOUTH;
            }
        }

        return Direction::IDLE;
    }
}