use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct GameConfig {
    pub tile_height: u32,
    pub tile_width: u32,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct DwarfConfig {
    pub game: GameConfig,
}
