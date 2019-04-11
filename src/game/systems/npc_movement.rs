use amethyst::{
    core::transform::Transform,
    ecs::{
        Entities,
        Join,
        ReadExpect,
        System,
        WriteStorage,
    },
};

use crate::game::{
    entity::{ DwarfNPC },
    map::MapResource
};

pub struct NPCMovement;

impl<'s> System<'s> for NPCMovement {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, DwarfNPC>,
        WriteStorage<'s, Transform>,
        ReadExpect<'s, MapResource>
    );

    fn run(&mut self, (
        entities,
        mut dwarves,
        mut transforms,
        map
    ): Self::SystemData) {
        for (entity, _, transform) in (&*entities, &mut dwarves, &mut transforms).join() {
            // Get worker reference from sim
            if let Some(worker) = map.world.get_worker(entity.id()) {
                // Update location
                let new_transform = map.place(worker.x as i32, worker.y as i32, 1.0);
                transform.set_x(new_transform.translation().x);
                transform.set_y(new_transform.translation().y);
                transform.set_z(new_transform.translation().z);
            }
        }
    }
}