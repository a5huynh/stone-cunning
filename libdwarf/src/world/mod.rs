use specs::prelude::*;
use specs::World;

use crate::{
    components::{MapObject, MapPosition, Worker},
    config::{ResourceConfig, WorldConfig},
    planner::Planner,
    resources::{time, Map, TaskQueue},
};

use libterrain::TerrainChunk;

#[derive(Default)]
pub struct WorldSim;
impl WorldSim {
    pub fn new(world: &mut World, terrain: &TerrainChunk, width: u32, height: u32) -> Self {
        world.register::<MapPosition>();
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

        // Initialize map.
        let map = Map::initialize(world, terrain, width, height);
        world.insert(map);

        // Initialize task queue.
        world.insert(TaskQueue::default());
        // Add time tracking resources
        world.insert(time::Time::default());
        world.insert(time::Stopwatch::default());

        Default::default()
    }
}
