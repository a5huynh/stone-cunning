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

#[derive(Default)]
pub struct Player {
    // Direction the player is facing radians
    pub direction: f32,
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
}