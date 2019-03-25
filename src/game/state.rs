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
        VirtualKeyCode,
    }
};

use crate::game::{
    entity::{
        ActivityConsole,
        CameraFollow,
        DwarfNPC,
        Floor,
        Object,
        Player
    },
    map::Map,
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
        let npc_spritesheet_handle = load_sprite_sheet(world, "npc");

        world.register::<Floor>();
        world.register::<Object>();
        world.register::<Player>();

        initialize_camera(world);
        // Initialize map terrain & objects.
        Map::initialize(world, terrain_spritesheet_handle, object_spritesheet_handle);
        // Initialize dwarf.
        DwarfNPC::initialize(world, npc_spritesheet_handle);
        // Initialize player.
        Player::initialize(world, player_spritesheet_handle);
        // Setup activity console.
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