use core::amethyst::{
    core::transform::Transform,
    ecs::{Join, Read, System, WriteExpect, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::game::{
    config::GameConfig,
    resources::{MapRenderer, ViewShed},
};
use libdwarf::components::EntityInfo;

pub struct MapRotateSystem;
impl<'s> System<'s> for MapRotateSystem {
    type SystemData = (
        WriteExpect<'s, MapRenderer>,
        WriteStorage<'s, EntityInfo>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, GameConfig>,
        WriteExpect<'s, ViewShed>,
    );

    fn run(
        &mut self,
        (mut map, mut map_things, mut transforms, input, _config, mut viewshed): Self::SystemData,
    ) {
        let rotate_left = input.action_is_down("rotate_world_left").unwrap_or(false);
        let rotate_right = input.action_is_down("rotate_world_right").unwrap_or(false);

        if !rotate_left && !rotate_right && map.rotate_cooldown {
            map.rotate_cooldown = false;
        }

        // Only handle rotation when the user hits the appropriate key once.
        if !map.rotate_cooldown && (rotate_left || rotate_right) {
            if rotate_left {
                map.rotate_left();
            } else if rotate_right {
                map.rotate_right();
            }

            for (thing, transform) in (&mut map_things, &mut transforms).join() {
                // Move map object / terrain to new place.
                let pos = &thing.pos;
                *transform = map.place(&pos, thing.z_offset);
            }

            viewshed.request_update = true;
        }
    }
}
