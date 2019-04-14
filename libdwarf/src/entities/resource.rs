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
#[derive(Debug, Deserialize)]
pub enum ResourceAttribute {
    Health(u32),
    Drops(String, u32),
}


#[derive(Debug, Deserialize)]
pub struct ResourceType {
    /// Name of this generic resource type
    name: String,
    /// Attributes
    attributes: Vec<ResourceAttribute>
}

#[cfg(test)]
mod test {
    use super::{ ResourceType, ResourceAttribute };

    #[test]
    fn test_resource_creation() {
        let tree = ResourceType {
            name: String::from("tree"),
            attributes: vec![
                ResourceAttribute::Health(10),
                ResourceAttribute::Drops(String::from("wood"), 10)
            ]
        };

        assert_eq!(tree.name, String::from("tree"));
    }
}