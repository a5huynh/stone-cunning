use specs::{Entities, ReadExpect, System, WriteExpect, WriteStorage};

use crate::{
    actions::Action,
    config::ResourceConfig,
    entities::{MapObject, MapPosition, Worker},
    resources::{Map, TaskQueue},
};

#[derive(Default)]
pub struct WorldUpdateSystem;
impl<'a> System<'a> for WorldUpdateSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Worker>,
        WriteStorage<'a, MapObject>,
        WriteStorage<'a, MapPosition>,
        WriteExpect<'a, TaskQueue>,
        WriteExpect<'a, Map>,
        ReadExpect<'a, ResourceConfig>,
    );

    fn run(
        &mut self,
        (entities, mut workers, mut objects, mut positions, mut tasks, mut map, resources): Self::SystemData,
    ) {
        let queue = &mut tasks.world;
        while let Some(event) = queue.pop_front() {
            match event {
                // Add an object to the map.
                Action::Add((x, y), name) => {
                    println!("Adding object '{}' @ ({}, {})", name, x, y);
                    let resource = resources.map.get(&name).unwrap().clone();
                    let new_entity = entities.create();
                    objects
                        .insert(new_entity, MapObject::new(&resource))
                        .unwrap();
                    positions.insert(new_entity, MapPosition { x, y }).unwrap();
                    map.collision_map.insert((x, y), new_entity.id());
                }
                Action::AddWorker((x, y)) => {
                    // println!("WUS: Adding worker @ {}, {}", x, y);
                    let entity = entities.create();
                    workers.insert(entity, Worker::new(x, y)).unwrap();
                    positions.insert(entity, MapPosition { x, y }).unwrap();
                }
                // Deal damage to a particular object
                Action::DealDamage(id, damage) => {
                    let entity = entities.entity(id);
                    if let Some(object) = objects.get_mut(entity) {
                        object.health -= damage;
                    }
                }
                // Destroy an object.
                Action::Destroy(id) => {
                    // Remove from map
                    let entity = entities.entity(id);
                    if let Some(_) = objects.get(entity) {
                        if let Some(pos) = positions.get(entity) {
                            map.collision_map.remove(&(pos.x, pos.y));
                        }
                    }
                    // Remove from world
                    entities.delete(entity).unwrap();
                }
                Action::Take { target, owner } => {
                    let target_entity = entities.entity(target);
                    if let Some(_) = objects.get(target_entity) {
                        if let Some(worker) = workers.get_mut(entities.entity(owner)) {
                            worker.inventory.push(target);
                            if let Some(pos) = positions.get(target_entity) {
                                map.collision_map.remove(&(pos.x, pos.y));
                                positions.remove(target_entity);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
