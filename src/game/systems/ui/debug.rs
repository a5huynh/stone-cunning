use amethyst::{
    ecs::{ReadExpect, ReadStorage, System, Write, WriteStorage},
    shrev::{EventChannel, ReaderId},
    ui::{UiEvent, UiEventType, UiFinder, UiText, UiTransform},
};

use libdwarf::{actions::Action, resources::TaskQueue};

use crate::game::entity::CursorSelected;

#[derive(Default)]
pub struct DebugUI {
    reader_id: Option<ReaderId<UiEvent>>,
}

impl<'s> System<'s> for DebugUI {
    type SystemData = (
        UiFinder<'s>,
        WriteStorage<'s, UiText>,
        ReadExpect<'s, CursorSelected>,
        Write<'s, TaskQueue>,
        Write<'s, EventChannel<UiEvent>>,
        ReadStorage<'s, UiTransform>,
    );

    fn run(
        &mut self,
        (finder, mut ui_text, cursor_selected, mut queue, mut events, buttons): Self::SystemData,
    ) {
        // Render currently selected info data
        if let Some(entity) = finder.find("debug_info") {
            if let Some(text) = ui_text.get_mut(entity) {
                let selected = &cursor_selected.hover_selected;
                if let Some(pick_info) = selected {
                    text.text = format!(
                        "object: {:?}\nterrain: {:?}",
                        pick_info.object, pick_info.terrain
                    );
                } else {
                    text.text = "object: N/A\nterrain: N/A".to_string();
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
