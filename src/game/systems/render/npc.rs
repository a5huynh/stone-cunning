use core::amethyst::{
    core::transform::Transform,
    ecs::{Entities, Entity, Join, ReadExpect, ReadStorage, System, WriteStorage},
    renderer::SpriteRender,
};

use libdwarf::components::{EntityInfo, Worker};

use crate::game::{resources::MapRenderer, sprite::SpriteSheetStorage};

const NPC_Z_OFFSET: f32 = 1.0;

pub struct RenderNPCSystem;
impl<'a> System<'a> for RenderNPCSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Worker>,
        ReadStorage<'a, EntityInfo>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, SpriteRender>,
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
            map_render,
            sheets,
        ): Self::SystemData,
    ) {
        // Find objects that don't have a sprite and give it one.
        let invisible: Vec<(Entity, &mut Worker, &EntityInfo, ())> =
            (&*entities, &mut workers, &positions, !&sprites)
                .join()
                .collect();

        for (entity, _, map_pos, _) in invisible {
            // Apply transformation
            let pos = map_pos.pos;
            transforms
                .insert(entity, map_render.place(&pos, NPC_Z_OFFSET))
                .unwrap();

            sprites
                .insert(
                    entity,
                    SpriteRender {
                        sprite_sheet: sheets.npc.clone(),
                        sprite_number: 0,
                    },
                )
                .unwrap();
        }

        // Update object positions
        for (_, map_pos, transform) in (&mut workers, &positions, &mut transforms).join() {
            let pos = map_pos.pos;
            let new_transform = map_render.place(&pos, NPC_Z_OFFSET);
            transform.set_translation_xyz(
                new_transform.translation().x,
                new_transform.translation().y,
                new_transform.translation().z,
            );
        }
    }
}
