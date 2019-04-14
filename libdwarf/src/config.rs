use serde::Deserialize;

use crate::{
    entities::ResourceType,
};

#[derive(Debug, Deserialize)]
pub struct WorldConfig {
    resources: Vec<ResourceType>
}