use libterrain::Point3;
use specs::{Component, VecStorage};

mod object;
mod resource;
mod worker;

pub use object::*;
pub use resource::*;
pub use worker::*;

#[derive(Clone, Component, Debug)]
#[storage(VecStorage)]
pub struct MapPosition {
    pub pos: Point3<u32>,
}
