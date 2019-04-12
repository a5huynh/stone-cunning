use std::collections::{ HashMap, VecDeque };

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

#[derive(Clone)]
pub struct WorldSim {
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

fn find_neighbors<'a>(map: &'a mut HashMap<(u32, u32), MapObject>, x: u32, y: u32) -> Vec<Option<&'a MapObject>> {
    // Generate the coordinates for the neighbors
    let mut neighbor_idx = Vec::with_capacity(4);
    neighbor_idx.push((x, y + 1));
    neighbor_idx.push((x + 1, y));
    if y > 0 {
        neighbor_idx.push((x, y - 1));
    }
    if x > 0 {
        neighbor_idx.push((x - 1, y));
    }

    // Find the neighbors and return the results
    let mut results = Vec::new();
    for idx in neighbor_idx.iter() {
        results.push(map.get(idx));
    }
    results
}

impl WorldSim {
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

        WorldSim {
            height,
            width,
            collision_map: HashMap::new(),
            objects: HashMap::new(),
            tasks: VecDeque::new(),
            terrain: map_terrain,
            workers: Vec::new(),
        }
    }

    pub fn has_collision(&self, x: u32, y: u32) -> bool {
        self.collision_map.contains_key(&(x, y))
    }

    /// Add object to map
    pub fn add_object(&mut self, object: MapObject) {
        self.objects.insert(object.id, object.clone());
        self.collision_map.insert((object.x, object.y), object);
    }

    /// Remove object from map
    pub fn remove_object(&mut self, oid: u32) -> Option<MapObject> {
        let result = self.objects.remove(&oid);
        if let Some(object) = result {
            self.collision_map.remove(&(object.x, object.y));
            return Some(object);
        }

        return None;
    }

    pub fn objects_at(&self, x: u32, y: u32) -> Option<&MapObject> {
        self.collision_map.get(&(x, y))
    }

    pub fn add_task(&mut self, task: Action) {
        self.tasks.push_back(task);
    }

    pub fn add_worker(&mut self, id: u32, x: u32, y: u32) {
        self.workers.push(Worker {
            id, x, y,
            actions: VecDeque::new(),
        });
    }

    /// Get a reference to the worker sim
    pub fn get_worker(&self, id: u32) -> Option<&Worker> {
        for worker in self.workers.iter() {
            if worker.id == id {
                return Some(worker);
            }
        }

        return None;
    }

    pub fn run_update(&mut self, update: &WorldUpdate) {
        let objects = &mut self.objects;

        // Route action to the correct place.
        if update.target == WorldSim::id() {
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
        let map = &mut self.collision_map;
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
                let neighbors = find_neighbors(map, worker.x, worker.y);
                if let Some(update) = worker.tick(neighbors) {
                    updates.push(update);
                };
            }
        }

        for (_, object) in objects.iter_mut() {
            if !object.actions.is_empty() {
                let neighbors = find_neighbors(map, object.x, object.y);
                if let Some(update) = object.tick(neighbors) {
                    updates.push(update);
                }
            }
        }

        for update in updates.iter_mut() {
            self.run_update(update);
        }
    }
}