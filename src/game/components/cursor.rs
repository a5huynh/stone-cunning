use crate::game::{components::PickInfo, sprite::SpriteSheetStorage};
use core::amethyst::{
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage, NullStorage},
    prelude::*,
    renderer::{SpriteRender, Transparent},
};
use specs_derive::*;

#[derive(Component, Default)]
#[storage(DenseVecStorage)]
pub struct Cursor;

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

        world
            .create_entity()
            .with(sprite_render)
            .with(Cursor::default())
            .with(Transform::default())
            .with(Transparent)
            .build();
    }
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct CursorDown;

/// Represents the currently selected thing (object, terrain, etc.) under the cursor
#[derive(Default)]
pub struct CursorSelected {
    /// Entity that was clicked on.
    pub pinned: Option<PickInfo>,
    /// Entity currently under the mouse cursor.
    pub hover_selected: Option<PickInfo>,
}
