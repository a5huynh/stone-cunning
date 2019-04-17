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

pub struct NPCSim;

impl<'s> System<'s> for NPCSim {
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
        // for (entity, _, transform) in (&*entities, &mut dwarves, &mut transforms).join() {
        //     // Update location
        //     let new_transform = map.place(worker.x as i32, worker.y as i32, 1.0);
        //     *transform = new_transform;
        // }
    }
}