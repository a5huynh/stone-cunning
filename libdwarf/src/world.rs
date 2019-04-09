use std::collections::{ HashMap, VecDeque };
use std::fmt;

use crate::{
    actors::{ Actor, Worker },
    objects::{ MapObject },
    tasks::{ Action }
};

#[derive(Clone, Debug)]
pub enum Terrain {
    STONE = 0,
    MARBLE = 1,
    GRASS = 2,
    NONE = -1,
}

pub struct World {
    // TODO: Support multi-tile objects.
    pub width: u32,
    pub height: u32,
    pub tasks: VecDeque<Action>,
    pub workers: Vec<Worker>,
    pub objects: HashMap<u32, MapObject>,
    // TODO: Support multiple objects per tile.
    pub collision_map: HashMap<(u32, u32), MapObject>,
    pub terrain: HashMap<(u32, u32), Terrain>,
}

pub struct WorldUpdate {
    pub target: u32,
    pub action: Action
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
        for (_, object) in self.objects.iter() {
            let idx = (object.y * self.width + object.x) as usize;
            let tile = match object {
                MapObject { id: 1, .. } => 'T',
                MapObject { id: 10, .. } => 'w',
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
    pub fn id() -> u32 { 0 }

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
            collision_map: HashMap::new(),
            objects: HashMap::new(),
            tasks: VecDeque::new(),
            terrain: map_terrain,
            workers: Vec::new(),
        }
    }

    pub fn add_object(&mut self, object: MapObject) {
        self.objects.insert(object.id, object.clone());
        self.collision_map.insert((object.x, object.y), object);
    }

    pub fn remove_object(&mut self, oid: u32) -> Option<MapObject> {
        let result = self.objects.remove(&oid);
        if let Some(object) = result {
            self.collision_map.remove(&(object.x, object.y));
            return Some(object);
        }

        return None;
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
        let objects = &mut self.objects;

        // Route action to the correct place.
        if update.target == World::id() {
            // Update the world.
            match update.action {
                // Destroy an object.
                Action::Destroy(object_id) => {
                    // Remove object from entities and remove from collision map.
                    let object = self.remove_object(object_id);
                    // Add any materials from this object to map
                    if let Some(object) = object {
                        self.add_object(MapObject::new(10, object.x, object.y));
                    }
                },
                _ => {}
            }
        } else {
            let object = objects.get_mut(&update.target);
            if let Some(receiver) = object {
                receiver.queue(&update.action);
            }
        }
    }

    pub fn tick(&mut self) {
        let objects = &mut self.objects;
        let workers = &mut self.workers;
        let tasks = &mut self.tasks;

        // Assign tasks to idle workers
        for worker in workers.iter_mut() {
            if worker.actions.is_empty() {
                if let Some(new_task) = tasks.pop_front() {
                    worker.queue(&new_task);
                }
            }
        }

        let mut updates = Vec::new();
        for worker in workers.iter_mut() {
            if !worker.actions.is_empty() {
                if let Some(update) = worker.tick() {
                    updates.push(update);
                };
            }
        }

        for (_, object) in objects.iter_mut() {
            if !object.actions.is_empty() {
                if let Some(update) = object.tick() {
                    updates.push(update);
                }
            }
        }

        for update in updates.iter_mut() {
            self.run_update(update);
        }
    }
}