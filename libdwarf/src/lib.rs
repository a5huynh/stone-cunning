pub mod world;
pub mod tasks;

use tasks::Action;
use world::World;

/// Move the world forward 1 tick
pub fn tick(world: &mut World) {
    // Loop through each worker
    let workers = &mut world.workers;
    let objects = &mut world.objects;

    for worker in workers.iter_mut() {
        // Assign tasks to idle workers.
        match &worker.current_action {
            Action::Chilling => {
                // Check for queued jobs
                if let Some(new_task) = world.tasks.next() {
                    worker.actions.push_back(new_task.action);
                    if let Some(target) = new_task.target {
                        worker.actions.push_front(Action::MoveTo(target));
                    }
                }

                // Tell worker to do next action if they have queued
                // actions
                if let Some(action) = worker.actions.pop_front() {
                    worker.current_action = action;
                }
            },
            _ => {}
        }

        // Perform action
        match &worker.current_action {
            // Route worker towards a target
            Action::MoveTo(target) => {
                // Is worker at target?
                if worker.x == target.0 && worker.y == target.1 {
                    worker.finish_action();
                } else {
                    // Otherwise move closer to target
                    let (dist_x, dist_y) = (target.0 - worker.x, target.1 - worker.y);
                    if dist_x > dist_y {
                        worker.x += 1;
                    } else {
                        worker.y += 1;
                    }
                }
            },
            // Perform an action.
            Action::PerformAction(_action) => {
                // Find the appropriate object nearby to interact with.
                let object = objects.get(&(worker.x, worker.y));
                if let Some(object) = object {
                    // Process action
                    worker.finish_action();
                } else {
                    // No object here. wtf?
                }
            },
            _ => {}
        }
    }
}