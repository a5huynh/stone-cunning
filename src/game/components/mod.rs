mod camera;
mod cursor;
mod player;
mod terrain;
pub use camera::*;
pub use cursor::*;
pub use player::*;
pub use terrain::*;

use core::{Point3, Uuid, WorldPos};
use libterrain::Biome;

#[derive(Clone, Debug, Default)]
pub struct PickInfo {
    pub worker: Option<Uuid>,
    pub object: Option<Uuid>,
    pub terrain: Option<Biome>,
    pub world_pos: Option<Point3<f32>>,
    pub position: Option<WorldPos>,
}

#[derive(Clone, Debug, Default)]
pub struct PassInfo {
    pub num_entities: Option<usize>,
    pub walltime: Option<u128>,
}
