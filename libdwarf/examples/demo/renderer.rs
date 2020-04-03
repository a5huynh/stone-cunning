use core::amethyst::{ecs::Join, prelude::*};
use crossterm::{cursor, terminal, ClearType, Terminal, TerminalCursor};

use core::Point3;
use libdwarf::{
    components::{EntityInfo, MapObject, Worker},
    resources::Map,
};

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
    pub fn render_objects(&mut self, world: &World, cells: &mut Vec<char>) {
        let map = world.read_resource::<Map>();
        let entities = world.entities();
        let objects = world.read_storage::<MapObject>();
        let infos = world.read_storage::<EntityInfo>();
        for (_, object, info) in (&entities, &objects, &infos).join() {
            let idx = (info.pos.y * map.width + info.pos.x) as usize;
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
                let terrain = map.terrain_at(Point3::new(x as i32, y as i32, 0));
                let tile = match terrain {
                    _ => '.',
                };

                cells[(y * map.width + x) as usize] = tile;
            }
        }
    }

    // Add workers to cells
    pub fn render_workers(&mut self, world: &World, cells: &mut Vec<char>) {
        let map = world.read_resource::<Map>();
        let infos = world.read_storage::<EntityInfo>();
        let workers = world.read_storage::<Worker>();
        for (info, _) in (&infos, &workers).join() {
            let idx = (info.pos.y * map.width + info.pos.x) as usize;
            cells[idx] = 'w';
        }
    }

    pub fn render(&mut self, world: &World) {
        self.terminal.clear(ClearType::All).unwrap();
        self.cursor.goto(0, 0).unwrap();

        print!("num ticks: {}\n\r", self.num_ticks);

        // Render world.
        let map = world.read_resource::<Map>();
        let mut cells = vec!['?'; (map.width * map.height) as usize];
        self.render_terrain(world, &mut cells);
        self.render_objects(world, &mut cells);
        self.render_workers(world, &mut cells);
        for y in (0..map.height).rev() {
            for x in 0..map.width {
                print!("{}", cells[(y * map.width + x) as usize]);
                if x < map.width - 1 {
                    print!(" ");
                }
            }
            print!("\n\r");
        }

        // Render workers & worker status
        self.cursor.goto(1, 12).unwrap();

        print!("\n\rWorkers\n\r--------------\n\r");
        let entities = world.entities();
        let workers = world.read_storage::<Worker>();
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
        let objects = world.read_storage::<MapObject>();
        let infos = world.read_storage::<EntityInfo>();
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
