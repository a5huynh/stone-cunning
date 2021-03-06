use core::amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::{Join, Read, ReadExpect, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::game::{components::CameraFollow, config::GameConfig};

pub struct MapMovementSystem;

impl<'s> System<'s> for MapMovementSystem {
    type SystemData = (
        ReadStorage<'s, CameraFollow>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        ReadExpect<'s, GameConfig>,
        Read<'s, Time>,
    );

    fn run(&mut self, (cameras, mut transforms, input, config, time): Self::SystemData) {
        let x_move = input.axis_value("move_x").unwrap();
        let y_move = input.axis_value("move_y").unwrap();

        let delta = time.delta_seconds();
        for (_, transform) in (&cameras, &mut transforms).join() {
            let map_move = delta * config.map_move_speed;
            transform.prepend_translation_x(x_move as f32 * map_move);
            transform.prepend_translation_y(y_move as f32 * map_move);
        }
    }
}
