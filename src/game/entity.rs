use amethyst::ecs::prelude::{Component, DenseVecStorage};


#[derive(Default)]
/// Used to move the camera and to follow around other entities
pub struct CameraFollow;
impl Component for CameraFollow {
    type Storage = DenseVecStorage<Self>;
}


#[derive(Default)]
pub struct Floor;
impl Component for Floor {
    type Storage = DenseVecStorage<Self>;
}