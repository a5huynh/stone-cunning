use amethyst::{
    core::{
        Parent,
        transform::Transform,
        Time,
    },
    input::{ is_close_requested, is_key_down },
    prelude::*,
    renderer::{
        Camera,
        DisplayConfig,
        Projection,
        VirtualKeyCode,
    },
    ui::{ UiCreator, UiFinder, UiText },
    utils::fps_counter::FPSCounter,
};

use libdwarf::world::WorldSim;

use crate::game::{
    config::GameConfig,
    entity::{
        CameraFollow,
        Cursor,
        CursorSelected,
        Floor,
        Object,
        Player
    },
    render::MapRenderer,
    resources::GameTick,
    sprite::SpriteSheetStorage,
};

pub struct RunningState;
impl SimpleState for RunningState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<Floor>();
        world.register::<Object>();
        world.register::<Player>();

        let storage = SpriteSheetStorage::new(world);
        world.add_resource(storage);

        // Initialize simulation;
        let (map_height, map_width) = {
            let config = &world.read_resource::<GameConfig>();
            (config.map_height, config.map_width)
        };
        let world_sim = WorldSim::new(world, map_width, map_height);
        world.add_resource(world_sim);

        initialize_camera(world);
        Cursor::initialize(world);
        // Initialize player.
        Player::initialize(world);

        // Render map
        let map_render = MapRenderer::initialize(world);
        world.add_resource(map_render);

        // Resources are data that is shared amongst all components
        let tick_delta = {
            let config = world.read_resource::<GameConfig>();
            config.tick_delta
        };

        world.add_resource(GameTick::new(tick_delta));
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

    /// Called at an interval of 1/60th second.
    fn fixed_update(&mut self, data: StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let world = data.world;
        // Update global game tick
        {
            let mut tick = world.write_resource::<GameTick>();
            let time = world.read_resource::<Time>().delta_seconds();
            if tick.last_tick > 0.0 {
                tick.last_tick -= time;
            } else {
                // Tick simulation
                tick.reset();
            }
        }

        Trans::None
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let StateData { world, .. } = data;

        // Update FPS counter
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