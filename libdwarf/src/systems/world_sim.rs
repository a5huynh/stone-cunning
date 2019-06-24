use specs::{Entities, ReadExpect, System, WriteExpect, WriteStorage};

use crate::{
    actions::Action,
    config::ResourceConfig,
    components::{MapObject, MapPosition, Worker},
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
                Action::Add(pt, name) => {
                    println!("[WUS] Adding object '{}' @ ({:?})", name, pt);
                    let resource = resources.map.get(&name).unwrap().clone();
                    let new_entity = entities.create();
                    objects
                        .insert(new_entity, MapObject::new(&resource))
                        .unwrap();
                    positions
                        .insert(new_entity, MapPosition { pos: pt })
                        .unwrap();
                    map.track_object(new_entity.id(), pt);
                }
                Action::AddWorker(pos) => {
                    println!("[WUS] Adding worker @ ({:?})", pos);
                    let entity = entities.create();
                    workers.insert(entity, Worker::new()).unwrap();
                    positions.insert(entity, MapPosition { pos }).unwrap();
                    map.track_worker(entity.id(), pos);
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
                        if let Some(map_pos) = positions.get(entity) {
                            map.remove_object(id, map_pos.pos);
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
                            if let Some(map_pos) = positions.get(target_entity) {
                                map.remove_object(target_entity.id(), map_pos.pos);
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
