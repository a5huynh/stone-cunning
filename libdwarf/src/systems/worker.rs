use std::collections::VecDeque;
use specs::{
    Entities,
    Join,
    ReadExpect,
    ReadStorage,
    System,
    Write,
    WriteStorage,
};

use crate::{
    actions::Action,
    entities::{ MapObject, Worker },
    resources::{ Map, TaskQueue },
};

pub struct WorkerSystem;
impl<'a> System<'a> for WorkerSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Worker>,
        ReadStorage<'a, MapObject>,
        ReadExpect<'a, Map>,
        Write<'a, TaskQueue>,
    );

    fn run (&mut self, (
        entities,
        mut workers,
        objects,
        map,
        mut tasks,
    ): Self::SystemData) {
        for (entity, worker) in (&*entities, &mut workers).join() {
            let mut new_queue = VecDeque::new();
            // Handle actions for worker
            while let Some(action) = worker.actions.pop_front() {
                // println!("Worker({}) - Processing action {:?}", entity.id(), action);
                match action {
                    // Route worker towards a target
                    Action::MoveTo(target_x, target_y) => {
                        worker.x = target_x;
                        worker.y = target_y;
                    },
                    // Perform an action.
                    Action::HarvestResource(pos, target, harvest) => {
                        let (target_x, target_y) = pos;
                        // Are we next to this resource? Move closer to it
                        let dist_x = (target_x as i32 - worker.x as i32).abs() as u32;
                        let dist_y = (target_y as i32 - worker.y as i32).abs() as u32;

                        if dist_x + dist_y <= 1 {
                            // Is the resource available nearby?
                            let neighbors = map.find_neighbors(worker.x, worker.y);

                            let harvest_resource = neighbors.iter()
                                .find(|&&&neighbor| {
                                    let entity = entities.entity(neighbor);
                                    if let Some(object) = objects.get(entity) {
                                        object.resource_type.name == harvest
                                    } else {
                                        false
                                    }
                                });

                            let target_resource = neighbors.iter()
                                .find(|&&&neighbor| {
                                    let entity = entities.entity(neighbor);
                                    if let Some(object) = objects.get(entity) {
                                        object.resource_type.name == target
                                    } else {
                                        false
                                    }
                                });

                            // If the harvest resource is on the ground nearby,
                            // add it to inventory.
                            if let Some(id) = harvest_resource {
                                tasks.add_world(Action::Take { target: **id, owner: entity.id() });
                            // Otherwise, try and harvest from a nearby target.
                            } else if let Some(id) = target_resource {
                                // Harvest by dealing damage to item.
                                new_queue.push_back(Action::HarvestResource(pos, target.clone(), harvest.clone()));
                                tasks.add_world(Action::DealDamage(**id, 10));
                            }
                        } else {
                            // Move closer
                            let mut new_x = worker.x;
                            let mut new_y = worker.y;
                            if dist_x > dist_y {
                                new_x += 1;
                            } else {
                                new_y += 1;
                            }

                            new_queue.push_back(Action::MoveTo(new_x, new_y));
                            new_queue.push_back(Action::HarvestResource(pos, target.clone(), harvest.clone()));
                        }
                    },
                    _ => {}
                }
            }

            worker.actions.append(&mut new_queue);
        }
    }
}
