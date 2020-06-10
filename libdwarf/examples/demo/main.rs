use crossterm::{input, RawScreen};

use core::amethyst::ecs::{self, DispatcherBuilder, WorldExt};

mod renderer;
use self::renderer::AsciiRenderer;

const MAP_WIDTH: u32 = 10;
const MAP_HEIGHT: u32 = 10;

use core::Point3;
use libdwarf::{
    resources::{TaskQueue, World},
    systems,
    trigger::TriggerType,
    world::WorldSim,
};

use libterrain::{ChunkEntity, TerrainChunk, TerrainLoader};

fn main() {
    // Setup ascii renderer
    let _screen = RawScreen::into_raw_mode();
    let mut renderer = AsciiRenderer::new();

    let mut ecs = ecs::World::new();

    // Initialize the world.
    let mut terrain = TerrainLoader::new(MAP_WIDTH, MAP_HEIGHT);
    let chunk = TerrainChunk::new((0, 0), MAP_WIDTH, MAP_HEIGHT);
    terrain.chunks.insert((0, 0), chunk);

    let world = World::new(&mut ecs, terrain);
    ecs.insert(world);

    WorldSim::new(&mut ecs);

    let mut dispatcher = DispatcherBuilder::new()
        .with(systems::WorkerSystem, "worker_sim", &[])
        .with(systems::ObjectSystem, "object_sim", &[])
        .with(
            systems::WorldUpdateSystem::default(),
            "world_updates",
            &["worker_sim", "object_sim"],
        )
        .with(systems::TimeTickSystem, "game_tick", &["world_updates"])
        .build();

    dispatcher.setup(&mut ecs);
    // Add entities to the world
    ecs.exec(|(mut queue,): (ecs::Write<TaskQueue>,)| {
        queue.add_world(TriggerType::AddWorker(Point3::new(0, 0, 0)));
        queue.add_world(TriggerType::Add(Point3::new(9, 9, 0), String::from("tree")));
    });

    let input = input();
    loop {
        // Render map
        renderer.render(&ecs);

        match input.read_char().unwrap() {
            // Add a task to the task queue.
            'a' => {
                ecs.exec(
                    |(mut queue, mut world): (ecs::Write<TaskQueue>, ecs::WriteExpect<World>)| {
                        let entity = world.terrain.get(&Point3::new(9, 9, 0));

                        if let Some(ChunkEntity::Object(uuid, _)) = entity {
                            if let Some(target_id) = world.entity_map.get(&uuid) {
                                queue.add(TriggerType::HarvestResource {
                                    target: *target_id,
                                    position: Point3::new(9, 9, 0),
                                    resource: String::from("wood"),
                                });
                            }
                        }
                    },
                );
            }
            // quit
            'q' => return,
            // Tick map
            '.' => {
                renderer.num_ticks += 1;
                // Tick map
                dispatcher.dispatch(&mut ecs);
                ecs.maintain();
            }
            _ => {}
        }

        println!("");
    }
}
