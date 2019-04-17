use ron::de::from_reader;
use specs::{ World };
use std::fs::File;

use crate::{
    config::WorldConfig,
    entities::{
        MapObject,
        Worker,
    },
    resources::{
        Map,
        TaskQueue,
        time,
    }
};

#[derive(Default)]
pub struct WorldSim;
impl WorldSim {
    pub fn new(world: &mut World, width: u32, height: u32) -> Self {
        world.register::<MapObject>();
        world.register::<Worker>();
        // Initialize map.
        world.add_resource(Map::initialize(width, height));
        // Initialize task queue.
        world.add_resource(TaskQueue::default());
        // Add time tracking resources
        world.add_resource(time::Time::default());
        world.add_resource(time::Stopwatch::default());

        let input_path = format!("./resources/data/resources.ron");
        let f = File::open(&input_path).expect("Failed opening file");
        let config: WorldConfig = match from_reader(f) {
            Ok(x) => x,
            Err(e) => {
                println!("Failed to load config: {}", e);
                std::process::exit(1);
            }
        };
        world.add_resource(config.resources.clone());

        Default::default()
    }
}