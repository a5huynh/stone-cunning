use crossterm::{input, RawScreen};

use specs::prelude::*;

mod renderer;
use self::renderer::AsciiRenderer;

const MAP_WIDTH: u32 = 10;
const MAP_HEIGHT: u32 = 10;

use libdwarf::{actions::Action, resources::TaskQueue, systems, world::WorldSim};
use libterrain::{Point3, TerrainChunk};

fn main() {
    // Setup ascii renderer
    let _screen = RawScreen::into_raw_mode();
    let mut renderer = AsciiRenderer::new();

    let mut world = World::new();

    // Initialize the world.
    let terrain = TerrainChunk::new(MAP_WIDTH, MAP_HEIGHT);
    WorldSim::new(&mut world, &terrain, MAP_WIDTH, MAP_HEIGHT);

    let mut dispatcher = DispatcherBuilder::new()
        .with(systems::AssignTaskSystem, "assign_task", &[])
        .with(systems::WorkerSystem, "worker_sim", &["assign_task"])
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
    world.exec(|(mut queue,): (specs::Write<TaskQueue>,)| {
        queue.add_world(Action::AddWorker(Point3::new(0, 0, 0)));
        queue.add_world(Action::Add(Point3::new(9, 9, 0), String::from("tree")));
    });
    // Add a task to the task queue.
    world.exec(|(mut queue,): (specs::Write<TaskQueue>,)| {
        queue.add(Action::HarvestResource(
            Point3::new(9, 9, 0),
            String::from("tree"),
            String::from("wood"),
        ));
    });

    let input = input();
    loop {
        // Render map
        renderer.render(&world);

        match input.read_char().unwrap() {
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
