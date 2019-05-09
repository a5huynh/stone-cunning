use amethyst::ecs::prelude::{Component, DenseVecStorage};

#[derive(Default)]
pub struct Object;
impl Component for Object {
    type Storage = DenseVecStorage<Self>;
}
