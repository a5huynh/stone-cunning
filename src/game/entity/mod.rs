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

pub struct ActivityConsole {
    pub text_handle: Entity,
}

impl Component for ActivityConsole {
    type Storage = DenseVecStorage<Self>;
}

// TODO: reimplement as ui widget.
impl ActivityConsole {
    pub fn initialize(world: &mut World) {
        let font = world.read_resource::<Loader>().load(
            "resources/fonts/PxPlus_IBM_VGA8.ttf",
            TtfFormat,
            Default::default(),
            (),
            &world.read_resource(),
        );

        let transform = UiTransform::new(
            "DEBUG".to_string(),
            Anchor::TopLeft,
            // x, y, z
            140.0, -20.0, 1.0,
            // width, height
            400.0, 40.0,
            // Tab order
            0
        );

        let text_handle = world.create_entity()
            .with(transform)
            .with(UiText::new(
                font.clone(),
                "Player is on x, y:".to_string(),
                [1., 1., 1., 1.],
                25.,
            )).build();

        world.add_resource(ActivityConsole { text_handle });
    }
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