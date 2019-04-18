use amethyst::{
    core::transform::Transform,
    ecs::{Join, ReadExpect, ReadStorage, System, Write, WriteStorage},
    shrev::{EventChannel, ReaderId},
    ui::{UiEvent, UiEventType, UiFinder, UiText, UiTransform},
};

use libdwarf::{actions::Action, resources::TaskQueue};

use crate::game::{
    entity::{CursorSelected, Player},
    render::MapRenderer,
};

#[derive(Default)]
pub struct DebugUI {
    reader_id: Option<ReaderId<UiEvent>>,
}

impl<'s> System<'s> for DebugUI {
    type SystemData = (
        WriteStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        UiFinder<'s>,
        WriteStorage<'s, UiText>,
        ReadExpect<'s, CursorSelected>,
        ReadExpect<'s, MapRenderer>,
        Write<'s, TaskQueue>,
        Write<'s, EventChannel<UiEvent>>,
        ReadStorage<'s, UiTransform>,
    );

    fn run(
        &mut self,
        (
            mut players,
            mut transforms,
            finder,
            mut ui_text,
            cursor_selected,
            map_render,
            mut queue,
            mut events,
            buttons,
        ): Self::SystemData,
    ) {
        let player_loc = (&mut players, &mut transforms).join().next().or(None);

        // Render current player map location
        if let Some((_player, transform)) = player_loc {
            let player_x = transform.translation().x;
            let player_y = transform.translation().y;
            // Convert player position into map coordinates and bump to new location.
            let (map_x, map_y) = map_render.to_map_coords(player_x, player_y);

            if let Some(entity) = finder.find("player_info") {
                if let Some(text) = ui_text.get_mut(entity) {
                    text.text = format!("Player: ({}, {})", map_x, map_y);
                }
            }
        }

        // Render currently selected info data
        if let Some(entity) = finder.find("debug_info") {
            if let Some(text) = ui_text.get_mut(entity) {
                let selected = &cursor_selected.selected;
                if let Some(pick_info) = selected {
                    text.text = format!(
                        "terrain: {}\ndesc: {}",
                        pick_info.is_terrain, pick_info.description,
                    );
                } else {
                    text.text = "N/A\nN/A".to_string();
                }
            }
        }

        // Handle UI events
        let reader_id = self
            .reader_id
            .get_or_insert_with(|| events.register_reader());

        for ev in events.read(reader_id) {
            match ev.event_type {
                UiEventType::Click => {
                    // Determine which button was clicked and implement action.
                    if let Some(button) = buttons.get(ev.target) {
                        match button.id.as_ref() {
                            "add_worker_btn" => {
                                queue.add_world(Action::AddWorker((1, 1)));
                            }
                            "add_resource_btn" => {
                                queue.add_world(Action::Add((9, 9), String::from("tree")));
                            }
                            "add_task_btn" => {
                                queue.add(Action::HarvestResource(
                                    (9, 9),
                                    String::from("tree"),
                                    String::from("wood"),
                                ));
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
