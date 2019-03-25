use amethyst::{
    assets::{ Loader },
    ecs::prelude::{Component, DenseVecStorage, Entity},
    prelude::*,
    ui::{
        Anchor,
        TtfFormat,
        UiText,
        UiTransform,
    }
};

mod terrain;
mod player;
pub use terrain::*;
pub use player::*;

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