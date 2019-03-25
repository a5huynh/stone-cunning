use amethyst::{
    ecs::prelude::{Component, DenseVecStorage},
};

#[derive(Default)]
pub struct Floor;
impl Component for Floor {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct Object;
impl Component for Object {
    type Storage = DenseVecStorage<Self>;
}
