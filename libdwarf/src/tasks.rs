use std::collections::VecDeque;

/// Specific actions for the "PerformAction" task.
#[derive(Debug, Clone, PartialEq)]
pub enum ActionType {
    HarvestResource(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    /// Idle dwarf
    Chilling,
    /// Performs <ActionType> on MapObject w/ id.
    /// e.g., Chopping wood
    PerformAction(ActionType, u32),
    /// Move to some location.
    MoveTo(u32, u32),
}

#[derive(Debug, Clone)]
pub struct Task {
    pub action: Action,
    /// The target location for an action/move_to
    pub target: Option<(u32, u32)>,
}

#[derive(Clone, Default)]
pub struct Tasks {
    pub queue: VecDeque<Task>,
}

impl Tasks {
    pub fn add(&mut self, action: Action, _priority: u32) {
        self.queue.push_back(Task {
            action,
            target: Some((9, 9)),
        });
    }

    pub fn next(&mut self) -> Option<Task> {
        return self.queue.pop_front();
    }
}