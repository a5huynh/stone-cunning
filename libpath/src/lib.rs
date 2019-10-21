use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::IntoIterator;

use indexmap::IndexMap;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Using the information provided in terrain, find a path from <start> to <goal>
/// - `start` is the starting node we're beginning our search.
/// - `goal` is the end node we'd immediately end our search.
/// - `heuristic` returns an approximation of the cost from a given node to the goal.
/// - `find_neighbors` returns the neighboring nodes for some given node.
pub fn find_path<IN, N, FH, FN>(
    start: N,
    goal: N,
    mut heuristic: FH,
    mut find_neighbors: FN,
) -> (IndexMap<N, (usize, usize)>, Vec<N>)
where
    N: Clone + Debug + Eq + Hash,
    IN: IntoIterator<Item = (N, usize)>,
    FH: FnMut(&N) -> usize,
    FN: FnMut(&N) -> IN,
{
    // Nodes that are left to explore.
    let mut frontier = BinaryHeap::new();
    frontier.push(State {
        cost: 0,
        position: 0,
    });

    let mut parents: IndexMap<N, (usize, usize)> = IndexMap::new();
    // Node -> (parent node index, cost)
    // The parent node is which node visited this node w/ the least cost.
    parents.insert(start.clone(), (0, std::usize::MAX));

    while let Some(State { cost, position }) = frontier.pop() {
        let (node, &(_parent, _path_cost)) = parents.get_index(position).unwrap();
        if *node == goal {
            // Reconstruct path and return it.
            let mut path = Vec::new();

            let mut parent_idx = position;
            while parent_idx != 0 {
                let (node, &(parent, _)) = parents.get_index(parent_idx).unwrap();
                path.push(node.clone());
                parent_idx = parent;
            }
            return (parents, path);
        }

        for (neighbor, move_cost) in find_neighbors(&node) {
            let new_cost = cost + move_cost;

            if !parents.contains_key(&neighbor) || parents.get(&neighbor).unwrap().1 > new_cost {
                // Insert / update the current path & path cost.
                let (index, _) = parents.insert_full(neighbor.clone(), (position, new_cost));
                // Add to list of neighbors to be visited.
                frontier.push(State {
                    cost: new_cost + heuristic(&neighbor),
                    position: index,
                });
            }
        }
    }

    (parents, Vec::new())
}
