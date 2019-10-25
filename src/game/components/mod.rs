use amethyst::ecs::prelude::{Component, DenseVecStorage};
use specs_derive::*;

mod cursor;
mod player;
mod terrain;
pub use cursor::*;
pub use player::*;
pub use terrain::*;

use core::Point3;
use libterrain::Biome;

#[derive(Component, Default)]
#[storage(DenseVecStorage)]
/// Used to move the camera and to follow around other entities
pub struct CameraFollow;

#[derive(Clone, Debug, Default)]
pub struct PickInfo {
    pub worker: Option<u32>,
    pub object: Option<u32>,
    pub terrain: Option<Biome>,
    pub position: Option<Point3<i32>>,
}
