use amethyst::{
    core::transform::Transform,
    ecs::{Entities, Entity, Join, ReadExpect, ReadStorage, System, WriteStorage},
    renderer::{SpriteRender, Transparent},
};

use libdwarf::components::{MapPosition, Worker};

use crate::game::{render::MapRenderer, sprite::SpriteSheetStorage};

pub struct RenderNPCSystem;
impl<'a> System<'a> for RenderNPCSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Worker>,
        ReadStorage<'a, MapPosition>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, SpriteRender>,
        WriteStorage<'a, Transparent>,
        ReadExpect<'a, MapRenderer>,
        ReadExpect<'a, SpriteSheetStorage>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut workers,
            positions,
            mut transforms,
            mut sprites,
            mut transparents,
            map_render,
            sheets,
        ): Self::SystemData,
    ) {
        // Find objects that don't have a sprite and give it one.
        let invisible: Vec<(Entity, &mut Worker, &MapPosition, ())> =
            (&*entities, &mut workers, &positions, !&sprites)
                .join()
                .collect();
        for (entity, _, map_pos, _) in invisible {
            // Appply transformation
            let pos = map_pos.pos;
            transforms
                .insert(
                    entity,
                    map_render.place(pos.x as i32, pos.y as i32, pos.z as i32, 1.0),
                )
                .unwrap();
            // Assign sprite to entity
            sprites
                .insert(
                    entity,
                    SpriteRender {
                        sprite_sheet: sheets.npc.clone(),
                        sprite_number: 0,
                    },
                )
                .unwrap();
            transparents.insert(entity, Transparent).unwrap();
        }

        // Update object positions
        for (_, map_pos, transform) in (&mut workers, &positions, &mut transforms).join() {
            let pos = map_pos.pos;
            let new_transform = map_render.place(pos.x as i32, pos.y as i32, pos.z as i32, 1.0);
            transform.set_translation_xyz(
                new_transform.translation().x,
                new_transform.translation().y,
                new_transform.translation().z,
            );
        }
    }
}
