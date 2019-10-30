use core::amethyst::{
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{SpriteRender, Transparent},
};
use specs_derive::*;

use crate::game::sprite::SpriteSheetStorage;

#[derive(Component, Default)]
#[storage(DenseVecStorage)]
pub struct Player {
    // Used to keep track of the player's animation frame.
    pub last_tick: f32,
}

impl Player {
    pub fn initialize(world: &mut World) {
        let sprite_sheet = {
            let sheets = world.read_resource::<SpriteSheetStorage>();
            sheets.player.clone()
        };

        let mut player_transform = Transform::default();
        player_transform.set_translation_xyz(0.0, 0.0, 1.0);

        let sprite_render = SpriteRender {
            sprite_sheet,
            sprite_number: 0,
        };

        world
            .create_entity()
            .with(sprite_render.clone())
            .with(Player::default())
            .with(player_transform)
            .with(Transparent)
            .build();
    }
}
