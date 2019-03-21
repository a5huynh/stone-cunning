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
    renderer::SpriteRender,
    ui::UiText,
};

use crate::game::{
    config::PlayerConfig,
    entity::{ ActivityConsole, Player },
    math::{ iso2cart },
};

pub struct PlayerMovement;
impl<'s> System<'s> for PlayerMovement {
    type SystemData = (
        WriteStorage<'s, Player>,
        ReadExpect<'s, PlayerConfig>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, InputHandler<String, String>>,
        Read<'s, Time>,
        WriteStorage<'s, UiText>,
        ReadExpect<'s, ActivityConsole>,
    );

    fn run(&mut self, (
        mut players,
        player_config,
        mut transforms,
        mut sprites,
        input,
        time,
        mut ui_text,
        activity_console,
    ): Self::SystemData) {
        let x_move = input.axis_value("player_x").unwrap();
        let y_move = input.axis_value("player_y").unwrap();

        for (player, transform, sprite) in (&mut players, &mut transforms, &mut sprites).join() {
            let player_x = transform.translation().x;
            let player_y = transform.translation().y;

            let mut new_x = player_x + x_move as f32 * player_config.move_speed;
            let mut new_y = player_y + y_move as f32 * player_config.move_speed;

            // Check collisions

            let (cartx, carty) = iso2cart(new_x, new_y);
            let map_x = (cartx / 48.0).floor();
            let map_y = (carty / 48.0).floor();
            let zindex = map_x + map_y - 1.0;

            transform.set_x(new_x);
            transform.set_y(new_y);
            transform.set_z(-zindex);

            if let Some(text) = ui_text.get_mut(activity_console.text_handle) {
                text.text = format!(
                    "Player: ({}, {}, {}) ({}, {})",
                    new_x, new_y, -zindex, map_x, map_y,
                )
            }
            // handle character animation
            let mut idle = true;
            // Start of the animation for this direction;
            if x_move != 0.0 || y_move != 0.0 {
                idle = false;
            }

            let new_dir = Player::calculate_direction(x_move, y_move);
            let dir_offset = player_config.animation_offsets[new_dir.clone() as usize];
            let has_new_dir = new_dir != player.direction;

            // 1, 2, 3, 4
            if idle {
                sprite.sprite_number = 0;
            } else {
                player.ticks += time.delta_seconds();
                if player.ticks > player_config.move_tick {
                    player.ticks = 0.0;

                    let sprite_max = player_config.move_num_frames + dir_offset - 1;
                    if has_new_dir {
                        sprite.sprite_number = dir_offset as usize;
                        player.direction = new_dir.clone();
                    } else {
                        sprite.sprite_number = sprite.sprite_number + 1;
                    }

                    if sprite.sprite_number > sprite_max as usize {
                        sprite.sprite_number = dir_offset as usize;
                    }
                }
            }
        }
    }
}