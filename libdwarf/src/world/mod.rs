use specs::World;

use crate::{
    config::{ResourceConfig, WorldConfig},
    entities::{MapObject, Worker},
    resources::{time, Map, TaskQueue},
};

#[derive(Default)]
pub struct WorldSim;
impl WorldSim {
    pub fn new(world: &mut World, width: u32, height: u32) -> Self {
        world.register::<MapObject>();
        world.register::<Worker>();

        // Load resource configs
        let resources = ResourceConfig::load("./resources/data/resources.ron");
        world.add_resource(resources);

        // Load sim config
        let world_config = WorldConfig::load("./resources/sim_config.ron");
        world.add_resource(world_config);

        // Initialize map.
        let map = Map::initialize(world, width, height);
        world.add_resource(map);

        // Initialize task queue.
        world.add_resource(TaskQueue::default());
        // Add time tracking resources
        world.add_resource(time::Time::default());
        world.add_resource(time::Stopwatch::default());

        Default::default()
    }
}
