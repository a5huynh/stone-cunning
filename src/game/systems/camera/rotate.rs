use core::amethyst::{
    core::transform::Transform,
    ecs::{Join, Read, ReadStorage, System, WriteExpect, WriteStorage},
    input::{InputHandler, StringBindings},
};
use core::WorldPos;

use crate::game::{
    components::CameraFollow,
    config::GameConfig,
    resources::{MapResource, ViewShed},
};
use libdwarf::components::EntityInfo;

pub struct MapRotateSystem;
impl<'s> System<'s> for MapRotateSystem {
    type SystemData = (
        WriteExpect<'s, MapResource>,
        WriteStorage<'s, EntityInfo>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, GameConfig>,
        WriteExpect<'s, ViewShed>,
        ReadStorage<'s, CameraFollow>,
    );

    fn run(
        &mut self,
        (mut map, mut map_things, mut transforms, input, _config, mut viewshed, cameras): Self::SystemData,
    ) {
        let rotate_left = input.action_is_down("rotate_world_left").unwrap_or(false);
        let rotate_right = input.action_is_down("rotate_world_right").unwrap_or(false);

        if !rotate_left && !rotate_right && map.rotate_cooldown {
            map.rotate_cooldown = false;
        }

        // Only handle rotation when the user hits the appropriate key once.
        if !map.rotate_cooldown && (rotate_left || rotate_right) {
            if rotate_left {
                map.rotate_ccw();
            } else if rotate_right {
                map.rotate_cw();
            }

            for (thing, transform) in (&mut map_things, &mut transforms).join() {
                // Move map object / terrain to new place.
                let pos = &thing.pos;
                *transform = map.place(&pos, thing.z_offset);
            }

            let center_offset = viewshed
                .center_world
                .unwrap_or_else(|| WorldPos::new(0, 0, 0));

            for (_, transform) in (&cameras, &mut transforms).join() {
                let new_center = map.rotate_camera(&center_offset);
                let mut cam_transform = map.place(&new_center, 0.0);
                cam_transform.set_translation_z(0.0);
                *transform = cam_transform;
            }

            viewshed.request_update = true;
        }
    }
}
