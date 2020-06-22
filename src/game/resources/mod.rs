use core::{Point3, WorldPos};
mod map;
pub use map::*;

#[derive(Default)]
/// Top-left most & bottom-right most map coordinates in view.
pub struct ViewShed {
    pub top_left_world: Option<WorldPos>,
    pub bottom_right_world: Option<WorldPos>,
    pub top_left: Option<Point3<f32>>,
    pub bottom_right: Option<Point3<f32>>,

    /// Viewshed needs an update.
    pub request_update: bool,

    /// Flags
    pub needs_chunking: bool,
    pub needs_sort: bool,
}

impl ViewShed {
    pub fn dirty(&mut self) {
        self.needs_chunking = true;
        self.needs_sort = true;
    }
}
