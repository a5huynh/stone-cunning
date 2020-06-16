use core::amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::{Join, Read, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::Camera,
    window::ScreenDimensions,
};
use core::{Point3, Vector2, Vector3, WorldPos};

use crate::game::{
    components::CameraFollow,
    config::GameConfig,
    resources::{MapRenderer, ViewShed},
};

pub struct MapMovementSystem;

impl<'s> System<'s> for MapMovementSystem {
    type SystemData = (
        ReadStorage<'s, Camera>,
        ReadStorage<'s, CameraFollow>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        ReadExpect<'s, GameConfig>,
        Read<'s, Time>,
        ReadExpect<'s, MapRenderer>,
        ReadExpect<'s, ScreenDimensions>,
        WriteExpect<'s, ViewShed>,
    );

    fn run(
        &mut self,
        (cameras, views, mut transforms, input, config, time, map_render, screen, mut viewshed): Self::SystemData,
    ) {
        let x_move = input.axis_value("move_x").unwrap();
        let y_move = input.axis_value("move_y").unwrap();

        if x_move == 0.0
            && y_move == 0.0
            // If viewshed has not been set, run through this system.
            && viewshed.top_left.is_some()
            && viewshed.bottom_right.is_some()
        {
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

        if let Some((camera, transform)) = (&cameras, &transforms).join().next().or(None) {
            let screen_dim = Vector2::new(screen.width(), screen.height());
            let top_left = camera.projection().screen_to_world_point(
                Point3::new(0.0, 0.0, transform.translation().z),
                screen_dim,
                transform,
            );

            let bottom_right = camera.projection().screen_to_world_point(
                Point3::new(
                    screen.width() as f32,
                    screen.height() as f32,
                    transform.translation().z,
                ),
                screen_dim,
                transform,
            );

            let tl_pos = map_render.to_map_coords(top_left.x, top_left.y);
            viewshed.top_left = Some(WorldPos::new(tl_pos.0, tl_pos.1, 0));
            let br_pos = map_render.to_map_coords(bottom_right.x, bottom_right.y);
            viewshed.bottom_right = Some(WorldPos::new(br_pos.0, br_pos.1, 0));
            viewshed.dirty = true;
        }
    }
}
