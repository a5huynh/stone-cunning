/// Handles the necessary checks to remove objects from the World when they
/// get below 0 health.

use amethyst::{
    core::transform::Transform,
    ecs::{
        Entities,
        Join,
        System,
        ReadExpect,
        WriteStorage,
    },
};

use crate::game:: {
    entity::Object,
    map::MapResource,
};

pub struct ObjectSim;

impl<'s> System<'s> for ObjectSim {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Object>,
        WriteStorage<'s, Transform>,
        ReadExpect<'s, MapResource>,
    );

    fn run(&mut self, (entities, mut objects, mut transforms, map): Self::SystemData) {
        for (entity, object, transform) in (&*entities, &mut objects, &mut transforms).join() {
            if let Some(object_data) = map.world.objects.get(&entity.id()) {
                // Update object details
                let new_transform = map.place(object_data.x as i32, object_data.y as i32, 1.0);
                *transform = new_transform;
            } else {
                // Doesn't exist anymore in simulation?
                // Remove from world.
                entities.delete(entity).unwrap();
            }
        }
    }
}