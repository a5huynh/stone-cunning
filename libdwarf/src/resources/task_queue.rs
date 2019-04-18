use crate::actions::Action;
use std::collections::VecDeque;

#[derive(Default)]
pub struct TaskQueue {
    // World updates
    pub world: VecDeque<Action>,
    // Worker tasks
    pub worker: VecDeque<Action>,
}

impl TaskQueue {
    pub fn add(&mut self, action: Action) {
        self.worker.push_back(action.clone());
    }

    pub fn add_world(&mut self, action: Action) {
        self.world.push_back(action.clone());
    }
}
