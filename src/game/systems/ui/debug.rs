use amethyst::{
    ecs::{Entities, ReadExpect, ReadStorage, System, Write, WriteStorage},
    shrev::{EventChannel, ReaderId},
    ui::{UiEvent, UiEventType, UiFinder, UiText, UiTransform},
};

use libdwarf::{
    actions::Action,
    components::{MapObject, Worker},
    resources::TaskQueue,
    Point3,
};

use crate::game::components::CursorSelected;

#[derive(Default)]
pub struct DebugUI {
    reader_id: Option<ReaderId<UiEvent>>,
}

impl<'s> System<'s> for DebugUI {
    type SystemData = (
        Entities<'s>,
        UiFinder<'s>,
        ReadStorage<'s, MapObject>,
        ReadStorage<'s, Worker>,
        WriteStorage<'s, UiText>,
        ReadExpect<'s, CursorSelected>,
        Write<'s, TaskQueue>,
        Write<'s, EventChannel<UiEvent>>,
        ReadStorage<'s, UiTransform>,
    );

    fn run(
        &mut self,
        (
            entities,
            finder,
            objects,
            workers,
            mut ui_text,
            cursor_selected,
            mut queue,
            mut events,
            buttons,
        ): Self::SystemData,
    ) {
        // Render currently selected info data
        if let Some(entity) = finder.find("debug_info") {
            let label = ui_text.get_mut(entity).unwrap();
            let selected = &cursor_selected.hover_selected;
            if let Some(pick_info) = selected {
                let mut worker_str = String::from("N/A");
                if let Some(worker_id) = pick_info.worker {
                    let entity = entities.entity(worker_id);
                    if let Some(worker) = workers.get(entity) {
                        worker_str = worker.to_string();
                    }
                }

                let mut object_str = String::from("N/A");
                if let Some(object_id) = pick_info.object {
                    let entity = entities.entity(object_id);
                    if let Some(object) = objects.get(entity) {
                        object_str = object.to_string();
                    }
                }

                let mut terrain_str = String::from("N/A");
                if let Some(terrain) = &pick_info.terrain {
                    terrain_str = format!("{:?}", terrain);
                }

                let mut pos_str = String::from("N/A");
                if let Some(position) = &pick_info.position {
                    pos_str = format!("({}, {}, {})", position.x, position.y, position.z);
                }

                label.text = format!(
                    "worker: {}\nobject: {}\nterrain: {}\npos: {}",
                    worker_str, object_str, terrain_str, pos_str
                );
            } else {
                label.text = "worker: N/A\nobject: N/A\nterrain: N/A\npos: N/A".to_string();
            }
        }

        // Handle UI events
        let reader_id = self
            .reader_id
            .get_or_insert_with(|| events.register_reader());

        for ev in events.read(reader_id) {
            if ev.event_type == UiEventType::Click {
                // Determine which button was clicked and implement action.
                if let Some(button) = buttons.get(ev.target) {
                    match button.id.as_ref() {
                        "add_worker_btn" => {
                            queue.add_world(Action::AddWorker(Point3::new(1, 1, 0)));
                        }
                        "add_resource_btn" => {
                            queue
                                .add_world(Action::Add(Point3::new(9, 9, 0), String::from("tree")));
                        }
                        "add_task_btn" => {
                            queue.add(Action::HarvestResource(
                                Point3::new(9, 9, 0),
                                String::from("tree"),
                                String::from("wood"),
                            ));
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
