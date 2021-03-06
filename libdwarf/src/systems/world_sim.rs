use core::{
    amethyst::ecs::{Entities, ReadExpect, System, WriteExpect, WriteStorage},
    log,
};

use crate::{
    components::{EntityInfo, MapObject, Worker},
    config::ResourceConfig,
    resources::{Map, TaskQueue},
    trigger::TriggerType,
};

#[derive(Default)]
pub struct WorldUpdateSystem;
impl<'a> System<'a> for WorldUpdateSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Worker>,
        WriteStorage<'a, MapObject>,
        WriteStorage<'a, EntityInfo>,
        WriteExpect<'a, TaskQueue>,
        WriteExpect<'a, Map>,
        ReadExpect<'a, ResourceConfig>,
    );

    fn run(
        &mut self,
        (entities, mut workers, mut objects, mut entity_infos, mut tasks, mut map, resources): Self::SystemData,
    ) {
        let queue = &mut tasks.world;
        while let Some(event) = queue.pop_front() {
            match event {
                // Add an object to the map.
                TriggerType::Add(pt, name) => {
                    log::info!("Adding object '{}' @ ({:?})", name, pt);
                    let resource = resources.map.get(&name).unwrap().clone();
                    let new_entity = entities.create();
                    objects
                        .insert(new_entity, MapObject::new(&resource))
                        .unwrap();
                    entity_infos
                        .insert(
                            new_entity,
                            EntityInfo {
                                pos: pt,
                                z_offset: 1.0,
                            },
                        )
                        .unwrap();
                    map.track_object(new_entity.id(), pt);
                }
                TriggerType::AddWorker(pos) => {
                    log::info!("Adding worker @ ({:?})", pos);
                    let entity = entities.create();
                    workers.insert(entity, Worker::new(entity.id())).unwrap();
                    entity_infos
                        .insert(entity, EntityInfo { pos, z_offset: 1.0 })
                        .unwrap();
                    map.track_worker(entity.id(), pos);
                }
                // Deal damage to a particular object
                TriggerType::DealDamage {
                    source: _,
                    target,
                    damage,
                } => {
                    let entity = entities.entity(target);
                    if let Some(object) = objects.get_mut(entity) {
                        object.health -= damage;
                    }
                }
                // Destroy an object.
                TriggerType::Destroy(id) => {
                    // Remove from map
                    let entity = entities.entity(id);
                    if objects.get(entity).is_some() {
                        if let Some(entity_info) = entity_infos.get(entity) {
                            map.remove_object(id, entity_info.pos);
                        }
                    }
                    // Remove from world
                    entities.delete(entity).unwrap();
                }
                TriggerType::Take { target, owner } => {
                    let target_entity = entities.entity(target);
                    if objects.get(target_entity).is_some() {
                        if let Some(worker) = workers.get_mut(entities.entity(owner)) {
                            worker.inventory.push(target);
                            if let Some(entity_info) = entity_infos.get(target_entity) {
                                map.remove_object(target_entity.id(), entity_info.pos);
                                entity_infos.remove(target_entity);
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
