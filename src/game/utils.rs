use amethyst::{core::Float, renderer::Camera, window::ScreenDimensions};

/// Utility function to convert mouse coordinates into world coordinates.
pub fn camera_to_world(
    mx: f64,
    my: f64,
    map_transform: (Float, Float),
    screen: &ScreenDimensions,
    camera: &Camera,
) -> (f32, f32) {
    if let Some(projection) = camera.projection().as_orthographic() {
        let scene_x = mx as f32 / screen.width() * (projection.right() - projection.left()).abs()
            - projection.right()
            + map_transform.0.as_f32();

        let scene_y = -my as f32 / screen.height() * (projection.top() - projection.bottom()).abs()
            + projection.top()
            + map_transform.1.as_f32();

        return (scene_x, scene_y);
    }

    (0.0, 0.0)
}
