use amethyst::{
    ecs::{Entities, ReadExpect, ReadStorage, System, Write},
};
use amethyst_imgui::imgui::{im_str, Condition, Window};

use libdwarf::{
    actions::Action,
    components::{MapObject, Worker},
    resources::TaskQueue,
    Point3,
};

use crate::game::components::CursorSelected;

#[derive(Default)]
pub struct DebugUI;
impl<'s> System<'s> for DebugUI {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, MapObject>,
        ReadStorage<'s, Worker>,
        ReadExpect<'s, CursorSelected>,
        Write<'s, TaskQueue>,
    );

    fn run(
        &mut self,
        (
            entities,
            objects,
            workers,
            cursor_selected,
            mut queue,
        ): Self::SystemData,
    ) {
        amethyst_imgui::with(|ui| {
            Window::new(im_str!("Workers"))
                .size([300.0, 100.0], Condition::FirstUseEver)
                .build(ui, || {
                    if ui.button(im_str!("Add Worker"), [0.0, 0.0]) {
                        queue.add_world(Action::AddWorker(Point3::new(1, 1, 0)));
                    }

                    if ui.button(im_str!("Add Resource"), [0.0, 0.0]) {
                        queue.add_world(Action::Add(Point3::new(9, 9, 0), String::from("tree")));
                    }

                    if ui.button(im_str!("Add Task"), [0.0, 0.0]) {
                        queue.add(Action::HarvestResource(
                            Point3::new(9, 9, 0),
                            String::from("tree"),
                            String::from("wood"),
                        ));
                    }
                });

            Window::new(im_str!("Hover"))
                .size([300.0, 100.0], Condition::FirstUseEver)
                .build(ui, || {
                    let selected = &cursor_selected.hover_selected;
                    if let Some(pick_info) = selected {
                        let worker_label = pick_info.worker
                            .and_then(|worker_id| {
                                let entity = entities.entity(worker_id);
                                workers.get(entity)
                            })
                            .and_then(|worker| Some(format!("worker: {}", worker.to_string())))
                            .unwrap_or("worker: N/A".to_string());
                        ui.text(worker_label);

                        let object_label = pick_info.object
                            .and_then(|object_id| {
                                let entity = entities.entity(object_id);
                                objects.get(entity)
                            })
                            .and_then(|object| Some(format!("object: {}", object.to_string())))
                            .unwrap_or("object: N/A".to_string());
                        ui.text(object_label);

                        let terrain_label = pick_info.terrain.as_ref()
                            .and_then(|terrain| Some(format!("terrain: {:?}", terrain)))
                            .unwrap_or("terrain: N/A".to_string());
                        ui.text(terrain_label);

                        let map_pos = pick_info.position
                            .and_then(|position| {
                                Some(format!("pos: ({}, {}, {})", position.x, position.y, position.z))
                            })
                            .unwrap_or("pos: N/A".to_string());
                        ui.text(map_pos);

                        let mouse_pos = ui.io().mouse_pos;
                        ui.text(im_str!("Mouse Position: ({:.1},{:.1})", mouse_pos[0], mouse_pos[1]));
                    }
                });
        });
    }
}
