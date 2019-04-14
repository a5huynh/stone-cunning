use crate::{
    actions::Action,
    entities::MapObject,
    world::WorldUpdate,
};

pub trait Actor {
    /// Actor ID, used to receive messages and identify this actor.
    fn id(&self) -> u32;
    /// Queue up actions.
    fn queue(&mut self, action: &Action);
    /// Process all pending actions.
    fn tick(&mut self, neighbors: Vec<Option<&MapObject>>) -> Option<WorldUpdate>;
}