use crossterm::{input, RawScreen};

use core::amethyst::ecs::{self, DispatcherBuilder, World, WorldExt};

mod renderer;
use self::renderer::AsciiRenderer;

const MAP_WIDTH: u32 = 10;
const MAP_HEIGHT: u32 = 10;

use core::Point3;
use libdwarf::{
    resources::TaskQueue,
    systems,
    trigger::TriggerType,
    world::WorldSim,
};

use libterrain::{TerrainChunk, TerrainLoader};

fn main() {
    // Setup ascii renderer
    let _screen = RawScreen::into_raw_mode();
    let mut renderer = AsciiRenderer::new();

    let mut world = World::new();

    // Initialize the world.
    let mut terrain = TerrainLoader::new(MAP_WIDTH, MAP_HEIGHT);
    let chunk = TerrainChunk::new(MAP_WIDTH, MAP_HEIGHT);
    terrain.chunks.insert((0, 0), chunk);
    world.insert(terrain);
    WorldSim::new(&mut world);

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

    dispatcher.setup(&mut world);
    // Add entities to the world
    world.exec(|(mut queue,): (ecs::Write<TaskQueue>,)| {
        queue.add_world(TriggerType::AddWorker(Point3::new(0, 0, 0)));
        queue.add_world(TriggerType::Add(Point3::new(9, 9, 0), String::from("tree")));
    });

    let input = input();
    loop {
        // Render map
        renderer.render(&world);

        match input.read_char().unwrap() {
            // Add a task to the task queue.
            'a' => {
                world.exec(
                    |(mut queue, map): (ecs::Write<TaskQueue>, ecs::WriteExpect<TerrainLoader>)| {
                        let entity_id = map.object_map.get(&Point3::new(9, 9, 0)).unwrap();
                        queue.add(TriggerType::HarvestResource {
                            target: *entity_id,
                            position: Point3::new(9, 9, 0),
                            resource: String::from("wood"),
                        });
                    },
                );
            }
            // quit
            'q' => return,
            // Tick map
            '.' => {
                renderer.num_ticks += 1;
                // Tick map
                dispatcher.dispatch(&mut world);
                world.maintain();
            }
            _ => {}
        }

        println!("");
    }
}
