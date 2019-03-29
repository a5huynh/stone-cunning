use amethyst::{
    assets::{ Loader },
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage, Entity},
    prelude::*,
    renderer::{
        SpriteSheetHandle,
        SpriteRender,
        Transparent,
    },
    ui::{
        Anchor,
        TtfFormat,
        UiText,
        UiTransform,
    }
};

mod terrain;
mod player;
mod npc;
pub use terrain::*;
pub use player::*;
pub use npc::*;

use crate::game::map::PickInfo;

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

#[derive(Default)]
pub struct Cursor;
impl Component for Cursor {
    type Storage = DenseVecStorage<Self>;
}

impl Cursor {
    pub fn initialize(world: &mut World, sprite_sheet: SpriteSheetHandle) {
        let sprite_render = SpriteRender {
            sprite_sheet,
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