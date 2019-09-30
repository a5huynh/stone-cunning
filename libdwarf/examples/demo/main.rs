use crossterm::{input, RawScreen};

use specs::{prelude::*};

use libterrain::Point3;

mod renderer;
use self::renderer::AsciiRenderer;

use libdwarf::{
    actions::Action,
    resources::TaskQueue,
    systems,
    world::WorldSim,
};

fn main() {
    // Setup ascii renderer
    let screen = RawScreen::into_raw_mode();
    let mut renderer = AsciiRenderer::new();

    let mut world = World::new();
    WorldSim::new(&mut world, 10, 10);

    let mut dispatcher = DispatcherBuilder::new()
        .with(systems::AssignTaskSystem, "assign_task", &[])
        .with(systems::WorkerSystem, "worker_sim", &["assign_task"])
        .with(systems::ObjectSystem, "object_sim", &[])
        .with(
            systems::WorldUpdateSystem::default(),
            "world_updates",
            &["worker_sim", "object_sim"],
        )
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
