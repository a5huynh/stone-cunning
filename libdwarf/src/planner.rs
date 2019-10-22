use ron::de::from_reader;
use serde::Deserialize;
use std::fs::File;

use libpath::find_path;

use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Deserialize, Eq, Hash, PartialEq)]
pub enum Condition {
    // Agent has destroyed entity
    Destroy(String),
    // Agent has `x` in their inventory.
    Has(String),
    // Agent has been tasked to do something.
    HasJob(String),
    // Agent has an item `x` w/ some property `y`
    HasProperty(String, String),
    // Enemy is killed
    Alive(String),
    // Agent is within `x` of some entity
    Near(String),
    // Agent can see `x`
    Visible(String),
}

pub type State = HashMap<Condition, bool>;

#[derive(Clone, Deserialize, Eq)]
pub struct Action {
    pub name: String,
    pub cost: usize,
    pub pre: State,
    pub post: State,
}

impl fmt::Debug for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Action<{} - {}>", self.name, self.cost)
    }
}

impl PartialEq for Action {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(Deserialize)]
pub struct Planner {
    /// List of all actions
    actions: Vec<Action>,
}

impl fmt::Debug for Planner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Planner<{:?}>",
            self.actions
                .iter()
                .map(|x| x.name.clone())
                .collect::<Vec<String>>()
        )
    }
}

impl Planner {
    pub fn new() -> Self {
        Planner {
            actions: Default::default(),
        }
    }

    pub fn load(input_path: &str) -> Self {
        let f = File::open(input_path).expect("Failed opening actions");
        let planner: Planner = match from_reader(f) {
            Ok(x) => x,
            Err(e) => {
                println!("Failed to load planner actions: {}", e);
                std::process::exit(1);
            }
        };
        planner
    }

    pub fn heuristic(a: &PlanNode, b: &PlanNode) -> usize {
        a.num_mismatched(&b.state)
    }

    pub fn add_action(
        &mut self,
        name: String,
        cost: usize,
        pre: Vec<(Condition, bool)>,
        post: Vec<(Condition, bool)>,
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

    pub fn next_actions<'a>(&self, plan: &PlanNode) -> Vec<(PlanNode, usize)> {
        let mut potential = Vec::new();

        for action in self.actions.iter() {
            // Does the current state match the pre conditions?
            if plan.num_mismatched(&action.pre) == 0 {
                let affects = &action.post;
                let new_state = plan.apply(affects);

                potential.push((
                    PlanNode {
                        last_action: Some(&action),
                        state: new_state.clone(),
                    },
                    action.cost,
                ));
            }
        }

        potential
    }

    pub fn plan(&self, initial: &State, end: &State) -> Vec<Action> {
        let start = PlanNode::new(initial);
        let goal = PlanNode::new(end);

        let (_, plan) = find_path(
            start,
            goal.clone(),
            |node| Planner::heuristic(&node, &goal),
            |node| self.next_actions(node),
        );

        let mut planned_actions = Vec::new();
        for node in plan.iter() {
            planned_actions.push(node.last_action.unwrap().clone());
        }

        planned_actions
    }
}

#[derive(Clone, Eq)]
pub struct PlanNode<'a> {
    pub last_action: Option<&'a Action>,
    pub state: State,
}

impl fmt::Debug for PlanNode<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "PlanNode<{:} - {:?}>",
            self.last_action.unwrap().name,
            self.state
                .iter()
                .map(|(key, &value)| {
                    let string = format!("{:?}", key);
                    if value {
                        return string.to_uppercase();
                    }

                    string
                })
                .collect::<Vec<String>>()
        )
    }
}

impl Hash for PlanNode<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if let Some(action) = self.last_action {
            action.name.hash(state);
        }

        for (key, value) in self.state.iter() {
            key.hash(state);
            value.hash(state);
        }
    }
}

impl PartialEq for PlanNode<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.num_mismatched(&other.state) == 0
    }
}

impl<'a> PlanNode<'a> {
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

    pub fn apply(&self, state: &State) -> State {
        let mut new_state = self.state.clone();

        for (name, value) in state.iter() {
            new_state.insert(name.clone(), value.clone());
        }

        new_state
    }
}
