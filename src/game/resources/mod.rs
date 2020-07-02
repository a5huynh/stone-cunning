use core::{Point3, WorldPos};
mod map;
pub use map::*;

/// Top-left most & bottom-right most map coordinates in view.
pub struct ViewShed {
    pub top_left_world: Option<WorldPos>,
    pub bottom_right_world: Option<WorldPos>,
    pub top_left: Option<Point3<f32>>,
    pub bottom_right: Option<Point3<f32>>,

    /// Center point for current view.
    /// The map will be rotated around this point.
    pub center_world: Option<WorldPos>,

    /// Viewshed needs an update.
    pub request_update: bool,

    /// Flags
    pub needs_chunking: bool,
    pub needs_sort: bool,
}

impl Default for ViewShed {
    fn default() -> Self {
        ViewShed {
            top_left_world: None,
            bottom_right_world: None,
            top_left: None,
            bottom_right: None,
            center_world: None,
            request_update: true,
            needs_chunking: false,
            needs_sort: false,
        }
    }
}

impl ViewShed {
    pub fn dirty(&mut self) {
        self.needs_chunking = true;
        self.needs_sort = true;
    }
}
