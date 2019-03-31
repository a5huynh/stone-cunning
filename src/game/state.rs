use amethyst::{
    core::{
        Parent,
        transform::Transform,
        Time,
    },
    input::{ InputEvent, is_close_requested, is_key_down },
    prelude::*,
    renderer::{
        Camera,
        DisplayConfig,
        Projection,
        VirtualKeyCode,
    },
    shrev::{ EventChannel, ReaderId },
    ui::{ UiCreator, UiFinder, UiText },
    utils::fps_counter::FPSCounter,
};

use crate::game::{
    entity::{
        CameraFollow,
        Cursor,
        CursorSelected,
        DwarfNPC,
        Floor,
        Object,
        Player
    },
    map::MapResource,
    sprite::{ load_sprite_sheet },
};

#[derive(Default)]
pub struct RunningState {
    event_reader: Option<ReaderId<InputEvent<String>>>,
}

impl SimpleState for RunningState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.event_reader = {
            let mut channel = world.write_resource::<EventChannel<InputEvent<String>>>();
            Some(channel.register_reader())
        };

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
        let map = MapResource::initialize(world, terrain_spritesheet_handle, object_spritesheet_handle);
        Cursor::initialize(world, cursor_spritesheet_handle);
        // Initialize dwarf.
        DwarfNPC::initialize(world, &map, npc_spritesheet_handle);
        // Initialize player.
        Player::initialize(world, player_spritesheet_handle);
        // Resources are data that is shared amongst all components
        world.add_resource(map);
        world.add_resource(CursorSelected::default());

        // Create the ui
        world.exec(|mut creator: UiCreator<'_>| {
            creator.create("resources/ui/debug.ron", ());
        });
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

    fn shadow_update(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let event_channel = world.read_resource::<EventChannel<InputEvent<String>>>();
        for event in event_channel.read(self.event_reader.as_mut().unwrap()) {
            if let InputEvent::ActionPressed(action) = event {
                match &**action {
                    "menu" => println!("ACTION!"),
                    _ => {},
                }
            }
        }
    }
    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let StateData { world, .. } = data;

        let mut fps_display = None;
        world.exec(|finder: UiFinder<'_>| {
            if let Some(entity) = finder.find("fps") {
                fps_display = Some(entity);
            }
        });

        let mut ui_text = world.write_storage::<UiText>();
        {
            if let Some(fps) = fps_display.and_then(|entity| ui_text.get_mut(entity)) {
                if world.read_resource::<Time>().frame_number() % 20 == 0 {
                    let fps_samp = world.read_resource::<FPSCounter>().sampled_fps();
                    fps.text = format!("FPS: {:.*}", 2, fps_samp);
                }
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