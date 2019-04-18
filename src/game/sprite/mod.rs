use amethyst::{
    assets::{AssetStorage, Loader},
    prelude::*,
    renderer::{
        PngFormat, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle, Texture, TextureMetadata,
    },
};

pub struct SpriteSheetStorage {
    pub cursor: SpriteSheetHandle,
    pub object: SpriteSheetHandle,
    pub terrain: SpriteSheetHandle,
    pub player: SpriteSheetHandle,
    pub npc: SpriteSheetHandle,
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

pub fn load_sprite_sheet(world: &mut World, name: &str) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            format!("./resources/textures/{}/spritesheet.png", name),
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        format!("./resources/textures/{}/spritesheet.ron", name),
        SpriteSheetFormat,
        texture_handle,
        (),
        &sprite_sheet_store,
    )
}
