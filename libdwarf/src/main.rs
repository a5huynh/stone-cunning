use specs::{self, prelude::*, Join, World};
use std::io::{self, Read, Write};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{clear, cursor, style};

use libdwarf::{
    actions::Action,
    entities::{MapObject, MapPosition, Worker},
    resources::{Map, TaskQueue, Terrain},
    systems,
    world::WorldSim,
};

struct AsciiRenderer<R, W: Write> {
    stdout: W,
    stdin: R,
    num_ticks: u16,
}

impl<R: Read, W: Write> AsciiRenderer<R, W> {
    pub fn new(stdin: R, stdout: W) -> AsciiRenderer<R, RawTerminal<W>> {
        AsciiRenderer {
            stdout: stdout.into_raw_mode().unwrap(),
            stdin: stdin,
            num_ticks: 0,
        }
    }

    // Add objects to cells
    pub fn render_objects(&mut self, world: &World, cells: &mut Vec<char>) {
        let map = world.read_resource::<Map>();
        let entities = world.entities();
        let objects = world.read_storage::<MapObject>();
        let positions = world.read_storage::<MapPosition>();
        for (_, object, pos) in (&entities, &objects, &positions).join() {
            let idx = (pos.y * map.width + pos.x) as usize;
            let tile = match object.resource_type.name.as_ref() {
                "tree" => 'T',
                "wood" => 'l',
                _ => '?',
            };

            cells[idx] = tile;
        }
    }

    pub fn render_terrain(&mut self, world: &World, cells: &mut Vec<char>) {
        let map = world.read_resource::<Map>();
        for y in (0..map.height).rev() {
            for x in 0..map.width {
                let idx = (x as u32, y as u32);
                let terrain = map.terrain.get(&idx);
                let tile = match terrain {
                    Some(Terrain::GRASS) => ',',
                    Some(Terrain::STONE) => '.',
                    Some(Terrain::MARBLE) => '.',
                    _ => '?',
                };

                cells[(y * map.width + x) as usize] = tile;
            }
        }
    }

    // Add workers to cells
    pub fn render_workers(&mut self, world: &World, cells: &mut Vec<char>) {
        let map = world.read_resource::<Map>();
        let entities = world.entities();
        let workers = world.read_storage::<Worker>();
        for (_, worker) in (&entities, &workers).join() {
            let idx = (worker.y * map.width + worker.x) as usize;
            cells[idx] = 'w';
        }
    }

    pub fn render(&mut self, world: &World) {
        write!(self.stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();
        write!(self.stdout, "num ticks: {}\n\r", self.num_ticks).unwrap();

        // Render world.
        let map = world.read_resource::<Map>();
        let mut cells = vec!['?'; (map.width * map.height) as usize];
        self.render_terrain(world, &mut cells);
        self.render_objects(world, &mut cells);
        self.render_workers(world, &mut cells);
        for y in (0..map.height).rev() {
            for x in 0..map.width {
                write!(self.stdout, "{}", cells[(y * map.width + x) as usize]).unwrap();
                if x < map.width - 1 {
                    write!(self.stdout, " ").unwrap();
                }
            }
            write!(self.stdout, "\n\r").unwrap();
        }

        // Render workers & worker status
        write!(self.stdout, "{}", cursor::Goto(1, 12)).unwrap();
        write!(self.stdout, "\n\rWorkers\n\r--------------\n\r").unwrap();
        let entities = world.entities();
        let workers = world.read_storage::<Worker>();
        for (_, worker) in (&entities, &workers).join() {
            write!(self.stdout, "[Inventory]\n\r").unwrap();
            for obj in worker.inventory.iter() {
                write!(self.stdout, "- {:?}\n\r", obj).unwrap();
            }

            write!(self.stdout, "[Task Queue]\n\r").unwrap();
            for action in worker.actions.iter() {
                write!(self.stdout, "- {:?}\n\r", action).unwrap();
            }
        }

        // Render objects & object queue
        write!(self.stdout, "\n\rObjects\n\r--------------\n\r").unwrap();
        let objects = world.read_storage::<MapObject>();
        let positions = world.read_storage::<MapPosition>();
        for (entity, object, pos) in (&entities, &objects, &positions).join() {
            write!(self.stdout, "{:?} {}\n\r", (pos.x, pos.y), entity.id()).unwrap();
            for action in object.actions.iter() {
                write!(self.stdout, "- {:?}\n\r", action).unwrap();
            }
        }

        // Render command prompt
        write!(self.stdout, "\n\r").unwrap();
        self.stdout.flush().unwrap();
    }
}

impl<R, W: Write> Drop for AsciiRenderer<R, W> {
    fn drop(&mut self) {
        write!(
            self.stdout,
            "{}{}{}",
            clear::All,
            style::Reset,
            cursor::Goto(1, 1)
        )
        .unwrap();
    }
}

fn main() {
    // Setup ascii renderer
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut renderer = AsciiRenderer::new(stdin.lock(), stdout.lock());

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
    dispatcher.setup(&mut world.res);
    // Add entities to the world
    world.exec(|(mut queue,): (specs::Write<TaskQueue>,)| {
        queue.add_world(Action::AddWorker((0, 0)));
        queue.add_world(Action::Add((9, 9), String::from("tree")));
    });
    // Add a task to the task queue.
    world.exec(|(mut queue,): (specs::Write<TaskQueue>,)| {
        queue.add(Action::HarvestResource(
            (9, 9),
            String::from("tree"),
            String::from("wood"),
        ));
    });

    loop {
        // Render map
        renderer.render(&world);
        // Get input and handle action
        let mut b = [0];
        renderer.stdin.read(&mut b).unwrap();
        match b[0] {
            // quit
            b'q' => return,
            // Tick map
            b'.' => {
                renderer.num_ticks += 1;
                // Tick map
                dispatcher.dispatch(&mut world.res);
                world.maintain();
            }
            _ => {}
        }

        renderer.stdout.flush().unwrap();
    }
}
