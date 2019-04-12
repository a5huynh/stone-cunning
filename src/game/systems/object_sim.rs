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
        for (entity, _, transform) in (&*entities, &mut objects, &mut transforms).join() {
            if let Some(_) = map.world.objects.get(&entity.id()) {
                // Update object details
            } else {
                // Doesn't exist anymore in simulation?
                // Remove from world.
                entities.delete(entity).unwrap();
            }
        }
    }
}