use std::collections::VecDeque;

use crate::{
    tasks::{ Action, Task }
};

pub trait Actor {
    fn tick(&mut self);
}

#[derive(Clone)]
pub struct Worker {
    /// Current state
    pub current_action: Action,
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
}

impl Worker {
    pub fn queue_task(&mut self, task: Task) {
        if let Some((x, y)) = task.target {
            self.actions.push_back(Action::MoveTo(x, y));
        }

        self.actions.push_back(task.action);
    }

    /// Mark the current action as finished and pop the next action.
    pub fn finish_action(&mut self) {
        if let Some(action) = self.actions.pop_front() {
            self.current_action = action;
        } else {
            self.current_action = Action::Chilling;
        }
    }
}

impl Actor for Worker {
    fn tick(&mut self) {
        if self.current_action == Action::Chilling {
            if let Some(action) = self.actions.pop_front() {
                self.current_action = action;
            }
        }

        match &self.current_action {
            // Route worker towards a target
            Action::MoveTo(target_x, target_y) => {
                // Move closer to target
                let (dist_x, dist_y) = (target_x - self.x, target_y - self.y);
                if dist_x > dist_y {
                    self.x += 1;
                } else {
                    self.y += 1;
                }

                // Mark action as finished if the next position hits the target.
                if self.x == *target_x && self.y == *target_y {
                    self.finish_action();
                }
            },
            // Perform an action.
            Action::PerformAction(action_type, object_id) => {
                self.finish_action();
            },
            _ => {}
        }
    }
}