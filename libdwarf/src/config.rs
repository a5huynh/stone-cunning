use serde::Deserialize;
use std::collections::HashMap;

use crate::{
    entities::ResourceType,
};

#[derive(Clone, Debug, Deserialize)]
pub struct ResourceConfig {
    pub map: HashMap<String, ResourceType>
}

#[derive(Debug, Deserialize)]
pub struct WorldConfig {
    pub resources: ResourceConfig
}