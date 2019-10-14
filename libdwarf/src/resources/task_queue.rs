use crate::actions::ActionType;
use std::collections::VecDeque;

#[derive(Default)]
pub struct TaskQueue {
    // World updates
    pub world: VecDeque<ActionType>,
    // Worker tasks
    pub worker: VecDeque<ActionType>,
}

impl TaskQueue {
    pub fn add(&mut self, action: ActionType) {
        self.worker.push_back(action.clone());
    }

    pub fn add_world(&mut self, action: ActionType) {
        self.world.push_back(action.clone());
    }
}
