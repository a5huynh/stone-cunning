use amethyst::{
    core::{transform::Transform, Parent, Time},
    ecs::{Join, Read, WriteStorage},
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{camera::Projection, Camera},
    ui::{UiCreator, UiFinder, UiText},
    utils::fps_counter::FpsCounter,
    window::DisplayConfig,
    winit::{Event, MouseScrollDelta, WindowEvent},
};

use crate::game::{
    components::CameraFollow,
    render::MapRenderer,
};

pub struct RunningState {
    zoom: f32,
}

impl Default for RunningState {
    fn default() -> RunningState {
        RunningState { zoom: 3.0 }
    }
}

impl SimpleState for RunningState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
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
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let world = data.world;

        if let StateEvent::Window(event) = &event {
            // Exit if the user hits escape
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }

            // Detect
            match event {
                Event::WindowEvent {
                    event:
                        WindowEvent::MouseWheel {
                            delta: MouseScrollDelta::LineDelta(_, scroll_y),
                            ..
                        },
                    ..
                } => {
                    world.exec(
                        |(mut cameras, display): (WriteStorage<Camera>, Read<DisplayConfig>)| {
                            let camera = (&mut cameras).join().next().or(None);
                            if let Some(camera) = camera {
                                self.zoom = (self.zoom + scroll_y / 4.0).max(1.0).min(10.0);
                                let (window_width, window_height) = display.dimensions.unwrap();
                                let zoom_width = window_width as f32 / self.zoom;
                                let zoom_height = window_height as f32 / self.zoom;

                                *camera = Camera::from(Projection::orthographic(
                                    -zoom_width,
                                    zoom_width,
                                    -zoom_height,
                                    zoom_height,
                                    -100.0,
                                    100.0,
                                ));
                            }
                        },
                    );
                }
                _ => {}
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
