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
    input::InputHandler,
    ui::UiText,
};

use crate::game::{
    config::PlayerConfig,
    entity::{ ActivityConsole, Player },
    map::Map,
};

pub struct PlayerMovement;
impl<'s> System<'s> for PlayerMovement {
    type SystemData = (
        WriteStorage<'s, Player>,
        ReadExpect<'s, PlayerConfig>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<String, String>>,
        Read<'s, Time>,
        WriteStorage<'s, UiText>,
        ReadExpect<'s, ActivityConsole>,
        ReadExpect<'s, Map>,
    );

    fn run(&mut self, (
        mut players,
        player_config,
        mut transforms,
        input,
        time,
        mut ui_text,
        activity_console,
        map,
    ): Self::SystemData) {
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
                let (map_x, map_y) = map.to_map_coords(player_x, player_y);
                (map_x + x_move as i32, map_y + y_move as i32)
            };

            if !map.has_collision(new_x, new_y) {
                let new_transform = map.place(new_x, new_y, 1.0);
                transform.set_x(new_transform.translation().x);
                transform.set_y(new_transform.translation().y);
                transform.set_z(new_transform.translation().z);
            }

            if let Some(text) = ui_text.get_mut(activity_console.text_handle) {
                text.text = format!("Player: ({}, {})", new_x, new_y);
            }
        }
    }
}