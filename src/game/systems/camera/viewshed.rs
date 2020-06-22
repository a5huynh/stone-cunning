use core::amethyst::{
    core::transform::Transform,
    ecs::{Join, ReadExpect, ReadStorage, System, WriteExpect},
    renderer::Camera,
    window::ScreenDimensions,
};
use core::{Point3, Vector2, WorldPos};

use crate::game::resources::{MapRenderer, ViewShed};

pub struct ViewshedUpdaterSystem;

impl<'s> System<'s> for ViewshedUpdaterSystem {
    type SystemData = (
        ReadStorage<'s, Camera>,
        ReadStorage<'s, Transform>,
        ReadExpect<'s, MapRenderer>,
        ReadExpect<'s, ScreenDimensions>,
        WriteExpect<'s, ViewShed>,
    );

    fn run(&mut self, (cameras, transforms, map_render, screen, mut viewshed): Self::SystemData) {
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
            viewshed.top_left = Some(top_left);
            viewshed.top_left_world = Some(WorldPos::new(tl_pos.0, tl_pos.1, 0));

            let br_pos = map_render.to_map_coords(bottom_right.x, bottom_right.y);
            viewshed.bottom_right = Some(bottom_right);
            viewshed.bottom_right_world = Some(WorldPos::new(br_pos.0, br_pos.1, 0));

            viewshed.dirty();
        }
    }
}
