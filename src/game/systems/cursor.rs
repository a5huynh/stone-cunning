use core::amethyst::{
    core::Transform,
    ecs::{Join, Read, ReadExpect, ReadStorage, System, Write, WriteExpect, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::Camera,
    window::ScreenDimensions,
};

use crate::game::{
    components::{Cursor, CursorSelected, Direction, PickInfo},
    resources::MapRenderer,
};
use core::{Point3, Vector2, WorldPos};
use libdwarf::resources::World;
use libterrain::ChunkEntity;

pub struct CursorSystem;

impl<'s> System<'s> for CursorSystem {
    type SystemData = (
        WriteStorage<'s, Cursor>,
        Write<'s, CursorSelected>,
        Read<'s, InputHandler<StringBindings>>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
        ReadExpect<'s, ScreenDimensions>,
        WriteExpect<'s, World>,
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
            mut world,
            map_render,
        ): Self::SystemData,
    ) {
        let screen_dimensions = Vector2::new(screen.width(), screen.height());
        // Convert screen point to world point
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
            let mut current_pt = Point3::new(map_x, map_y, 0);
            let mut above_pt = Point3::new(0, 0, 0);
            let mut valid_pt = current_pt;

            // Start at the highest point
            for z in 0..63 {
                current_pt.z = z;
                // Pointer to terrain above the current tile.
                above_pt.x = current_pt.x;
                above_pt.y = current_pt.y;
                above_pt.z = z + 1;

                // Loop until we find the first piece of visible terrain.
                let biome = world.terrain.get(&current_pt);
                let above = world.terrain.get(&above_pt);

                if biome.is_some() && above.is_none() {
                    if let Some(ChunkEntity::Terrain { biome, visible: _ }) = biome {
                        pick_info.terrain = Some(biome);
                        // Last valid point we've seen.
                        valid_pt.x = current_pt.x;
                        valid_pt.y = current_pt.y;
                        valid_pt.z = current_pt.z;
                    }
                }

                // Based on the current rotation, we'll want to search for the
                // correct z-level in different ways.
                match map_render.rotation {
                    Direction::NORTH => {
                        current_pt.x -= 1;
                        current_pt.y -= 1;
                    }
                    Direction::EAST => {
                        current_pt.x -= 1;
                        current_pt.y += 1;
                    }
                    Direction::SOUTH => {
                        current_pt.x += 1;
                        current_pt.y += 1;
                    }
                    Direction::WEST => {
                        current_pt.x += 1;
                        current_pt.y -= 1;
                    }
                }
            }

            pick_info.world_pos = Some(Point3::new(scene_x, scene_y, 0.0));
            pick_info.position = Some(valid_pt);

            // Move cursor to new position.
            let new_transform =
                map_render.place(&WorldPos::new(valid_pt.x, valid_pt.y, valid_pt.z), 0.1);

            *cursor_transform = new_transform;

            // If there are worker/objects at this location, show debug info about
            // those
            valid_pt.z += 1;
            // pick_info.worker = map.worker_at(valid_pt);
            if let Some(ChunkEntity::Object(uuid, _object)) = world.terrain.get(&valid_pt) {
                pick_info.object = Some(uuid);
            }

            cursor_selected.hover_selected = Some(pick_info);
        }
    }
}
