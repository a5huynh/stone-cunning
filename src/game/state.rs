use amethyst::{
    core::{
        Parent,
        transform::Transform
    },
    input::{ is_close_requested, is_key_down },
    prelude::*,
    renderer::{
        Camera,
        DisplayConfig,
        Projection,
        VirtualKeyCode,
    }
};

use crate::game::{
    entity::{
        ActivityConsole,
        CameraFollow,
        Cursor,
        DwarfNPC,
        Floor,
        Object,
        Player
    },
    map::Map,
    sprite::{ load_sprite_sheet },
};

pub struct RunningState;

impl SimpleState for RunningState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let cursor_spritesheet_handle = load_sprite_sheet(world, "cursor");
        let object_spritesheet_handle = load_sprite_sheet(world, "objects");
        let terrain_spritesheet_handle = load_sprite_sheet(world, "terrain");
        let player_spritesheet_handle = load_sprite_sheet(world, "player");
        let npc_spritesheet_handle = load_sprite_sheet(world, "npc");

        world.register::<Floor>();
        world.register::<Object>();
        world.register::<Player>();

        initialize_camera(world);
        // Initialize map terrain & objects.
        let map = Map::initialize(world, terrain_spritesheet_handle, object_spritesheet_handle);
        Cursor::initialize(world, cursor_spritesheet_handle);
        // Initialize dwarf.
        DwarfNPC::initialize(world, &map, npc_spritesheet_handle);
        // Initialize player.
        Player::initialize(world, player_spritesheet_handle);
        // Setup activity console.
        ActivityConsole::initialize(world);

        world.add_resource(map);
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
    let (window_width, window_height) = {
        let display = world.read_resource::<DisplayConfig>();
        display.dimensions.unwrap()
    };

    let mut transform = Transform::default();
    transform.set_z(10.0);

    // Add an entity we can use to move around the camera.
    let entity = world.create_entity()
        .with(CameraFollow::default())
        .with(transform.clone())
        .build();

    let cam_zoom = 3.0;
    let window_width_half = window_width as f32 / (2.0 * cam_zoom);
    let window_height_half = window_height as f32 / (2.0 * cam_zoom);

    world.create_entity()
        .with(Camera::from(Projection::orthographic(
            -window_width_half,
            window_width_half,
            -window_height_half,
            window_height_half,
        )))
        .with(Parent { entity })
        .with(transform)
        .build();
}