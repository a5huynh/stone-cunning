use core::amethyst::{
    core::transform::Transform,
    ecs::{Join, Read, ReadExpect, System, WriteExpect, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::game::{
    config::GameConfig,
    resources::{CameraWindow, MapRenderer},
};
use libdwarf::components::MapPosition;

pub struct MapRotateSystem;
impl<'s> System<'s> for MapRotateSystem {
    type SystemData = (
        ReadExpect<'s, MapRenderer>,
        WriteExpect<'s, CameraWindow>,
        WriteStorage<'s, MapPosition>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, GameConfig>,
    );

    fn run(
        &mut self,
        (map_render, mut camera_window, mut map_things, mut transforms, input, _config): Self::SystemData,
    ) {
        let rotate_left = input.action_is_down("rotate_world_left").unwrap_or(false);
        let rotate_right = input.action_is_down("rotate_world_right").unwrap_or(false);

        if !rotate_left && !rotate_right && camera_window.rotate_cooldown {
            camera_window.rotate_cooldown = false;
        }

        // Only handle rotation when the user hits the appropriate key once.
        if !camera_window.rotate_cooldown && (rotate_left || rotate_right) {
            if rotate_left {
                camera_window.rotate_left();
            } else if rotate_right {
                camera_window.rotate_right();
            }

            for (thing, transform) in (&mut map_things, &mut transforms).join() {
                // Move map object / terrain to new place.
                let pos = &thing.pos;
                *transform = map_render.place(&pos, 1.0, camera_window.rotation);
            }
        }
    }
}
