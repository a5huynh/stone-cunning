/// Menu state
/// In this state, we allow the user to update settings / choose their save file
/// etc.
///
use core::amethyst::{
    ecs::prelude::Entity,
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    ui::{UiCreator, UiEvent, UiEventType, UiFinder},
};

use crate::state;

const BTN_START: &str = "start";
const BTN_LOAD: &str = "load";
const BTN_OPTS: &str = "options";
const BTN_EXIT: &str = "exit";

pub struct MenuState {
    ui_root: Option<Entity>,
    btn_start: Option<Entity>,
    btn_load: Option<Entity>,
    btn_opts: Option<Entity>,
    btn_exit: Option<Entity>,
}

impl Default for MenuState {
    fn default() -> MenuState {
        MenuState {
            ui_root: None,
            btn_exit: None,
            btn_load: None,
            btn_opts: None,
            btn_start: None,
        }
    }
}

impl SimpleState for MenuState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        // Create the menu UI
        self.ui_root =
            Some(world.exec(|mut creator: UiCreator<'_>| creator.create("ui/menu.ron", ())));
    }

    fn handle_event(
        &mut self,
        _: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match &event {
            StateEvent::Window(event) => {
                // Exit if the user hits escape
                if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                    return Trans::Quit;
                }
            },
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                if Some(*target) == self.btn_exit {
                    return Trans::Quit
                }

                if Some(*target) == self.btn_start {
                    println!("start");
                    return Trans::Switch(Box::new(state::InitState::default()))
                }

                if Some(*target) == self.btn_load {
                    println!("load");
                }

                if Some(*target) == self.btn_opts {
                    println!("options");
                }
            },
            _ => {}

        }
        Trans::None
    }

    fn update(&mut self, date: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let StateData { world, .. } = date;

        if self.btn_exit.is_none()
            || self.btn_start.is_none()
            || self.btn_load.is_none()
            || self.btn_opts.is_none() {
            world.exec(|ui_finder: UiFinder<'_>| {
                self.btn_exit = ui_finder.find(BTN_EXIT);
                self.btn_load = ui_finder.find(BTN_LOAD);
                self.btn_opts = ui_finder.find(BTN_OPTS);
                self.btn_start = ui_finder.find(BTN_START);
            });
        }

        Trans::None
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        // Destroy the menu UI so it's not lingering there when we switch over to
        // the new state.
        if let Some(root_entity) = self.ui_root {
            data.world
                .delete_entity(root_entity)
                .expect("Failed to remove MainMenu");
        }

        // Invalidate references, we grab these on start anyhow.
        self.ui_root = None;
        self.btn_exit = None;
        self.btn_load = None;
        self.btn_opts = None;
        self.btn_start = None;
    }
}
