use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Default)]
pub struct Floor;

impl Component for Floor {
    type Storage = DenseVecStorage<Self>;
}