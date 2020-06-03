use core::amethyst::ecs::{World, WorldExt};

use crate::{
    components::{EntityInfo, MapObject, Worker},
    config::{ResourceConfig, WorldConfig},
    planner::Planner,
    resources::{time, TaskQueue},
};

#[derive(Default)]
pub struct WorldSim;
impl WorldSim {
    pub fn new(world: &mut World) -> Self {
        world.register::<EntityInfo>();
        world.register::<MapObject>();
        world.register::<Worker>();

        // Initialize planner
        let planner = Planner::load("./resources/data/actions.ron");
        world.insert(planner);

        // Load resource configs
        let resources = ResourceConfig::load("./resources/data/resources.ron");
        world.insert(resources);

        // Load sim config
        let world_config = WorldConfig::load("./resources/sim_config.ron");
        world.insert(world_config);

        // Initialize task queue.
        world.insert(TaskQueue::default());
        // Add time tracking resources
        world.insert(time::Time::default());
        world.insert(time::Stopwatch::default());

        Default::default()
    }
}
