/// System to correctly handle sprite sorting order. This only needs to happen when
/// the camera view changes in anyway.
use core::{
    amethyst::{
        core::Transform,
        ecs::{prelude::Entity, Entities, Join, ReadStorage, System, Write, WriteExpect},
    },
    WorldPos,
};

use crate::game::resources::ViewShed;
use libdwarf::components::{EntityInfo, Terrain};

#[derive(Default, Debug)]
pub struct SpriteVisibility {
    pub order: Vec<Entity>,
}

struct Internal {
    pub entity: Entity,
    pub pos: WorldPos,
}

#[derive(Default)]
pub struct SpriteSortingSystem {
    filtered: Vec<Internal>,
}

impl SpriteSortingSystem {
    pub fn new() -> Self {
        Default::default()
    }
}

impl<'s> System<'s> for SpriteSortingSystem {
    type SystemData = (
        Entities<'s>,
        Write<'s, SpriteVisibility>,
        WriteExpect<'s, ViewShed>,
        ReadStorage<'s, EntityInfo>,
        ReadStorage<'s, Terrain>,
        ReadStorage<'s, Transform>,
    );

    fn run(
        &mut self,
        (entities, mut visibility, mut viewshed, entity_infos, terrains, transforms): Self::SystemData,
    ) {
        if !viewshed.needs_sort {
            return;
        }

        let top_left = viewshed.top_left;
        let bottom_right = viewshed.bottom_right;

        if top_left.is_none() || bottom_right.is_none() {
            return;
        }

        visibility.order.clear();
        self.filtered.clear();

        // Filter out non-visible entities.
        if let Some(top_left) = top_left {
            if let Some(bottom_right) = bottom_right {
                for (entity, entity_info, _, transform) in
                    (&entities, &entity_infos, &terrains, &transforms).join()
                {
                    // Not within view? Skip it
                    if (transform.translation().x < top_left.x
                        || transform.translation().x > bottom_right.x)
                        && (transform.translation().y < top_left.y
                            || transform.translation().y > bottom_right.y)
                    {
                        continue;
                    }

                    self.filtered.push(Internal {
                        entity,
                        pos: entity_info.pos,
                    });
                }
            }
        }

        self.filtered.sort_by(|a, b| {
            let ta = a.pos;
            let tb = b.pos;

            (ta.x + ta.y + ta.z)
                .partial_cmp(&(tb.x + tb.y + tb.z))
                .unwrap()
                .reverse()
        });

        visibility
            .order
            .extend(self.filtered.iter().map(|c| c.entity));

        viewshed.needs_sort = false;
    }
}
