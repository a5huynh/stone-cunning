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
    resources::{CameraWindow, MapRenderer},
    sprite::SpriteSheetStorage,
    state::RunningState,
};
use libdwarf::{resources::TaskQueue, trigger::TriggerType, world::WorldSim};
use libterrain::TerrainGenerator;

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
        let (map_height, map_width) = {
            let config = &world.read_resource::<GameConfig>();
            (config.map_height, config.map_width)
        };

        // Initialize simulation
        info!("Generating map w/ dims: ({}, {})", map_width, map_height);
        let now = SystemTime::now();
        let terrain_gen = TerrainGenerator::new(map_width, map_height).build();
        info!("Terrain gen took: {}ms", now.elapsed().unwrap().as_millis());

        WorldSim::new(world, &terrain_gen.get_terrain(), map_width, map_height);

        // Render map
        world.insert(CameraWindow::default());
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
