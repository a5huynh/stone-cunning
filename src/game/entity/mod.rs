use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{SpriteRender, Transparent},
};

mod player;
mod terrain;
pub use player::*;
pub use terrain::*;

use crate::game::sprite::SpriteSheetStorage;
use libdwarf::Point3;
use libterrain::Biome;

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

#[derive(Clone, Debug, Default)]
pub struct PickInfo {
    pub worker: Option<u32>,
    pub object: Option<u32>,
    pub terrain: Option<Biome>,
    pub position: Option<Point3<i32>>,
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
