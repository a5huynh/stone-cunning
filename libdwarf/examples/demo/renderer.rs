use crossterm::{cursor, terminal, ClearType, Terminal, TerminalCursor};
use specs::{prelude::*, Join};


use libdwarf::{
    components::{MapObject, MapPosition, Worker},
    resources::Map,
};
use libterrain::Point3;

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
        let positions = world.read_storage::<MapPosition>();
        for (_, object, pos) in (&entities, &objects, &positions).join() {
            let idx = (pos.pos.y * map.width + pos.pos.x) as usize;
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
        let positions = world.read_storage::<MapPosition>();
        let workers = world.read_storage::<Worker>();
        for (pos, _) in (&positions, &workers).join() {
            let idx = (pos.pos.y * map.width + pos.pos.x) as usize;
            cells[idx] = 'w';
        }
    }

    pub fn render(&mut self, world: &World) {

        self.terminal.clear(ClearType::All);
        self.cursor.goto(0, 0);

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
        self.cursor.goto(1, 12);

        print!("\n\rWorkers\n\r--------------\n\r");
        let entities = world.entities();
        let workers = world.read_storage::<Worker>();
        for (_, worker) in (&entities, &workers).join() {
            print!("[Inventory]\n\r");
            for obj in worker.inventory.iter() {
                print!("- {:?}\n\r", obj);
            }

            print!("[Task Queue]\n\r");
            for action in worker.actions.iter() {
                print!("- {:?}\n\r", action);
            }
        }

        // Render objects & object queue
        print!("\n\rObjects\n\r--------------\n\r");
        let objects = world.read_storage::<MapObject>();
        let positions = world.read_storage::<MapPosition>();
        for (entity, object, pos) in (&entities, &objects, &positions).join() {
            print!("{:?} {}\n\r", (pos.pos.x, pos.pos.y), entity.id());
            for action in object.actions.iter() {
                print!("- {:?}\n\r", action);
            }
        }

        // Render command prompt
        print!("\n\r");
    }
}