use core::{
    amethyst::ecs::{Component, VecStorage},
    WorldPos,
    Uuid,
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
    pub uuid: Uuid,
    pub pos: WorldPos,
    pub z_offset: f32,
}
