use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{
        SpriteSheetHandle,
        SpriteRender,
        Transparent,
    },
};

use super::Direction;

#[derive(Default)]
pub struct Player {
    // Used to keep track of the player's animation frame.
    pub last_tick: f32,
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

impl Player {
    pub fn initialize(world: &mut World, player_sprite: SpriteSheetHandle) {
        let mut player_transform = Transform::default();
        player_transform.set_xyz(0.0, 0.0, 1.0);

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
            return Direction::WEST;
        } else if x > 0.0 {
            return Direction::EAST;
        } else if x == 0.0 {
            if y > 0.0 {
                return Direction::NORTH;
            } else if y < 0.0 {
                return Direction::SOUTH;
            }
        }

        return Direction::NORTH;
    }
}