use core::amethyst::{
    ecs::{self, Join},
    prelude::*,
};
use crossterm::{cursor, terminal, ClearType, Terminal, TerminalCursor};

use core::Point3;
use libdwarf::{
    components::{EntityInfo, MapObject, Worker},
    resources::World,
};

const MAP_WIDTH: i32 = 10;
const MAP_HEIGHT: i32 = 10;

pub struct AsciiRenderer {
    pub num_ticks: u16,
    cursor: TerminalCursor,
    terminal: Terminal,
}

impl AsciiRenderer {
    pub fn new() -> AsciiRenderer {
        AsciiRenderer {
            cursor: cursor(),
            terminal: terminal(),
            num_ticks: 0,
        }
    }

    // Add objects to cells
    pub fn render_objects(&mut self, ecs: &ecs::World, cells: &mut Vec<char>) {
        let entities = ecs.entities();
        let objects = ecs.read_storage::<MapObject>();
        let infos = ecs.read_storage::<EntityInfo>();
        for (_, object, info) in (&entities, &objects, &infos).join() {
            let idx = (info.pos.y * MAP_WIDTH + info.pos.x) as usize;
            let tile = match object.resource_type.name.as_ref() {
                "tree" => 'T',
                "wood" => 'l',
                _ => '?',
            };

            cells[idx] = tile;
        }
    }

    pub fn render_terrain(&mut self, ecs: &ecs::World, cells: &mut Vec<char>) {
        let mut world = ecs.write_resource::<World>();
        for y in (0..MAP_HEIGHT).rev() {
            for x in 0..MAP_WIDTH {
                let terrain = world.terrain.get(&Point3::new(x as i32, y as i32, 0));
                let tile = match terrain {
                    _ => '.',
                };

                cells[(y * MAP_WIDTH + x) as usize] = tile;
            }
        }
    }

    // Add workers to cells
    pub fn render_workers(&mut self, ecs: &ecs::World, cells: &mut Vec<char>) {
        let infos = ecs.read_storage::<EntityInfo>();
        let workers = ecs.read_storage::<Worker>();
        for (info, _) in (&infos, &workers).join() {
            let idx = (info.pos.y * MAP_WIDTH + info.pos.x) as usize;
            cells[idx] = 'w';
        }
    }

    pub fn render(&mut self, ecs: &ecs::World) {
        self.terminal.clear(ClearType::All).unwrap();
        self.cursor.goto(0, 0).unwrap();

        print!("num ticks: {}\n\r", self.num_ticks);

        // Render ecs.
        let mut cells = vec!['?'; (MAP_WIDTH * MAP_HEIGHT) as usize];
        self.render_terrain(ecs, &mut cells);
        self.render_objects(ecs, &mut cells);
        self.render_workers(ecs, &mut cells);
        for y in (0..MAP_HEIGHT).rev() {
            for x in 0..MAP_HEIGHT {
                print!("{}", cells[(y * MAP_WIDTH + x) as usize]);
                if x < MAP_WIDTH - 1 {
                    print!(" ");
                }
            }
            print!("\n\r");
        }

        // Render workers & worker status
        self.cursor.goto(1, 12).unwrap();

        print!("\n\rWorkers\n\r--------------\n\r");
        let entities = ecs.entities();
        let workers = ecs.read_storage::<Worker>();
        for (entity, worker) in (&entities, &workers).join() {
            print!("[W{}: Current Action]\n\r", entity.id());
            print!("- {:?}\n\r", worker.current_action);

            print!("[W{}: Current Path]\n\r", entity.id());
            print!("- {:?}\n\r", worker.current_path);

            print!("[W{}: Inventory]\n\r", entity.id());
            for obj in worker.inventory.iter() {
                print!("- {:?}\n\r", obj.to_string());
            }

            print!("[W{}: Task Queue]\n\r", entity.id());
            for action in worker.queue.iter() {
                print!("- {:?}\n\r", action);
            }
        }

        // Render objects & object queue
        print!("\n\rWorld Objects\n\r--------------\n\r");
        let objects = ecs.read_storage::<MapObject>();
        let infos = ecs.read_storage::<EntityInfo>();
        for (entity, object, info) in (&entities, &objects, &infos).join() {
            print!(
                "{} [pos: {:?} id: {}]\n\r",
                object.to_string(),
                (info.pos.x, info.pos.y),
                entity.id()
            );
        }

        // Render command prompt
        print!("\n\r");
    }
}
