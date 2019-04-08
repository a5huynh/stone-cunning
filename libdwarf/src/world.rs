use std::collections::{ HashMap, VecDeque };
use std::fmt;

use crate::{
    actors::{ Actor, Worker },
    objects::{ MapObject, ResourceType },
    tasks::{ Action }
};

#[derive(Clone, Debug)]
pub enum Terrain {
    STONE = 0,
    MARBLE = 1,
    GRASS = 2,
    NONE = -1,
}

pub struct WorldUpdate {
    pub sender: u32,
    pub receiver: u32,
    pub action: Action,
}

pub struct World {
    // TODO: Support multi-tile objects.
    pub width: u32,
    pub height: u32,
    pub tasks: VecDeque<Action>,
    pub workers: Vec<Worker>,
    // TODO: Support multiple objects per tile.
    pub objects: HashMap<(u32, u32), MapObject>,
    pub terrain: HashMap<(u32, u32), Terrain>,

    resource_map: HashMap<String, ResourceType>,
}

impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut cells = vec!['?'; (self.width * self.height) as usize];
        // Render terrain first
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                let idx = (x as u32, y as u32);
                let terrain = self.terrain.get(&idx);
                let tile = match terrain {
                    Some(Terrain::GRASS) => ',',
                    Some(Terrain::STONE) => '.',
                    Some(Terrain::MARBLE) => '.',
                    _ => '?',
                };

                cells[(y * self.width + x) as usize] = tile;
            }
        }

        // Add objects to cells
        for (pos, value) in self.objects.iter() {
            let idx = (pos.1 * self.width + pos.0) as usize;
            let tile = match value {
                MapObject { id: 1, .. } => 'T',
                _ => '?'
            };

            cells[idx] = tile;
        }

        // Add workers to cells
        for worker in self.workers.iter() {
            let idx = (worker.y * self.width + worker.x) as usize;
            cells[idx] = 'w';
        }

        // Output completed cells.
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                write!(f, "{}", cells[(y * self.width + x) as usize])?;
                if x < self.width - 1 {
                    write!(f, " ")?;
                }
            }
            write!(f, "\n\r")?;
        }

        Ok(())
    }
}

impl World {
    pub fn new(height: u32, width: u32) -> Self {
        let mut map_terrain = HashMap::new();
        // TODO: Actually generate terrain.
        for y in 0..height {
            for x in 0..width {
                let tile = ((x + y) % 3) as usize;
                let terrain = match tile {
                    0 => Terrain::STONE,
                    1 => Terrain::MARBLE,
                    2 => Terrain::GRASS,
                    _ => Terrain::NONE,
                };

                map_terrain.insert((x as u32, y as u32), terrain);
            }
        }

        World {
            height,
            width,
            tasks: VecDeque::new(),
            objects: HashMap::new(),
            terrain: map_terrain,
            workers: Vec::new(),
            resource_map: HashMap::new(),
        }
    }

    pub fn add_object(&mut self, x: u32, y: u32, object: MapObject) {
        self.objects.insert((x, y), object);
    }

    pub fn add_task(&mut self, task: Action) {
        self.tasks.push_back(task);
    }

    pub fn add_worker(&mut self, x: u32, y: u32) {
        self.workers.push(Worker {
            id: 0,
            actions: VecDeque::new(),
            x, y
        });
    }

    pub fn run_update(&mut self, update: &WorldUpdate) {
        // find receiver
        let object = self.objects
            .iter_mut()
            .find(|(_, object)| object.id == update.receiver);
        if let Some((_, receiver)) = object {
            receiver.queue(&update.action);
        }
    }

    pub fn tick(&mut self) {
        println!("World tick");
        let workers = &mut self.workers;
        let tasks = &mut self.tasks;
        // Handle assign any queued tasks to idle workers
        let mut updates = VecDeque::new();

        for worker in workers.iter_mut() {
            if worker.actions.is_empty() {
                if let Some(new_task) = tasks.pop_front() {
                    worker.queue(&new_task);
                }
            }

            if let Some(update) = worker.tick() {
                updates.push_back(update);
            };
        }

        // Process worker updates
        for update in updates.iter_mut() {
            println!("updating object");
            self.run_update(update);
        }

        for (_, object) in self.objects.iter_mut() {
            object.tick();
        }
    }
}