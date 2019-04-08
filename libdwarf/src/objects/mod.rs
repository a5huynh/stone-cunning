mod resource;
pub use resource::*;

use crate::{
    actors::Actor
};

#[derive(Clone, Debug)]
pub struct MapObject {
    pub id: u32
}

impl MapObject {
    pub fn new(id: u32) -> Self {
        MapObject { id }
    }
}

impl Actor for MapObject {
    fn tick(&mut self) {

    }
}