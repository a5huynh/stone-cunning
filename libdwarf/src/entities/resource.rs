use serde::Deserialize;
///
/// Defines a generic resource type. These act as constants that will be used by
/// the actual objects to determine their behavior.
///
/// For example, a tree is generic resource since there will exist variants
/// that build upon the initial tree, e.g. an (Oak tree, Acacia, etc).
///
/// - Can be destructable or indestructible if no health attribute.
/// - ResourceTypes can drop other types when destroyed. For example, a tree should
///   drop wood.
///
#[derive(Clone, Debug, Deserialize)]
pub enum ResourceAttribute {
    Health(u32),
    Drops(String, u32),
}

impl ResourceAttribute {
    pub fn is_drop(&self) -> bool {
        match self {
            ResourceAttribute::Drops(_, _) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ResourceType {
    /// Name of this generic resource type
    pub name: String,
    /// Attributes
    pub attributes: Vec<ResourceAttribute>,
}

#[cfg(test)]
mod test {
    use super::{ResourceAttribute, ResourceType};
    use std::collections::HashMap;

    #[test]
    fn test_resource_creation() {
        let tree = ResourceType {
            name: String::from("tree"),
            attributes: vec![
                ResourceAttribute::Health(10),
                // Can have multiple drops
                ResourceAttribute::Drops(String::from("wood"), 3),
                ResourceAttribute::Drops(String::from("acorn"), 10),
            ],
        };

        assert_eq!(tree.name, String::from("tree"));
    }
}
