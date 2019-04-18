use ron::de::from_reader;
use serde::Deserialize;
use std::{
    collections::HashMap,
    fs::File
};

use crate::{
    entities::ResourceType,
};

#[derive(Clone, Debug, Deserialize)]
pub struct ResourceConfig {
    pub map: HashMap<String, ResourceType>
}

impl ResourceConfig {
    pub fn load(input_path: &str) -> Self {
        let f = File::open(input_path).expect("Failed opening file");
        let config: ResourceConfig = match from_reader(f) {
            Ok(x) => x,
            Err(e) => {
                println!("Failed to load config: {}", e);
                std::process::exit(1);
            }
        };
        config
    }
}

#[derive(Debug, Deserialize)]
pub struct WorldConfig {
    /// How fast workers regain energy
    pub action_cost: f32,
    pub worker_stamina: f32,
}

impl WorldConfig {
    pub fn load(input_path: &str) -> Self {
        let f = File::open(input_path).expect("Failed opening file");
        let config: WorldConfig = match from_reader(f) {
            Ok(x) => x,
            Err(e) => {
                println!("Failed to load config: {}", e);
                std::process::exit(1);
            }
        };
        config
    }
}