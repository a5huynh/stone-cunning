use core::amethyst::{core::math::Vector3, renderer::Camera, window::ScreenDimensions};

/// Utility function to convert mouse coordinates into world coordinates.
pub fn camera_to_world(
    mx: f32,
    my: f32,
    map_transform: (f32, f32),
    screen: &ScreenDimensions,
    camera: &Camera,
    camera_scale: &Vector3<f32>,
) -> (f32, f32) {
    if let Some(projection) = camera.projection().as_orthographic() {
        let scene_x = mx as f32 / screen.width()
            * ((projection.right() - projection.left()).abs() * camera_scale.x)
            - (projection.right() * camera_scale.x)
            + map_transform.0;

        let scene_y = -my as f32 / screen.height()
            * ((projection.top() - projection.bottom()).abs() * camera_scale.y)
            + (projection.top() * camera_scale.y)
            + map_transform.1;

        return (scene_x, scene_y);
    }

    (0.0, 0.0)
}
