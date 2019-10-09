use specs::{Component, VecStorage};
use std::collections::VecDeque;

use crate::{
    actions::Action,
    components::{ResourceAttribute, ResourceType},
};

#[derive(Clone, Component, Debug)]
#[storage(VecStorage)]
pub struct MapObject {
    pub health: i32,
    pub resource_type: ResourceType,
    pub actions: VecDeque<Action>,
}

impl MapObject {
    /// Build a new map object positioned at (x, y).
    pub fn new(resource_type: &ResourceType) -> Self {
        let mut default_health = 1;
        for attribute in &resource_type.attributes {
            match attribute {
                ResourceAttribute::Health(health) => {
                    default_health = *health as i32;
                }
                _ => {}
            }
        }

        MapObject {
            resource_type: resource_type.clone(),
            actions: VecDeque::new(),
            health: default_health,
        }
    }

    pub fn drop_table(&self) -> Vec<&ResourceAttribute> {
        self.resource_type
            .attributes
            .iter()
            .filter(|x| x.is_drop())
            .collect()
    }

    pub fn to_string(&self) -> String {
        format!("{} ({})", self.resource_type.name, self.health)
    }
}
