use amethyst::{
    core::{
        Transform,
        nalgebra::{ Orthographic3 },
    },
    ecs::{
        Join,
        Read,
        ReadExpect,
        ReadStorage,
        System,
        WriteStorage,
        Write,
    },
    input::InputHandler,
    renderer::{
        Camera,
        ScreenDimensions,
    }
};

use libdwarf::resources::Map;
use crate::game::{
    entity::{
        CameraFollow,
        Cursor,
        PickInfo,
        CursorSelected,
    },
    render::MapRenderer,
};

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

    fn run(&mut self, (
        mut cursors,
        mut cursor_selected,
        input,
        mut transforms,
        cameras,
        follow,
        screen_dim,
        map,
        map_render,
    ): Self::SystemData) {
        // render on screen cursor
        let camera_follow = (&transforms, &follow).join()
            .next().or(None).clone();

        let (map_transform_x, map_transform_y) = {
            if let Some((follow_transform, _)) = camera_follow {
                (
                    follow_transform.translation().x,
                    follow_transform.translation().y
                )
            } else {
                (0.0, 0.0)
            }
        };

        let camera_transform = (&transforms, &cameras).join()
            .next().or(None).clone();

        let (scene_x, scene_y) = {
            if let Some((mx, my)) = input.mouse_position() {
                if let Some((_, camera)) = camera_transform {
                    let projection = Orthographic3::from_matrix_unchecked(camera.proj);

                    let scene_x = mx as f32 / screen_dim.width() *
                        (projection.right() - projection.left()).abs()
                        - projection.right()
                        + map_transform_x;

                    let scene_y = -my as f32 / screen_dim.height() *
                        (projection.top() - projection.bottom()).abs()
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

        for (_cursor, transform) in (&mut cursors, &mut transforms).join() {
            let (map_x, map_y) = map_render.to_map_coords(scene_x, scene_y);
            // If there are objects at this location, show debug info about those
            if let Some(object) = map.objects_at(map_x, map_y) {
                cursor_selected.selected = Some(PickInfo {
                    is_terrain: false,
                    description: format!("{:?}", object)
                });
            // Otherwise, get the terrain at this location.
            } else if let Some(terrain) = map.terrain_at(map_x, map_y) {
                cursor_selected.selected = Some(PickInfo {
                    is_terrain: true,
                    description: format!("{:?}", terrain)
                });
            } else {
                cursor_selected.selected = None
            }

            let new_transform = map_render.place(map_x, map_y, 0.0);
            transform.set_x(new_transform.translation().x);
            transform.set_y(new_transform.translation().y);
        }
    }
}