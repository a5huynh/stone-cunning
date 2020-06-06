use core::amethyst::{
    core::transform::Transform,
    ecs::{Entities, Entity, Join, ReadExpect, ReadStorage, System, WriteStorage},
    renderer::{SpriteRender, Transparent},
};

use libdwarf::components::{EntityInfo, MapObject};

use crate::game::{resources::MapRenderer, sprite::SpriteSheetStorage};

pub struct RenderObjectSystem;
impl<'a> System<'a> for RenderObjectSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, MapObject>,
        ReadStorage<'a, EntityInfo>,
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
            mut objects,
            positions,
            mut transforms,
            mut sprites,
            mut transparents,
            map_render,
            sheets,
        ): Self::SystemData,
    ) {
        // Find objects that don't have a sprite and give it one.
        let invisible: Vec<(Entity, &mut MapObject, &EntityInfo, ())> =
            (&*entities, &mut objects, &positions, !&sprites)
                .join()
                .collect();
        for (entity, object, map_pos, _) in invisible {
            // Appply transformation
            let pos = map_pos.pos;
            transforms
                .insert(entity, map_render.place(&pos, 1.0))
                .unwrap();
            // Assign sprite to entity
            sprites
                .insert(
                    entity,
                    SpriteRender {
                        sprite_sheet: sheets.object.clone(),
                        sprite_number: object.resource_type.sprite,
                    },
                )
                .unwrap();
            transparents.insert(entity, Transparent).unwrap();
        }

        // Remove sprites for hidden objects
        let needs_hiding: Vec<(Entity, &mut MapObject, ())> =
            (&*entities, &mut objects, !&positions).join().collect();
        for (entity, _, ()) in needs_hiding {
            sprites.remove(entity);
        }
    }
}
