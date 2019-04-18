use specs::{Entities, Join, System, Write, WriteStorage};

use crate::{
    actions::Action,
    entities::{MapObject, MapPosition, ResourceAttribute},
    resources::TaskQueue,
};

pub struct ObjectSystem;
impl<'a> System<'a> for ObjectSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, MapObject>,
        WriteStorage<'a, MapPosition>,
        Write<'a, TaskQueue>,
    );

    fn run(&mut self, (entities, mut objects, mut positions, mut tasks): Self::SystemData) {
        for (entity, object, position) in (&*entities, &mut objects, &mut positions).join() {
            // Check object health. Queue destruction if <= 0.
            if object.health <= 0 {
                // Destroy this object
                tasks.add_world(Action::Destroy(entity.id()));
                // Add any drops to world
                for drop in object.drop_table().iter() {
                    match drop {
                        ResourceAttribute::Drops(name, _amount) => {
                            tasks
                                .add_world(Action::Add((position.x, position.y), name.to_string()));
                        }
                        _ => {}
                    }
                }
                continue;
            }

            // Otherwise, process any object specific actions.
            while let Some(action) = object.actions.pop_front() {
                match action {
                    _ => {}
                }
            }
        }
    }
}
