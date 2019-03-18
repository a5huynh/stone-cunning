use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GameConfig {
    pub map_height: u32,
    pub map_width: u32,
    pub tile_height: u32,
    pub tile_width: u32,
    pub tile_scale: f32,
    pub map_move_speed: f32,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct PlayerConfig {
    pub move_speed: f32,
    // Number of frames in the movement animation.
    pub move_num_frames: u32,
    // How fast the animation will occur.
    pub move_tick: f32,
    // Offset in the player spritesheet for each player animation.
    // 0: idle
    // 1-8: player movement
    pub animation_offsets: [u32; 9],
}


#[derive(Debug, Default, Deserialize, Serialize)]
pub struct DwarfConfig {
    pub game: GameConfig,
    pub player: PlayerConfig,
}
