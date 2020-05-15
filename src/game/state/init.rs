use core::{
    amethyst::{ecs::Write, prelude::*},
    log::info,
    Point3,
};
/// Loading/initialization state.
/// In this state, we're initializing the resources necessary to begin simulation.
/// This can either be starting the terrain generation or loading the necessary
/// terrain chunks from disk.
///
use std::time::SystemTime;

use crate::game::{
    components::{Cursor, CursorSelected, Object, Player},
    config::GameConfig,
    resources::MapRenderer,
    sprite::SpriteSheetStorage,
    state::RunningState,
};
use libdwarf::{resources::TaskQueue, trigger::TriggerType, world::WorldSim};
use libterrain::TerrainLoader;

pub struct InitState {
    finished: bool,
}

impl Default for InitState {
    fn default() -> InitState {
        InitState { finished: false }
    }
}

impl SimpleState for InitState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<Cursor>();
        world.register::<Object>();
        world.register::<Player>();

        let storage = SpriteSheetStorage::new(world);
        world.insert(storage);

        // Initialize simulation;
        let (chunk_height, chunk_width) = {
            let config = &world.read_resource::<GameConfig>();
            (config.chunk_height, config.chunk_width)
        };

        // Initialize simulation
        info!("Generating map w/ dims: ({}, {})", chunk_width, chunk_height);
        let now = SystemTime::now();
        let terloader = TerrainLoader::new(chunk_width, chunk_height);
        let chunk = terloader.get_chunk(0, 0);
        info!("Terrain gen took: {}ms", now.elapsed().unwrap().as_millis());

        WorldSim::new(world, &chunk, chunk_width, chunk_height);

        // Render map
        let map_render = MapRenderer::initialize(world);
        world.insert(map_render);

        // Initialize cursor sprite.
        Cursor::initialize(world);

        // Initialize player.
        // Player::initialize(world);

        world.insert(CursorSelected::default());

        // Initialize workers
        world.exec(|mut queue: Write<'_, TaskQueue>| {
            queue.add_world(TriggerType::AddWorker(Point3::new(8, 8, 42)));
        });

        self.finished = true;
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if self.finished {
            return Trans::Switch(Box::new(RunningState::default()));
        }

        Trans::None
    }
}
