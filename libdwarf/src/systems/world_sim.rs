use core::{
    amethyst::ecs::{Entities, ReadExpect, System, WriteExpect, WriteStorage},
    log, Uuid,
};

use crate::{
    components::{EntityInfo, MapObject, Worker},
    config::ResourceConfig,
    resources::{TaskQueue, World},
    trigger::TriggerType,
};

use libterrain::{ChunkEntity, ObjectType};

#[derive(Default)]
pub struct WorldUpdateSystem;
impl<'a> System<'a> for WorldUpdateSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Worker>,
        WriteStorage<'a, MapObject>,
        WriteStorage<'a, EntityInfo>,
        WriteExpect<'a, TaskQueue>,
        WriteExpect<'a, World>,
        ReadExpect<'a, ResourceConfig>,
    );

    fn run(
        &mut self,
        (entities, mut workers, mut objects, mut entity_infos, mut tasks, mut world, resources): Self::SystemData,
    ) {
        let queue = &mut tasks.world;
        while let Some(event) = queue.pop_front() {
            match event {
                // Add an object to the map.
                TriggerType::Add(pt, name) => {
                    log::info!("Adding object '{}' @ ({:?})", name, pt);
                    let resource = resources.map.get(&name).unwrap().clone();

                    let uuid = Uuid::new_v4();
                    let new_entity = entities
                        .build_entity()
                        .with(MapObject::new(&resource), &mut objects)
                        .with(
                            EntityInfo {
                                uuid,
                                pos: pt,
                                z_offset: 1.0,
                                needs_delete: false,
                                needs_update: false,
                            },
                            &mut entity_infos,
                        )
                        .build();

                    world.add(
                        &pt,
                        new_entity.id(),
                        uuid,
                        ChunkEntity::Object(uuid, ObjectType::TREE),
                    );
                }
                TriggerType::AddWorker(pos) => {
                    log::info!("Adding worker @ ({:?})", pos);

                    let uuid = Uuid::new_v4();
                    let entity = entities
                        .build_entity()
                        .with(Worker::new(), &mut workers)
                        .with(
                            EntityInfo {
                                uuid,
                                pos,
                                z_offset: 1.0,
                                needs_delete: false,
                                needs_update: false,
                            },
                            &mut entity_infos,
                        )
                        .build();

                    world.add(&pos, entity.id(), uuid, ChunkEntity::Worker(uuid));
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
                        if let Some(entity_info) = entity_infos.get_mut(entity) {
                            world.destroy(&entity_info.pos, entity_info.uuid);
                        }
                    }
                    // Remove from world
                    // TODO: Use entity_info.needs_delete = true so that cleanup happens
                    // all at once.
                    entities.delete(entity).unwrap();
                }
                TriggerType::Take { target, owner } => {
                    let target_entity = entities.entity(target);
                    if objects.get(target_entity).is_some() {
                        if let Some(worker) = workers.get_mut(entities.entity(owner)) {
                            worker.inventory.push(target);
                            if let Some(entity_info) = entity_infos.get(target_entity) {
                                world.destroy(&entity_info.pos, entity_info.uuid);
                                entities.delete(target_entity).unwrap();
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
