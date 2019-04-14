use std::collections::VecDeque;

use crate::{
    actors::Actor,
    actions::Action,
    entities::MapObject,
    world::WorldUpdate,
};

#[derive(Clone)]
pub struct Worker {
    /// Queue of actions this worker has. e.g. a queue might look like the
    /// following for a worker:
    /// - MoveTo -> x, x
    /// - PerformAction(Chop) @ x,x
    ///
    /// The worker needs to MoveTo some location first before they are able
    /// to perform an action.
    pub actions: VecDeque<Action>,
    pub x: u32,
    pub y: u32,
    pub id: u32,
}

impl Actor for Worker {
    fn id(&self) -> u32 { self.id }

    fn queue(&mut self, action: &Action) {
        self.actions.push_back(action.clone());
    }

    fn tick(&mut self, _neighbors: Vec<Option<&MapObject>>) -> Option<WorldUpdate> {
        // queue actions for the new tick.
        let mut new_queue = VecDeque::new();
        // updates to broadcast to the world.
        let mut update = None;

        while let Some(action) = self.actions.pop_front() {
            match action {
                // Route worker towards a target
                Action::MoveTo(target_x, target_y) => {
                    self.x = target_x;
                    self.y = target_y;
                },
                // Perform an action.
                Action::HarvestResource(pos, resource_type, object_id) => {
                    let (target_x, target_y) = pos;
                    // Are we next to this resource? Move closer to it
                    let dist_x = (target_x as i32 - self.x as i32).abs() as u32;
                    let dist_y = (target_y as i32 - self.y as i32).abs() as u32;

                    if dist_x + dist_y <= 1 {
                        // Is the resource available nearby?
                        // Try to harvest.
                        new_queue.push_back(Action::HarvestResource(pos, resource_type.clone(), object_id));
                        update = Some(WorldUpdate {
                            target: object_id,
                            action: Action::DealDamage(10),
                        });
                    } else {
                        // Move closer
                        let mut new_x = self.x;
                        let mut new_y = self.y;
                        if dist_x > dist_y {
                            new_x += 1;
                        } else {
                            new_y += 1;
                        }

                        new_queue.push_back(Action::MoveTo(new_x, new_y));
                        new_queue.push_back(Action::HarvestResource(pos, resource_type.clone(), object_id));
                    }
                },
                _ => {}
            }
        }

        self.actions.append(&mut new_queue);
        update
    }
}