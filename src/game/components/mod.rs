mod camera;
mod cursor;
mod player;
mod terrain;
pub use camera::*;
pub use cursor::*;
pub use player::*;
pub use terrain::*;

use core::Point3;
use libterrain::Biome;

#[derive(Clone, Debug, Default)]
pub struct PickInfo {
    pub worker: Option<u32>,
    pub object: Option<u32>,
    pub terrain: Option<Biome>,
    pub world_pos: Option<Point3<f32>>,
    pub position: Option<Point3<i32>>,
}
