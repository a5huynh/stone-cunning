use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{SpriteRender, Transparent},
};

use crate::game::{entity::PickInfo, sprite::SpriteSheetStorage};

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

        world
            .create_entity()
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
    /// Entity that was clicked on.
    pub pinned: Option<PickInfo>,
    /// Entity currently under the mouse cursor.
    pub hover_selected: Option<PickInfo>,
}
