use core::WorldPos;
mod map;
pub use map::*;

#[derive(Default)]
/// Top-left most & bottom-right most map coordinates in view.
pub struct ViewShed {
    pub top_left: Option<WorldPos>,
    pub bottom_right: Option<WorldPos>,
    pub dirty: bool,
}
