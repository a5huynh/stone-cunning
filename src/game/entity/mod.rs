use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{
        SpriteRender,
        Transparent,
    },
};

mod terrain;
mod player;
pub use terrain::*;
pub use player::*;

use crate::game::{
    sprite::SpriteSheetStorage,
};

#[derive(Default)]
/// Used to move the camera and to follow around other entities
pub struct CameraFollow;
impl Component for CameraFollow {
    type Storage = DenseVecStorage<Self>;
}

// #[derive(Clone, Debug, PartialEq)]
pub enum Direction {
    WEST,
    NORTH,
    EAST,
    SOUTH,
}

#[derive(Debug)]
pub struct PickInfo {
    pub is_terrain: bool,
    pub description: String,
}

#[derive(Default)]
pub struct Cursor;
impl Component for Cursor {
    type Storage = DenseVecStorage<Self>;
}

impl Cursor {
    pub fn initialize(world: &mut World) {
        let cursor = {
            let sheets = world.read_resource::<SpriteSheetStorage>();
            sheets.cursor.clone()
        };

        let sprite_render = SpriteRender {
            sprite_sheet: cursor,
            sprite_number: 0,
        };

        world.create_entity()
            .with(sprite_render)
            .with(Cursor::default())
            .with(Transform::default())
            .with(Transparent)
            .build();
    }
}

/// Represents the currently selected thing (object, terrain, etc.) under the cursor
#[derive(Default)]
pub struct CursorSelected {
    pub selected: Option<PickInfo>,
}