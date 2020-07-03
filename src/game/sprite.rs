use core::amethyst::{
    assets::{AssetStorage, Handle, Loader},
    prelude::*,
    renderer::{ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
};

pub struct SpriteSheetStorage {
    pub cursor: Handle<SpriteSheet>,
    pub object: Handle<SpriteSheet>,
    pub terrain: Handle<SpriteSheet>,
    pub player: Handle<SpriteSheet>,
    pub npc: Handle<SpriteSheet>,
}

impl SpriteSheetStorage {
    pub fn new(world: &mut World) -> Self {
        SpriteSheetStorage {
            cursor: load_sprite_sheet(world, "cursor"),
            object: load_sprite_sheet(world, "objects"),
            terrain: load_sprite_sheet(world, "terrain"),
            player: load_sprite_sheet(world, "player"),
            npc: load_sprite_sheet(world, "npc"),
        }
    }
}

pub fn load_sprite_sheet(world: &mut World, name: &str) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            format!("textures/{}/spritesheet.png", name),
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        format!("textures/{}/spritesheet.ron", name),
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}
