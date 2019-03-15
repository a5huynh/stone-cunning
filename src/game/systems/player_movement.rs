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
};

use super::super::{
    config::PlayerConfig,
    entity::{ Player },
};

pub struct PlayerMovement;
impl<'s> System<'s> for PlayerMovement {
    type SystemData = (
        WriteStorage<'s, Player>,
        ReadExpect<'s, PlayerConfig>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, InputHandler<String, String>>,
        Read<'s, Time>
    );

    fn run(&mut self, (
        mut players,
        player_config,
        mut transforms,
        mut sprites,
        input,
        time
    ): Self::SystemData) {
        let x_move = input.axis_value("player_x").unwrap();
        let y_move = input.axis_value("player_y").unwrap();

        for (player, transform, sprite) in (&mut players, &mut transforms, &mut sprites).join() {
            transform.translate_x(x_move as f32 * player_config.move_speed);
            transform.translate_y(y_move as f32 * player_config.move_speed);

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