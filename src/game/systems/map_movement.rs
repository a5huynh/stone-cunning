use amethyst::{
    core::transform::Transform,
    ecs::{
        Join,
        Read,
        ReadExpect,
        ReadStorage,
        System,
        WriteStorage,
    },
    input::InputHandler,
};

use super::super::{
    config::GameConfig,
    entity::CameraFollow
};

pub struct MapMovementSystem;

impl<'s> System<'s> for MapMovementSystem {
    type SystemData = (
        ReadStorage<'s, CameraFollow>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<String, String>>,
        ReadExpect<'s, GameConfig>,
    );

    fn run(&mut self, (cameras, mut transforms, input, config): Self::SystemData) {
        let x_move = input.axis_value("move_x").unwrap();
        let y_move = input.axis_value("move_y").unwrap();

        for (_, transform) in (&cameras, &mut transforms).join() {
            transform.translate_x(x_move as f32 * config.map_move_speed);
            transform.translate_y(y_move as f32 * config.map_move_speed);
        }
    }
}