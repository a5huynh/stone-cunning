/// System to correctly handle sprite sorting order. This only needs to happen when
/// the camera view changes in anyway.
use core::amethyst::{
    core::Transform,
    ecs::{prelude::Entity, Entities, Join, ReadStorage, System, Write, WriteExpect},
    renderer::sprite::SpriteRender,
};
use core::Vector3;

use crate::game::resources::ViewShed;

#[derive(Default, Debug)]
pub struct SpriteVisibility {
    pub order: Vec<Entity>,
}

struct Internal {
    pub entity: Entity,
    pub pos: Vector3<f32>,
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
        ReadStorage<'s, SpriteRender>,
        ReadStorage<'s, Transform>,
    );

    fn run(
        &mut self,
        (entities, mut visibility, mut viewshed, sprites, transforms): Self::SystemData,
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
                for (entity, _, transform) in (&entities, &sprites, &transforms).join() {
                    let pos = transform.translation();
                    // Not within view? Skip it
                    if (pos.x < top_left.x || pos.x > bottom_right.x)
                        && (pos.y < top_left.y || pos.y > bottom_right.y)
                    {
                        continue;
                    }

                    self.filtered.push(Internal {
                        entity,
                        pos: Vector3::new(pos.x, pos.y, pos.z),
                    });
                }
            }
        }

        self.filtered.sort_by(|a, b| {
            let ta = a.pos;
            let tb = b.pos;

            ta.z.partial_cmp(&tb.z).unwrap()
        });

        visibility
            .order
            .extend(self.filtered.iter().map(|c| c.entity));

        viewshed.needs_sort = false;
    }
}
