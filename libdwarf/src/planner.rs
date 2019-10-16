use libpath::find_path;

use std::collections::BTreeMap;
use std::fmt;
use std::hash::{Hash, Hasher};

pub type State = BTreeMap<String, bool>;

pub struct Action {
    pub name: String,
    pub cost: usize,
    pub pre: State,
    pub post: State,
}

pub struct Planner {
    /// List of all actions
    actions: Vec<Action>,
}

impl fmt::Debug for Planner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Planner<{:?}>",
            self.actions.iter().map(|x| x.name.clone()).collect::<Vec<String>>()
        )
    }
}

impl Planner {
    pub fn new() -> Planner {
        Planner { actions: Default::default() }
    }

    pub fn heuristic(a: &PlanNode, b: &PlanNode) -> usize {
        a.num_mismatched(&b.state)
    }

    pub fn add_action(
        &mut self,
        name: String,
        cost: usize,
        pre: Vec<(String, bool)>,
        post: Vec<(String, bool)>,
    ) {
        let mut action = Action {
            name,
            cost,
            pre: State::new(),
            post: State::new(),
        };

        for (atom, value) in pre {
            action.pre.insert(atom, value);
        }

        for (atom, value) in post {
            action.post.insert(atom, value);
        }

        self.actions.push(action);
    }

    pub fn next_actions(&self, plan: &PlanNode) -> Vec<(PlanNode, usize)> {
        let mut potential = Vec::new();

        for action in self.actions.iter() {
            // Does the current state match the pre conditions?
            if plan.num_mismatched(&action.pre) == 0 {
                potential.push((plan.apply(&action), action.cost));
            }
        }

        potential
    }

    pub fn plan(&self, initial: &State, end: &State) -> Vec<PlanNode> {
        let start = PlanNode::new(initial);
        let goal = PlanNode::new(end);

        let (_, plan) = find_path(
            start,
            goal.clone(),
            |node| Planner::heuristic(&node, &goal),
            |node| self.next_actions(node),
        );

        plan
    }
}

#[derive(Clone, Eq)]
pub struct PlanNode {
    pub last_action: Option<String>,
    pub state: State,
}

impl fmt::Debug for PlanNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PlanNode<{:?} - {:?}>",
            self.last_action,
            self.state
                .iter()
                .map(|(key, &value)| {
                    if value {
                        return key.clone().to_uppercase();
                    }

                    key.clone()
                })
                .collect::<Vec<String>>()
        )
    }
}

impl Hash for PlanNode {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if let Some(action) = &self.last_action {
            action.hash(state);
        }

        for (key, value) in self.state.iter() {
            key.hash(state);
            value.hash(state);
        }
    }
}

impl PartialEq for PlanNode {
    fn eq(&self, other: &Self) -> bool {
        self.num_mismatched(&other.state) == 0
    }
}

impl PlanNode {
    pub fn new(initial_state: &State) -> PlanNode {
        PlanNode {
            last_action: None,
            state: initial_state.clone(),
        }
    }

    pub fn num_mismatched(&self, state: &State) -> usize {
        let mut count: usize = 0;
        for (name, target_value) in state.iter() {
            if let Some(current_value) = self.state.get(name) {
                if current_value != target_value {
                    count += 1
                }
            } else {
                count += 1;
            }
        }

        count
    }

    pub fn apply(&self, action: &Action) -> PlanNode {
        let mut new_node = PlanNode {
            last_action: Some(action.name.clone()),
            state: self.state.clone(),
        };

        for (name, value) in &action.post {
            new_node.state.insert(name.clone(), value.clone());
        }

        new_node
    }
}
