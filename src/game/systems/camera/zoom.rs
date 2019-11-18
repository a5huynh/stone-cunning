use core::amethyst::{
    core::{math::Vector3, transform::Transform},
    ecs::{Join, Read, System, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::Camera,
};

use crate::game::config::GameConfig;

pub struct CameraZoomSystem;
impl<'s> System<'s> for CameraZoomSystem {
    type SystemData = (
        WriteStorage<'s, Camera>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, GameConfig>,
    );

    fn run(&mut self, (mut cameras, mut transforms, input, config): Self::SystemData) {
        let scroll_z = input.axis_value("zoom").unwrap();

        for (_, transform) in (&mut cameras, &mut transforms).join() {
            if scroll_z != 0.0 {
                let zoom = scroll_z * config.zoom_speed;
                let scale = transform.scale();
                let scale = Vector3::new(
                    (scale.x + zoom).max(config.zoom_min).min(config.zoom_max),
                    (scale.y + zoom).max(config.zoom_min).min(config.zoom_max),
                    (scale.z + zoom).max(config.zoom_min).min(config.zoom_max),
                );
                transform.set_scale(scale);
            }
        }
    }
}
