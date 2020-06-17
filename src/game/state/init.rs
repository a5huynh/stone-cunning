/// Loading/initialization state.
/// In this state, we're initializing the resources necessary to begin simulation.
/// This can either be starting the terrain generation or loading the necessary
/// terrain chunks from disk.
///
use core::{
    amethyst::{ecs::Write, prelude::*},
    Point3,
};
use libdwarf::{
    resources::{TaskQueue, World},
    trigger::TriggerType,
    world::WorldSim,
};
use libterrain::TerrainLoader;

use crate::game::{
    components::{Cursor, CursorSelected, Object, PassInfo, Player},
    config::GameConfig,
    resources::{MapRenderer, ViewShed},
    sprite::SpriteSheetStorage,
    state::RunningState,
};

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
        let mut world = data.world;
        world.register::<Cursor>();
        world.register::<Object>();
        world.register::<Player>();

        world.insert(PassInfo::default());

        let storage = SpriteSheetStorage::new(world);
        world.insert(storage);

        // Load the terrain for this game.
        let (chunk_height, chunk_width) = {
            let config = &world.read_resource::<GameConfig>();
            (config.chunk_height, config.chunk_width)
        };

        WorldSim::new(world);
        let terloader = TerrainLoader::new(chunk_width, chunk_height);
        let simworld = World::new(&mut world, terloader);
        world.insert(simworld);
        // Initialize simulation. Will also load visible chunks and prep
        // for rendering.

        // Utils for map rendering
        let map_render = MapRenderer::initialize(world);
        world.insert(map_render);

        // Initialize an empty viewshed, this will update one we get a camera
        world.insert(ViewShed::default());

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
