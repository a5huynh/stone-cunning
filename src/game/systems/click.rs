use amethyst::{
    core::Transform,
    ecs::{Entities, Join, Read, ReadExpect, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::rendy::wsi::winit::MouseButton,
};

use crate::game::entity::{Cursor, CursorDown, CursorSelected};
use libdwarf::entities::MapObject;

pub struct ClickSystem;

impl<'s> System<'s> for ClickSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Cursor>,
        WriteStorage<'s, CursorDown>,
        ReadExpect<'s, CursorSelected>,
        ReadStorage<'s, MapObject>,
        Read<'s, InputHandler<StringBindings>>,
        WriteStorage<'s, Transform>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut cursors,
            mut cursor_down,
            cursor_selected,
            map_objects,
            input,
            mut transforms,
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
                        println!("click! {:?}", obj_info);
                    }
                }

                cursor_down.remove(entity);
            }
        }
    }
}
