use specs::{Entities, Join, System, Write, WriteStorage};

use crate::{actions::Action, components::Worker, resources::TaskQueue};

/// Assign tasks to idle workers.
pub struct AssignTaskSystem;

impl<'a> System<'a> for AssignTaskSystem {
    type SystemData = (Entities<'a>, WriteStorage<'a, Worker>, Write<'a, TaskQueue>);

    fn run(&mut self, (entities, mut workers, mut queue): Self::SystemData) {
        for (_, worker) in (&entities, &mut workers).join() {
            if worker.actions.is_empty() {
                if let Some(new_task) = queue.worker.pop_front() {
                    worker.actions.push_back(new_task.clone());
                } else {
                    worker.actions.push_back(Action::Chilling);
                }
            }
        }
    }
}
