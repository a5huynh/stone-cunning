use amethyst::ecs::prelude::{Component, DenseVecStorage};

mod cursor;
mod player;
mod terrain;
pub use cursor::*;
pub use player::*;
pub use terrain::*;

use libdwarf::Point3;
use libterrain::Biome;

#[derive(Default)]
/// Used to move the camera and to follow around other entities
pub struct CameraFollow;
impl Component for CameraFollow {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone, Debug, Default)]
pub struct PickInfo {
    pub worker: Option<u32>,
    pub object: Option<u32>,
    pub terrain: Option<Biome>,
    pub position: Option<Point3<i32>>,
}
