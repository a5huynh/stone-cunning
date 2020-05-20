use core::{
    amethyst::ecs::{Component, VecStorage},
    Point3,
};

mod object;
mod resource;
mod worker;

pub use object::*;
pub use resource::*;
pub use worker::*;

#[derive(Clone, Component, Debug)]
#[storage(VecStorage)]
pub struct EntityInfo {
    pub pos: Point3<i32>,
    pub z_offset: f32,
}
