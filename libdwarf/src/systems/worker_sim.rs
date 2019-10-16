use specs::{Entities, Join, ReadExpect, ReadStorage, System, Write, WriteExpect, WriteStorage};
use std::collections::VecDeque;

use libpath::find_path;
use libterrain::TerrainChunk;

use crate::{
    actions::ActionType,
    components::{MapObject, MapPosition, Worker},
    config::WorldConfig,
    resources::{time::Time, Map, TaskQueue},
};

pub struct WorkerSystem;
impl<'a> System<'a> for WorkerSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Worker>,
        ReadStorage<'a, MapObject>,
        WriteStorage<'a, MapPosition>,
        WriteExpect<'a, Map>,
        Write<'a, TaskQueue>,
        ReadExpect<'a, Time>,
        ReadExpect<'a, WorldConfig>,
    );

    fn run(
        &mut self,
        (entities, mut workers, objects, mut positions, mut map, mut tasks, time, config): Self::SystemData,
    ) {
        for (entity, worker, position) in (&*entities, &mut workers, &mut positions).join() {
            // Regen worker energy.
            worker.energy += config.worker_stamina * time.delta_seconds();
            if worker.energy < config.action_cost {
                continue;
            }

            // Handle actions for worker
            let mut new_queue = VecDeque::new();
            let current_pos = position.pos;
            while let Some(action) = worker.actions.pop_front() {
                match action {
                    ActionType::Chilling => {
                        worker.energy -= config.action_cost;
                    }
                    ActionType::Move(mut path) => {
                        if !path.is_empty() {
                            if let Some(pt) = path.pop() {
                                position.pos = pt;
                                worker.energy -= config.action_cost;
                                println!("Moving worker to {}", position.pos);
                                map.move_worker(entity.id(), position.pos, position.pos);
                            }
                        }
                    }
                    // Perform an action.
                    ActionType::HarvestResource(pos, target, harvest) => {
                        println!("Harvesting {:?} @ {}", target, pos);

                        // Are we next to this resource? Move closer to it if not.
                        let dist_x = (pos.x as i32 - current_pos.x as i32).abs() as u32;
                        let dist_y = (pos.y as i32 - current_pos.y as i32).abs() as u32;

                        if dist_x + dist_y <= 1 {
                            // Is the resource available nearby?
                            let neighbors = map.find_neighbors(current_pos);

                            let harvest_resource = neighbors.iter().find(|&&&neighbor| {
                                let entity = entities.entity(neighbor);
                                if let Some(object) = objects.get(entity) {
                                    object.resource_type.name == harvest
                                } else {
                                    false
                                }
                            });

                            let target_resource = neighbors.iter().find(|&&&neighbor| {
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
                                worker.energy -= config.action_cost;
                                tasks.add_world(ActionType::Take {
                                    target: **id,
                                    owner: entity.id(),
                                });
                            // Otherwise, try and harvest from a nearby target.
                            } else if let Some(id) = target_resource {
                                // Harvest by dealing damage to item.
                                worker.energy -= config.action_cost;
                                new_queue.push_back(ActionType::HarvestResource(
                                    pos,
                                    target.clone(),
                                    harvest.clone(),
                                ));
                                tasks.add_world(ActionType::DealDamage(**id, 10));
                            }
                        } else {
                            // Move closer
                            let (_, path) = find_path(
                                current_pos,
                                pos,
                                |node| TerrainChunk::heuristic(&node, &pos),
                                |pt| map.terrain.neighbors(pt),
                            );

                            new_queue.push_back(ActionType::Move(path));
                            new_queue.push_back(ActionType::HarvestResource(
                                pos,
                                target.clone(),
                                harvest.clone(),
                            ));
                        }
                    }
                    _ => {}
                }
            }

            worker.actions.append(&mut new_queue);
        }
    }
}
