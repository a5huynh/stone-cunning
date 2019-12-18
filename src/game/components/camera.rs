use core::amethyst::ecs::prelude::{Component, DenseVecStorage};
use specs_derive::*;

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

#[derive(Component, Default)]
#[storage(DenseVecStorage)]
/// Used to move the camera and to follow around other entities
pub struct CameraFollow;
