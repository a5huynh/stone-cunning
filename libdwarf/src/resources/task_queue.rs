use crate::trigger::TriggerType;
use std::collections::VecDeque;

#[derive(Default)]
pub struct TaskQueue {
    // World updates
    pub world: VecDeque<TriggerType>,
    // Worker tasks
    pub worker: VecDeque<TriggerType>,
}

impl TaskQueue {
    pub fn add(&mut self, action: TriggerType) {
        self.worker.push_back(action);
    }

    pub fn add_world(&mut self, action: TriggerType) {
        self.world.push_back(action);
    }
}
