use core::amethyst::ecs::{Component, VecStorage};
use std::fmt;

use crate::components::{ResourceAttribute, ResourceType};

#[derive(Clone, Component, Debug)]
#[storage(VecStorage)]
pub struct MapObject {
    pub health: i32,
    pub resource_type: ResourceType,
}

impl MapObject {
    /// Build a new map object positioned at (x, y).
    pub fn new(resource_type: &ResourceType) -> Self {
        let mut default_health = 1;
        for attribute in &resource_type.attributes {
            if let ResourceAttribute::Health(health) = attribute {
                default_health = *health as i32;
            }
        }

        MapObject {
            resource_type: resource_type.clone(),
            health: default_health,
        }
    }

    pub fn is_destroyed(&self) -> bool {
        self.health <= 0
    }

    pub fn drop_table(&self) -> Vec<&ResourceAttribute> {
        self.resource_type
            .attributes
            .iter()
            .filter(|x| x.is_drop())
            .collect()
    }
}

impl fmt::Display for MapObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.resource_type.name, self.health)
    }
}
