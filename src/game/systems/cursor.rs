use amethyst::{
    core::{nalgebra::Orthographic3, Transform},
    ecs::{Join, Read, ReadExpect, ReadStorage, System, Write, WriteStorage},
    input::InputHandler,
    renderer::{Camera, ScreenDimensions},
};

use crate::game::{
    entity::{CameraFollow, Cursor, CursorSelected, PickInfo},
    render::MapRenderer,
};
use libdwarf::{resources::Map, Point3};

pub struct CursorSystem;

impl<'s> System<'s> for CursorSystem {
    type SystemData = (
        WriteStorage<'s, Cursor>,
        Write<'s, CursorSelected>,
        Read<'s, InputHandler<String, String>>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
        ReadStorage<'s, CameraFollow>,
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
            follow,
            screen_dim,
            map,
            map_render,
        ): Self::SystemData,
    ) {
        // Grab the transform from map movement.
        let camera_follow = (&transforms, &follow).join().next().or(None).clone();
        let (map_transform_x, map_transform_y) = {
            if let Some((follow_transform, _)) = camera_follow {
                (
                    follow_transform.translation().x,
                    follow_transform.translation().y,
                )
            } else {
                (0.0, 0.0)
            }
        };
        // Grab the zoom level of the camera
        let camera_transform = (&transforms, &cameras).join().next().or(None).clone();
        // Convert mouse position into scene coordinates.
        let (scene_x, scene_y) = {
            if let Some((mx, my)) = input.mouse_position() {
                if let Some((_, camera)) = camera_transform {
                    let projection = Orthographic3::from_matrix_unchecked(camera.proj);

                    let scene_x = mx as f32 / screen_dim.width()
                        * (projection.right() - projection.left()).abs()
                        - projection.right()
                        + map_transform_x;

                    let scene_y = -my as f32 / screen_dim.height()
                        * (projection.top() - projection.bottom()).abs()
                        + projection.top()
                        + map_transform_y;

                    (scene_x, scene_y)
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
            let mut map_pt = Point3::new(map_x, map_y, 0);
            for z in 0..64 {
                map_pt.z = z;
                if map.is_inside_map(map_pt) {
                    let biome = map.terrain_at(map_pt);
                    if biome.is_some() {
                        pick_info.terrain = biome;
                        map_pt.x += 1;
                        map_pt.y += 1;
                    } else {
                        map_pt.x -= 1;
                        map_pt.y -= 1;
                        map_pt.z -= 1;
                        break;
                    }
                }
            }

            // Move cursor to new position.
            let new_transform = map_render.place(map_pt.x, map_pt.y, map_pt.z, 0.0);
            cursor_transform.set_x(new_transform.translation().x);
            cursor_transform.set_y(new_transform.translation().y);

            // If the cursor is outside of the map, don't show any debug info.
            let map_pos = Point3::new(map_x, map_y, 0);
            if !map.is_inside_map(map_pos) {
                cursor_selected.hover_selected = None;
                return;
            }

            // If there are worker/objects at this location, show debug info about
            // those
            pick_info.worker = map.worker_at(map_pos);
            pick_info.object = map.objects_at(map_pos);
            cursor_selected.hover_selected = Some(pick_info);
        }
    }
}
