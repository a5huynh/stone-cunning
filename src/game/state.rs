use amethyst::{
    core::{
        Parent,
        transform::Transform
    },
    input::{ is_close_requested, is_key_down },
    prelude::*,
    renderer::{
        Camera,
        Projection,
        SpriteRender,
        SpriteSheetHandle,
        Transparent,
        VirtualKeyCode,
    }
};

use crate::game::{
    config::GameConfig,
    entity::{ ActivityConsole, CameraFollow, Floor, Object, Player, Map },
    math::{ cart2iso },
    sprite::{ load_sprite_sheet },
};

pub const MAP_HEIGHT: f32 = 1024.0;
pub const MAP_WIDTH: f32 = 1024.0;
pub const FRAC_MAP_HEIGHT_2: f32 = MAP_HEIGHT / 2.0;
pub const FRAC_MAP_WIDTH_2: f32 = MAP_WIDTH / 2.0;

pub struct RunningState;

impl SimpleState for RunningState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let object_spritesheet_handle = load_sprite_sheet(world, "objects");
        let terrain_spritesheet_handle = load_sprite_sheet(world, "terrain");
        let player_spritesheet_handle = load_sprite_sheet(world, "player");

        world.add_resource(Map::default());
        world.register::<Floor>();
        world.register::<Object>();
        world.register::<Player>();

        initialize_map(
            world,
            terrain_spritesheet_handle,
            object_spritesheet_handle
        );

        initialize_camera(world);
        Player::initialize(world, player_spritesheet_handle);
        ActivityConsole::initialize(world);
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
    transform.set_z(10.0);

    // Add an entity we can use to move around the camera.
    let entity = world.create_entity()
        .with(CameraFollow::default())
        .with(transform.clone())
        .build();

    world.create_entity()
        .with(Camera::from(Projection::orthographic(
            -FRAC_MAP_WIDTH_2,
            FRAC_MAP_WIDTH_2,
            -FRAC_MAP_HEIGHT_2,
            FRAC_MAP_HEIGHT_2,
        )))
        .with(Parent { entity })
        .with(transform)
        .build();
}

fn initialize_map(
    world: &mut World,
    sprite_sheet: SpriteSheetHandle,
    objects_sheet: SpriteSheetHandle
) {
    let (map_height, map_width, tile_height, tile_width, tile_scale) = {
        let config = &world.read_resource::<GameConfig>();
        (
            config.map_height,
            config.map_width,
            config.tile_height,
            config.tile_width,
            config.tile_scale
        )
    };

    let scaled_width = (tile_width as f32 * tile_scale) / 2.0;
    let scaled_height = (tile_height as f32 * tile_scale) / 2.0;

    for y in 0..map_height {
        for x in 0..map_width {
            let terrain_render = SpriteRender {
                sprite_sheet: sprite_sheet.clone(),
                sprite_number: ((x + y) % 3) as usize,
            };

            let cart_x = x as f32 * scaled_width;
            let cart_y = y as f32 * scaled_height;
            let zindex = (x + y) as f32;
            let (iso_x, iso_y) = cart2iso(cart_x, cart_y);

            let mut transform = Transform::default();
            // Add tile offset as config option.
            transform.set_xyz(iso_x, iso_y, -zindex);
            transform.set_scale(tile_scale, tile_scale, tile_scale);

            world.create_entity()
                .with(terrain_render)
                .with(Floor::default())
                .with(transform)
                .with(Transparent)
                .build();
        }
    }

    let object_render = SpriteRender {
        sprite_sheet: objects_sheet.clone(),
        sprite_number: 2,
    };

    let cart_x = 5.0 * scaled_width;
    let cart_y = 5.0 * scaled_height;
    let (iso_x, iso_y) = cart2iso(cart_x, cart_y);

    let mut transform = Transform::default();
    transform.set_xyz(iso_x, iso_y + 32.0, -8.0);
    transform.set_scale(tile_scale, tile_scale, tile_scale);
    world.create_entity()
        .with(object_render.clone())
        .with(Object::default())
        .with(transform)
        .with(Transparent)
        .build();

    let mut map = world.write_resource::<Map>();
    map.objects.insert((5, 5), 2);
}
