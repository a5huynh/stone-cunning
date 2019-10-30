use amethyst::{
    core::{transform::Transform, ArcThreadPool, Parent, SystemBundle, Time},
    ecs::{Dispatcher, DispatcherBuilder},
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{camera::Projection, Camera},
    ui::{UiCreator, UiFinder, UiText},
    utils::fps_counter::FpsCounter,
    window::DisplayConfig,
};

use crate::game::{
    components::CameraFollow,
    render::MapRenderer,
    systems::{
        camera, ui::debug::DebugUI, ClickSystem, CursorSystem, PlayerMovement, RenderNPCSystem,
        RenderObjectSystem,
    },
};
use libdwarf::WorldSimBundle;

pub struct RunningState<'a, 'b> {
    dispatcher: Option<Dispatcher<'a, 'b>>,
    input_dispatcher: Option<Dispatcher<'a, 'b>>,
    ui_dispatcher: Option<Dispatcher<'a, 'b>>,
    paused: bool,
    zoom: f32,
}

impl Default for RunningState<'_, '_> {
    fn default() -> Self {
        RunningState {
            dispatcher: None,
            input_dispatcher: None,
            ui_dispatcher: None,
            paused: false,
            zoom: 3.0,
        }
    }
}

impl<'a, 'b> SimpleState for RunningState<'a, 'b> {
    fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        let mut world = &mut data.world;

        let mut dispatcher_builder = DispatcherBuilder::new();
        WorldSimBundle::default()
            .build(&mut world, &mut dispatcher_builder)
            .expect("Failed to register WorldSimBundle");

        // Render systems. Takes entities from the simulations and assigns sprites
        // to them as they get added.
        dispatcher_builder.add(RenderObjectSystem, "render_obj_system", &["world_updates"]);
        dispatcher_builder.add(RenderNPCSystem, "render_npc_system", &["world_updates"]);

        let mut input_db = DispatcherBuilder::new();
        // Cursor selection
        input_db.add(CursorSystem, "cursor", &[]);
        // We handle click after the cursor is correctly transformed on the map.
        input_db.add(ClickSystem, "click", &["cursor"]);
        // Moving around the map
        input_db.add(camera::MapMovementSystem, "map_movement", &[]);
        input_db.add(camera::CameraZoomSystem, "camera_zoom", &[]);
        input_db.add(PlayerMovement, "player_movement", &[]);

        let mut ui_db = DispatcherBuilder::new();
        // Should always be last so we have the most up-to-date info.
        ui_db.add(DebugUI::default(), "debug_ui", &[]);

        let mut dispatcher = dispatcher_builder
            .with_pool((*world.read_resource::<ArcThreadPool>()).clone())
            .build();

        let mut input_dispatcher = input_db
            .with_pool((*world.read_resource::<ArcThreadPool>()).clone())
            .build();

        let mut ui_dispatcher = ui_db
            .with_pool((*world.read_resource::<ArcThreadPool>()).clone())
            .build();

        dispatcher.setup(world);
        input_dispatcher.setup(world);
        ui_dispatcher.setup(world);

        self.dispatcher = Some(dispatcher);
        self.input_dispatcher = Some(input_dispatcher);
        self.ui_dispatcher = Some(ui_dispatcher);

        // Initialize the camera
        let point = {
            let map_render = world.read_resource::<MapRenderer>();
            map_render.place(8, 8, 42, 0.0)
        };
        initialize_camera(world, point, self.zoom);

        // Create the ui
        world.exec(|mut creator: UiCreator<'_>| {
            creator.create("resources/ui/debug.ron", ());
        });
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Exit if the user hits escape
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }

            if is_key_down(&event, VirtualKeyCode::Space) {
                self.paused = !self.paused;
            }
        }

        Trans::None
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let StateData { world, .. } = data;

        if !self.paused {
            if let Some(dispatcher) = self.dispatcher.as_mut() {
                dispatcher.dispatch(&world);
            }
        }

        if let Some(dispatcher) = self.input_dispatcher.as_mut() {
            dispatcher.dispatch(&world);
        }

        if let Some(dispatcher) = self.ui_dispatcher.as_mut() {
            dispatcher.dispatch(&world);
        }

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
                    let fps_samp = world.read_resource::<FpsCounter>().sampled_fps();
                    fps.text = format!("FPS: {:.*}", 2, fps_samp);
                }
            }
        }

        Trans::None
    }
}

fn initialize_camera(world: &mut World, center: Transform, cam_zoom: f32) {
    let (window_width, window_height) = {
        let display = world.read_resource::<DisplayConfig>();
        display.dimensions.unwrap()
    };

    // Add an entity we can use to move around the camera.
    let mut transform = center.clone();
    transform.set_translation_z(10.0);
    let entity = world
        .create_entity()
        .with(CameraFollow::default())
        .with(transform.clone())
        .build();

    let width = window_width as f32 / cam_zoom;
    let height = window_height as f32 / cam_zoom;

    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            -(width as f32),
            width as f32,
            -(height as f32),
            height as f32,
            -100.0,
            100.0,
        )))
        .with(Parent { entity })
        .with(Transform::default())
        .build();
}
