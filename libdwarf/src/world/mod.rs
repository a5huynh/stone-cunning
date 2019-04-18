use specs::{ World };

use crate::{
    config::{ ResourceConfig, WorldConfig },
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

        let resources = ResourceConfig::load("./resources/data/resources.ron");
        world.add_resource(resources);

        let world_config = WorldConfig::load("./resources/sim_config.ron");
        world.add_resource(world_config);

        Default::default()
    }
}