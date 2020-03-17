use core::amethyst::{
    core::Transform,
    ecs::{Join, Read, ReadExpect, ReadStorage, System, Write, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::Camera,
    window::ScreenDimensions,
};

use crate::game::{
    components::{Cursor, CursorSelected, PickInfo},
    resources::MapRenderer,
};
use core::{Point3, Vector2};
use libdwarf::resources::Map;

pub struct CursorSystem;

impl<'s> System<'s> for CursorSystem {
    type SystemData = (
        WriteStorage<'s, Cursor>,
        Write<'s, CursorSelected>,
        Read<'s, InputHandler<StringBindings>>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
        ReadExpect<'s, ScreenDimensions>,
        ReadExpect<'s, Map>,
        ReadExpect<'s, MapRenderer>,
    );

    fn run(
        &mut self,
        (
            mut cursors,
            mut cursor_selected,
            input,
            mut transforms,
            cameras,
            screen,
            map,
            map_render,
        ): Self::SystemData,
    ) {
        // Grab the transform from map movement.
        let screen_dimensions = Vector2::new(screen.width(), screen.height());

        // Grab the zoom level of the camera
        let camera_transform = (&transforms, &cameras).join().next().or(None);
        let (scene_x, scene_y) = {
            if let Some(mouse_pos) = input.mouse_position() {
                if let Some((transform, camera)) = camera_transform {
                    let world_point = camera.projection().screen_to_world_point(
                        Point3::new(mouse_pos.0, mouse_pos.1, transform.translation().z),
                        screen_dimensions,
                        transform,
                    );

                    (world_point.x, world_point.y)
                } else {
                    (0.0, 0.0)
                }
            } else {
                (0.0, 0.0)
            }
        };

        // Update the cursor position
        let cursor_transform = (&mut cursors, &mut transforms).join().next().or(None);
        if let Some((_, cursor_transform)) = cursor_transform {
            let mut pick_info = PickInfo::default();
            // These are the base map coords where z == 0. To find whats currently
            // shown on the map, we'll loop through the z levels.
            // let mut map_z = 0;
            let (map_x, map_y) = map_render.to_map_coords(scene_x, scene_y);

            // From the view port, the tallest z level from lower (x,y) coordinates will
            // show up over ones from higher ones.
            let mut map_pt = Point3::new(map_x - 63, map_y - 63, 0);
            for z in (0..64).rev() {
                map_pt.z = z;
                // Loop until we find the first piece of terrain.
                if map.is_inside_map(map_pt) {
                    let biome = map.terrain_at(map_pt);
                    if biome.is_some() {
                        pick_info.terrain = biome;
                        break;
                    }
                }

                map_pt.x += 1;
                map_pt.y += 1;
            }

            pick_info.position = Some(map_pt);

            // Move cursor to new position.
            let new_transform = map_render.place(
                &Point3::new(map_pt.x as u32, map_pt.y as u32, map_pt.z as u32),
                0.0
            );

            *cursor_transform = new_transform;

            // If there are worker/objects at this location, show debug info about
            // those
            map_pt.z += 1;
            pick_info.worker = map.worker_at(map_pt);
            pick_info.object = map.objects_at(map_pt);
            cursor_selected.hover_selected = Some(pick_info);
        }
    }
}
