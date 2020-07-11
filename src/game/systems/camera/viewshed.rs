use std::cmp::{max, min};

use core::amethyst::{
    core::transform::Transform,
    ecs::{Join, ReadExpect, ReadStorage, System, WriteExpect},
    renderer::Camera,
    window::ScreenDimensions,
};
use core::{Point3, Vector2, WorldPos};

use crate::game::resources::{MapResource, ViewShed};

pub struct ViewshedUpdaterSystem;

impl<'s> System<'s> for ViewshedUpdaterSystem {
    type SystemData = (
        ReadStorage<'s, Camera>,
        ReadStorage<'s, Transform>,
        WriteExpect<'s, MapResource>,
        ReadExpect<'s, ScreenDimensions>,
        WriteExpect<'s, ViewShed>,
    );

    fn run(&mut self, (cameras, transforms, map_render, screen, mut viewshed): Self::SystemData) {
        if !viewshed.request_update {
            return;
        }

        if let Some((camera, transform)) = (&cameras, &transforms).join().next().or(None) {
            let screen_dim = Vector2::new(screen.width(), screen.height());
            let top_left = camera.projection().screen_to_world_point(
                Point3::new(0.0, 0.0, transform.translation().z),
                screen_dim,
                transform,
            );

            let top_right = camera.projection().screen_to_world_point(
                Point3::new(screen.width(), 0.0, transform.translation().z),
                screen_dim,
                transform,
            );

            let bottom_left = camera.projection().screen_to_world_point(
                Point3::new(0.0, screen.height(), transform.translation().z),
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

            let center = camera.projection().screen_to_world_point(
                Point3::new(
                    screen.width() / 2.0,
                    screen.height() / 2.0,
                    transform.translation().z,
                ),
                screen_dim,
                transform,
            );

            // Used in view culling.
            viewshed.top_left = Some(top_left);
            viewshed.bottom_right = Some(bottom_right);
            let center_pos = map_render.to_map_coords(center.x, center.y);
            viewshed.center_world = Some(WorldPos::new(center_pos.0, center_pos.1, 0));

            // Used to determine next chunks to load, if any.
            let tl_pos = map_render.to_map_coords(top_left.x, top_left.y);
            let tr_pos = map_render.to_map_coords(top_right.x, top_right.y);
            let bl_pos = map_render.to_map_coords(bottom_left.x, bottom_left.y);
            let br_pos = map_render.to_map_coords(bottom_right.x, bottom_right.y);

            viewshed.top_left_world = Some(WorldPos::new(
                max(tl_pos.0, max(tr_pos.0, max(bl_pos.0, br_pos.0))),
                max(tl_pos.1, max(tr_pos.1, max(bl_pos.1, br_pos.1))),
                0,
            ));

            viewshed.bottom_right_world = Some(WorldPos::new(
                min(tl_pos.0, min(tr_pos.0, min(bl_pos.0, br_pos.0))),
                min(tl_pos.1, min(tr_pos.1, min(bl_pos.1, br_pos.1))),
                0,
            ));

            viewshed.dirty();
        }

        viewshed.request_update = false;
    }
}
