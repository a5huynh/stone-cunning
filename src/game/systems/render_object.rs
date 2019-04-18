use amethyst::{
    core::{ transform::Transform },
    ecs::{
        Entity,
        Entities,
        Join,
        System,
        ReadExpect,
        ReadStorage,
        WriteStorage,
    },
    renderer::{
        SpriteRender,
        Transparent,
    },
};

use libdwarf::{
    entities::{ MapObject, MapPosition }
};

use crate::game::{
    sprite::SpriteSheetStorage,
    render::MapRenderer,
};

pub struct RenderObjectSystem;
impl<'a> System<'a> for RenderObjectSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, MapObject>,
        ReadStorage<'a, MapPosition>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, SpriteRender>,
        WriteStorage<'a, Transparent>,
        ReadExpect<'a, MapRenderer>,
        ReadExpect<'a, SpriteSheetStorage>,
    );

    fn run(&mut self, (
        entities,
        mut objects,
        positions,
        mut transforms,
        mut sprites,
        mut transparents,
        map_render,
        sheets,
    ): Self::SystemData) {
        // Find objects that don't have a sprite and give it one.
        let invisible: Vec<(Entity, &mut MapObject, &MapPosition, ())> = (&*entities, &mut objects, &positions, !&sprites).join().collect();
        for (entity, _, pos, _) in invisible {
             // Appply transformation
            transforms.insert(entity, map_render.place(pos.x as i32, pos.y as i32, 1.0)).unwrap();
            // Assign sprite to entity
            sprites.insert(entity, SpriteRender {
                sprite_sheet: sheets.object.clone(),
                sprite_number: 2
            }).unwrap();
            transparents.insert(entity, Transparent).unwrap();
        }

        // Remove sprites for hidden objects
        let needs_hiding: Vec<(Entity, &mut MapObject, ())> = (
            &*entities,
            &mut objects,
            !&positions
        ).join().collect();
        for (entity, _, ()) in needs_hiding {
            sprites.remove(entity);
        }
    }
}