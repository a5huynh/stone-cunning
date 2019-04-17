use specs_derive::*;
use specs::{
    Component,
    VecStorage,
};

mod object;
mod resource;
mod worker;

pub use object::*;
pub use resource::*;
pub use worker::*;

#[derive(Clone, Component, Debug)]
#[storage(VecStorage)]
pub struct MapPosition {
    pub x: u32,
    pub y: u32,
}