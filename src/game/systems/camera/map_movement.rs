use core::amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::{Join, Read, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage},
    input::{InputHandler, StringBindings},
};
use core::Vector3;

use crate::game::{components::CameraFollow, config::GameConfig, resources::ViewShed};

pub struct MapMovementSystem;

impl<'s> System<'s> for MapMovementSystem {
    type SystemData = (
        ReadStorage<'s, CameraFollow>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        ReadExpect<'s, GameConfig>,
        Read<'s, Time>,
        WriteExpect<'s, ViewShed>,
    );

    fn run(
        &mut self,
        (views, mut transforms, input, config, time, mut viewshed): Self::SystemData,
    ) {
        let x_move = input.axis_value("move_x").unwrap();
        let y_move = input.axis_value("move_y").unwrap();

        if x_move == 0.0 && y_move == 0.0 {
            return;
        }

        let delta = time.delta_seconds();
        for (_, transform) in (&views, &mut transforms).join() {
            let map_move = delta * config.map_move_speed;
            transform.append_translation(Vector3::new(
                x_move as f32 * map_move,
                y_move as f32 * map_move,
                0.0,
            ));
        }

        viewshed.request_update = true;
    }
}
