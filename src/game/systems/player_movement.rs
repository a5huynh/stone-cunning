use amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::{Join, Read, ReadExpect, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::game::{components::Player, config::PlayerConfig, render::MapRenderer};
use core::Point3;
use libdwarf::resources::Map;

pub struct PlayerMovement;
impl<'s> System<'s> for PlayerMovement {
    type SystemData = (
        WriteStorage<'s, Player>,
        ReadExpect<'s, PlayerConfig>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
        ReadExpect<'s, Map>,
        ReadExpect<'s, MapRenderer>,
    );

    fn run(
        &mut self,
        (
        mut players,
        player_config,
        mut transforms,
        input,
        time,
        map,
        map_render,
    ): Self::SystemData,
    ) {
        let x_move = input.axis_value("player_x").unwrap();
        let y_move = input.axis_value("player_y").unwrap();

        for (player, transform) in (&mut players, &mut transforms).join() {
            player.last_tick -= time.delta_seconds();
            if player.last_tick > 0.0 {
                continue;
            }

            player.last_tick = player_config.move_tick;
            let player_x = transform.translation().x;
            let player_y = transform.translation().y;
            // Convert player position into map coordinates and bump to new location.
            let (new_x, new_y) = {
                let (map_x, map_y) = map_render.to_map_coords(player_x, player_y);
                (map_x + x_move as i32, map_y + y_move as i32)
            };

            if !map.has_collision(Point3::new(new_x, new_y, 0)) {
                let new_transform = map_render.place(new_x, new_y, 0, 1.0);
                transform.set_translation_x(new_transform.translation().x);
                transform.set_translation_y(new_transform.translation().y);
                transform.set_translation_z(new_transform.translation().z);
            }
        }
    }
}
