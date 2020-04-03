use core::amethyst::ecs::{Entities, Join, System, Write, WriteStorage};

use crate::{
    components::{EntityInfo, MapObject, ResourceAttribute},
    resources::TaskQueue,
    trigger::TriggerType,
};

pub struct ObjectSystem;
impl<'a> System<'a> for ObjectSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, MapObject>,
        WriteStorage<'a, EntityInfo>,
        Write<'a, TaskQueue>,
    );

    fn run(&mut self, (entities, mut objects, mut entity_infos, mut tasks): Self::SystemData) {
        for (entity, object, entity_info) in (&*entities, &mut objects, &mut entity_infos).join() {
            let current_pos = entity_info.pos;
            // Check object health. Queue destruction if <= 0.
            if object.health <= 0 {
                // Destroy this object
                tasks.add_world(TriggerType::Destroy(entity.id()));
                // Add any drops to world
                for drop in &object.drop_table() {
                    match drop {
                        ResourceAttribute::Drops(name, _amount) => {
                            tasks.add_world(TriggerType::Add(current_pos, name.to_string()));
                        }
                        _ => {}
                    }
                }
                continue;
            }
        }
    }
}
