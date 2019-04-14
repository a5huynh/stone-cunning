use std::collections::VecDeque;
use specs_derive::*;
use specs::{ Component, VecStorage };

use crate::{
    actors::Actor,
    actions::Action,
    world::{ WorldSim, WorldUpdate },
};

#[derive(Clone, Component, Debug)]
#[storage(VecStorage)]
pub struct MapObject {
    pub id: u32,
    pub health: i32,
    pub actions: VecDeque<Action>,
    // Position on map.
    pub x: u32,
    pub y: u32,
}

impl MapObject {
    pub fn new(x: u32, y: u32) -> Self {
        MapObject {
            id: Default::default(),
            x, y,
            actions: VecDeque::new(),
            health: 10,
        }
    }
}

impl Actor for MapObject {
    fn id(&self) -> u32 { self.id }

    fn tick(&mut self, _neighbors: Vec<Option<&MapObject>>) -> Option<WorldUpdate> {
        let mut update = None;
        while let Some(action) = self.actions.pop_front() {
            match action {
                Action::DealDamage(_) => {
                    update = Some(WorldUpdate {
                        target: WorldSim::id(),
                        action: Action::Destroy(self.id)
                    });
                },
                _ => {}
            }
        }

        update
    }

    fn queue(&mut self, action: &Action) {
        self.actions.push_back(action.clone());
    }
}