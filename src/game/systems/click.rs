use amethyst::{
    core::Transform,
    ecs::{Entities, Join, Read, ReadExpect, ReadStorage, System, WriteStorage, Write},
    input::{InputHandler, StringBindings},
    renderer::rendy::wsi::winit::MouseButton,
};

use crate::game::components::{Cursor, CursorDown, CursorSelected};
use libdwarf::{
    actions::Action,
    components::{ MapObject, MapPosition },
    resources::TaskQueue,
    Point3
};

pub struct ClickSystem;

impl<'s> System<'s> for ClickSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Cursor>,
        WriteStorage<'s, CursorDown>,
        ReadExpect<'s, CursorSelected>,
        ReadStorage<'s, MapObject>,
        ReadStorage<'s, MapPosition>,
        Read<'s, InputHandler<StringBindings>>,
        WriteStorage<'s, Transform>,
        Write<'s, TaskQueue>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut cursors,
            mut cursor_down,
            cursor_selected,
            map_objects,
            map_pos,
            input,
            mut transforms,
            mut task_queue,
        ): Self::SystemData,
    ) {
        // Capture mouse down events.
        let cursor_is_down = (&*entities, &mut cursors, !&cursor_down)
            .join()
            .next()
            .or(None);
        if let Some((entity, _, _)) = cursor_is_down {
            if input.mouse_button_is_down(MouseButton::Left) {
                // Flag cursor
                cursor_down.insert(entity, Default::default()).unwrap();
            }
        }

        // Handle clicks by only looking at the cursor w/ the CursorDown flag.
        let cursor_transform = (&*entities, &mut transforms, &mut cursors, &cursor_down)
            .join()
            .next()
            .or(None);
        if let Some((entity, _, _, _)) = cursor_transform {
            if !input.mouse_button_is_down(MouseButton::Left) {
                if let Some(pick) = &cursor_selected.hover_selected {
                    if let Some(obj_entity) = pick.object {

                        let obj_info = map_objects.get(entities.entity(obj_entity));
                        let obj_pos = map_pos.get(entities.entity(obj_entity));

                        if let Some(info) = obj_info {
                                if let Some(pos) = obj_pos {
                                println!("click! {:?}", info);
                                // Add to task queue
                                task_queue.add(Action::HarvestResource(
                                    pos.pos,
                                    String::from("tree"),
                                    String::from("wood")
                                ));
                            }
                        }
                    }
                }

                cursor_down.remove(entity);
            }
        }
    }
}
