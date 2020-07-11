use core::amethyst::{
    core::transform::Transform,
    ecs::{
        prelude::{Component, NullStorage},
        Entities, Join, ReadExpect, ReadStorage, System, WriteStorage,
    },
    renderer::{SpriteRender, Transparent},
};
use specs_derive::*;

use crate::game::{resources::MapResource, sprite::SpriteSheetStorage};
use libdwarf::components::Worker;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct PathDebugComponent;

pub struct PathDebugSystem;

impl<'a> System<'a> for PathDebugSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Worker>,
        WriteStorage<'a, PathDebugComponent>,
        ReadExpect<'a, MapResource>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, SpriteRender>,
        WriteStorage<'a, Transparent>,
        ReadExpect<'a, SpriteSheetStorage>,
    );

    fn run(
        &mut self,
        (
            entities,
            workers,
            mut debug_components,
            map,
            mut transforms,
            mut sprites,
            mut transparents,
            sheets,
        ): Self::SystemData,
    ) {
        // Remove existing debug paths
        for (entity, _) in (&entities, &mut debug_components).join() {
            entities
                .delete(entity)
                .expect("Could not delete debug component");
        }

        for (worker,) in (&workers,).join() {
            // If the worker has a path, let's render it.
            if let Some(path) = &worker.current_path {
                for pos in path.iter() {
                    let entity = entities.create();
                    transforms.insert(entity, map.place(&pos, 0.0)).unwrap();

                    sprites
                        .insert(
                            entity,
                            SpriteRender {
                                sprite_sheet: sheets.object.clone(),
                                sprite_number: 4,
                            },
                        )
                        .unwrap();

                    debug_components.insert(entity, PathDebugComponent).unwrap();
                    transparents.insert(entity, Transparent).unwrap();
                }
            }
        }
    }
}
