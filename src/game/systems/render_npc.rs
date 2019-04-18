use amethyst::{
    core::transform::Transform,
    ecs::{Entities, Entity, Join, ReadExpect, ReadStorage, System, WriteStorage},
    renderer::{SpriteRender, Transparent},
};

use libdwarf::entities::{MapPosition, Worker};

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
        for (entity, _, pos, _) in invisible {
            println!("Found worker w/ no sprite");
            // Appply transformation
            transforms
                .insert(entity, map_render.place(pos.x as i32, pos.y as i32, 1.0))
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
        for (worker, transform) in (&mut workers, &mut transforms).join() {
            let new_transform = map_render.place(worker.x as i32, worker.y as i32, 1.0);
            *transform = new_transform;
        }
    }
}
