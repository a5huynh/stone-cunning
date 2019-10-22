use specs::prelude::*;
use specs_derive::*;
use std::collections::VecDeque;

use crate::trigger::TriggerType;

#[derive(Clone, Component, Debug)]
#[storage(VecStorage)]
pub struct Worker {
    /// Energy a worker has. Each action depletes energy. One it reaches, 0
    /// it'll have to wait a couple frames before it can do something else.
    pub energy: f32,
    /// Queue of actions this worker has. e.g. a queue might look like the
    /// following for a worker:
    /// - MoveTo -> x, x
    /// - PerformAction(Chop) @ x,x
    ///
    /// The worker needs to MoveTo some location first before they are able
    /// to perform an action.
    pub actions: VecDeque<TriggerType>,
    /// Worker's inventory.
    pub inventory: Vec<u32>,
}

impl Worker {
    pub fn new() -> Self {
        Worker {
            energy: 1.0,
            actions: Default::default(),
            inventory: Default::default(),
        }
    }

    pub fn to_string(&self) -> String {
        format!("({})", self.energy)
    }
}
