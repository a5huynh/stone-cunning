use amethyst::{
    assets::{ AssetStorage, Loader },
    core::transform::Transform,
    input::{ is_close_requested, is_key_down },
    prelude::*,
    renderer::{
        Camera,
        PngFormat,
        Projection,
        SpriteRender,
        SpriteSheet,
        SpriteSheetFormat,
        SpriteSheetHandle,
        Texture,
        TextureMetadata,
        Transparent,
        VirtualKeyCode,
    },
};

use super::{
    config::GameConfig,
    entity::{ Floor },
    math::{
        cart2iso
    },
};

pub const MAP_HEIGHT: f32 = 1024.0;
pub const MAP_WIDTH: f32 = 1024.0;
pub const FRAC_MAP_HEIGHT_2: f32 = MAP_HEIGHT / 2.0;
pub const FRAC_MAP_WIDTH_2: f32 = MAP_WIDTH / 2.0;

pub struct RunningState;

impl SimpleState for RunningState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let sprite_sheet_handle = load_sprite_sheet(world);

        world.register::<Floor>();
        initialize_map(world, sprite_sheet_handle);
        initialize_camera(world);
    }

    fn handle_event(&mut self, _: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Exit if the user hits escape
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }

        Trans::None
    }
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);

    world.create_entity()
        .with(Camera::from(Projection::orthographic(
            -FRAC_MAP_WIDTH_2,
            FRAC_MAP_WIDTH_2,
            -FRAC_MAP_HEIGHT_2,
            FRAC_MAP_HEIGHT_2,
        )))
        .with(transform)
        .build();
}

fn initialize_map(world: &mut World, sprite_sheet: SpriteSheetHandle) {
    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet.clone(),
        sprite_number: 0,
    };

    let (tile_height, tile_width) = {
        let config = &world.read_resource::<GameConfig>();
        (config.tile_height, config.tile_width)
    };

    for y in 0..2 {
        for x in 0..2 {
            let cart_x = ((2 - x) * tile_height / 2) as f32;
            let cart_y = ((2 - y) * tile_width / 2) as f32;

            let (iso_x, iso_y) = cart2iso(cart_x, cart_y);

            let mut transform = Transform::default();
            transform.set_xyz(iso_x, iso_y, 0.0);

            world.create_entity()
                .with(sprite_render.clone())
                .with(Floor::default())
                .with(transform)
                .with(Transparent)
                .build();
        }
    }
}


fn load_sprite_sheet(world: &mut World) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "./resources/textures/spritesheet.png",
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "./resources/textures/spritesheet.ron",
        SpriteSheetFormat,
        texture_handle,
        (),
        &sprite_sheet_store
    )
}