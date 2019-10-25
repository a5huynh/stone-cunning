use amethyst::ecs::{Entities, Join, ReadExpect, ReadStorage, System, Write};
use amethyst_imgui::imgui::{im_str, Condition, Window};

use core::Point3;
use libdwarf::{
    components::{MapObject, Worker},
    resources::TaskQueue,
    trigger::TriggerType,
};

use crate::game::components::CursorSelected;

#[derive(Default)]
pub struct DebugUI {
    new_worker_pos: [i32; 3],
}

impl<'s> System<'s> for DebugUI {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, MapObject>,
        ReadStorage<'s, Worker>,
        ReadExpect<'s, CursorSelected>,
        Write<'s, TaskQueue>,
    );

    fn run(&mut self, (entities, objects, workers, cursor_selected, mut queue): Self::SystemData) {
        amethyst_imgui::with(|ui| {
            Window::new(im_str!("Tasks"))
                .size([300.0, 500.0], Condition::FirstUseEver)
                .build(ui, || {
                    if ui.collapsing_header(im_str!("world")).build() {
                        for action in queue.world.iter() {
                            ui.text(&im_str!("{:?}", action));
                        }
                    }

                    if ui.collapsing_header(im_str!("worker")).build() {
                        for action in queue.worker.iter() {
                            ui.text(&im_str!("{:?}", action));
                        }
                    }
                });

            Window::new(im_str!("Workers"))
                .size([300.0, 100.0], Condition::FirstUseEver)
                .build(ui, || {
                    ui.input_int3(im_str!("map pos"), &mut self.new_worker_pos)
                        .build();

                    if ui.button(im_str!("Add Worker"), [0.0, 0.0]) {
                        queue.add_world(TriggerType::AddWorker(Point3::new(
                            self.new_worker_pos[0] as u32,
                            self.new_worker_pos[1] as u32,
                            self.new_worker_pos[2] as u32,
                        )));
                    }

                    ui.separator();

                    for (entity, worker) in (&entities, &workers).join() {
                        if ui
                            .collapsing_header(&im_str!("Worker {}", entity.id()))
                            .build()
                        {
                            ui.text(&im_str!("inventory: {}", worker.inventory.len()));
                            if ui.collapsing_header(im_str!("Action Queue")).build() {
                                for action in worker.queue.iter() {
                                    ui.text(&im_str!("{:?}", action));
                                }
                            }
                        }

                        ui.separator();
                    }
                });

            Window::new(im_str!("Hover"))
                .size([300.0, 100.0], Condition::FirstUseEver)
                .build(ui, || {
                    let selected = &cursor_selected.hover_selected;
                    if let Some(pick_info) = selected {
                        let worker_label = pick_info
                            .worker
                            .and_then(|worker_id| {
                                let entity = entities.entity(worker_id);
                                workers.get(entity)
                            })
                            .and_then(|worker| Some(format!("Worker: {}", worker.to_string())))
                            .unwrap_or("Worker: N/A".to_string());
                        ui.text(worker_label);

                        let object_label = pick_info
                            .object
                            .and_then(|object_id| {
                                let entity = entities.entity(object_id);
                                objects.get(entity)
                            })
                            .and_then(|object| Some(format!("Object: {}", object.to_string())))
                            .unwrap_or("Object: N/A".to_string());
                        ui.text(object_label);

                        let terrain_label = pick_info
                            .terrain
                            .as_ref()
                            .and_then(|terrain| Some(format!("Terrain: {:?}", terrain)))
                            .unwrap_or("Terrain: N/A".to_string());
                        ui.text(terrain_label);

                        let map_pos = pick_info
                            .position
                            .and_then(|position| {
                                Some(format!(
                                    "Map Pos: ({}, {}, {})",
                                    position.x, position.y, position.z
                                ))
                            })
                            .unwrap_or("Map Pos: N/A".to_string());
                        ui.text(map_pos);

                        let mouse_pos = ui.io().mouse_pos;
                        ui.text(im_str!(
                            "Mouse Pos: ({:.1},{:.1})",
                            mouse_pos[0],
                            mouse_pos[1]
                        ));
                    }
                });
        });
    }
}
