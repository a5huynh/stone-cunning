use amethyst::{
    core::{
        timing::Time,
        transform::Transform,
    },
    ecs::{
        Join,
        Read,
        ReadExpect,
        System,
        WriteStorage,
    },
};

use crate::game::{
    entity::{ DwarfNPC },
    map::Map,
};

pub struct NPCMovement;

impl<'s> System<'s> for NPCMovement {
    type SystemData = (
        WriteStorage<'s, DwarfNPC>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        ReadExpect<'s, Map>,
    );

    fn run(&mut self, (mut dwarves, mut transforms, time, map): Self::SystemData) {
        for (npc, transform) in (&mut dwarves, &mut transforms).join() {
            // Move around randomly
        }
    }
}