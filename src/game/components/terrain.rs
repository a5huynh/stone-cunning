use amethyst::ecs::prelude::{Component, DenseVecStorage};
use specs_derive::*;

#[derive(Component, Default)]
#[storage(DenseVecStorage)]
pub struct Object;
