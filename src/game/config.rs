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
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct DwarfConfig {
    pub game: GameConfig,
}
