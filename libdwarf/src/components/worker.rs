use core::amethyst::ecs::{Component, VecStorage};
use std::collections::VecDeque;
use std::fmt;

use core::{utils::is_near, Point3};
use libterrain::Path;

use crate::{
    components::{EntityInfo, MapObject},
    planner::{Action, Condition, State},
    resources::{Map, TaskQueue},
    trigger::TriggerType,
};

#[derive(Clone)]
pub struct WorkerAction {
    pub action: Action,
    pub target: Option<u32>,
    pub target_pos: Point3<i32>,
}

impl fmt::Debug for WorkerAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Action<{}>", self.action.name)
    }
}

#[derive(Clone, Component, Debug)]
#[storage(VecStorage)]
pub struct Worker {
    pub id: u32,
    /// Energy a worker has. Each action depletes energy. One it reaches, 0
    /// it'll have to wait a couple frames before it can do something else.
    pub energy: f32,
    pub current_action: Option<WorkerAction>,
    // Current path this worker is following.
    pub current_path: Option<Path>,
    /// Queue of actions this worker has. e.g. a queue might look like the
    /// following for a worker:
    /// - MoveTo -> x, x
    /// - PerformAction(Chop) @ x,x
    ///
    /// The worker needs to MoveTo some location first before they are able
    /// to perform an action.
    pub queue: VecDeque<WorkerAction>,
    /// Worker's inventory.
    pub inventory: Vec<u32>,
}

impl Worker {
    pub fn new(id: u32) -> Self {
        Worker {
            id,
            energy: 1.0,
            current_action: None,
            current_path: None,
            queue: Default::default(),
            inventory: Default::default(),
        }
    }

    /// Adds workers current state to the planner initial state.
    pub fn add_state(&self, state: &mut State) {
        state.insert(Condition::Has("axe".to_string()), true);
    }

    //  - `unecessary_unwrap` is ignored since we are unable to do multiple
    //    if let Some(yyy) = xxx on a single line.
    //
    //  - `nonminimal_bool` disabled due to false positive.
    #[allow(clippy::unnecessary_unwrap, clippy::nonminimal_bool)]
    pub fn do_work(
        &mut self,
        tasks: &mut TaskQueue,
        map: &mut Map,
        entity_info: &mut EntityInfo,
        target_obj: Option<&MapObject>,
    ) {
        // An action will be marked as finished once all it's conditions are
        // true.
        let mut finished = true;
        if let Some(action) = &self.current_action {
            // process action post conditions
            for (condition, _value) in action.action.post.iter() {
                match condition {
                    // Attempt to destroy entity
                    Condition::Destroy(_) => {
                        // Assume target is destroyed if we don't have a ref
                        // to it.
                        if target_obj.is_some() && action.target.is_some() {
                            let target_id = action.target.as_ref().unwrap();
                            let target_obj = target_obj.unwrap();

                            if !target_obj.is_destroyed() {
                                // Queue damage to this entity
                                tasks.add_world(TriggerType::DealDamage {
                                    source: self.id,
                                    target: *target_id,
                                    damage: 10,
                                });
                                finished = false;
                            }
                        }
                    }
                    // Pickup item
                    Condition::Has(_) => {
                        // Queue picking up this resource
                        let resource = map.object_map.get(&action.target_pos).unwrap();
                        tasks.add_world(TriggerType::Take {
                            target: *resource,
                            owner: self.id,
                        });
                    }
                    // Path closer to this entity
                    Condition::Near(_) => {
                        // Does this worker have a path?
                        if self.current_path.is_none()
                            && !is_near(&entity_info.pos, &action.target_pos)
                        {
                            // If not, path from it's current position to the entity.
                            self.current_path =
                                Some(map.find_path(&entity_info.pos, &action.target_pos));
                        }

                        // Move worker to next location in path!
                        if let Some(path) = self.current_path.as_mut() {
                            if let Some(new_pt) = path.pop() {
                                if is_near(&entity_info.pos, &action.target_pos) {
                                    // Finished!
                                    finished = finished && true;
                                } else {
                                    // Move laong path.
                                    let current_pos = entity_info.pos;
                                    entity_info.pos = new_pt;
                                    map.move_worker(self.id, current_pos, new_pt);
                                    finished = false;
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        // Clear action if finished
        if finished {
            self.current_action = None;
            self.current_path = None;
        }
    }
}

impl fmt::Display for Worker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.energy)
    }
}
