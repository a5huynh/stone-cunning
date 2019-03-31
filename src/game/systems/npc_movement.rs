use amethyst::{
    core::transform::Transform,
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
    map::MapResource,
    resources::GameTick,
};

pub struct NPCMovement;

impl<'s> System<'s> for NPCMovement {
    type SystemData = (
        WriteStorage<'s, DwarfNPC>,
        WriteStorage<'s, Transform>,
        ReadExpect<'s, MapResource>,
        Read<'s, GameTick>,
    );

    fn run(&mut self, (mut dwarves, mut transforms, map, tick): Self::SystemData) {
        for (_npc, transform) in (&mut dwarves, &mut transforms).join() {
            // Move around randomly
            let npc_x = transform.translation().x;
            let npc_y = transform.translation().y;

            let (mut map_x, mut map_y) = map.to_map_coords(npc_x, npc_y);

            let dist_x = 9 - map_x;
            let dist_y = 9 - map_y;

            if dist_x == 0 && dist_y == 0 {
                continue;
            } else if dist_x > dist_y {
                map_x += 1;
            } else {
                map_y += 1;
            }

            let new_transform = map.place(map_x, map_y, 1.0);
            if tick.last_tick <= 0.0 {
                transform.set_x(new_transform.translation().x);
                transform.set_y(new_transform.translation().y);
                transform.set_z(new_transform.translation().z);
            }
        }
    }
}