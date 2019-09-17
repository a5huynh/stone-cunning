use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

use libterrain::{Point3, TerrainChunk};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct State {
    cost: usize,
    position: Point3<u32>,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.x.cmp(&other.position.x))
            .then_with(|| self.position.y.cmp(&other.position.y))
            .then_with(|| self.position.z.cmp(&other.position.z))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn heuristic(a: Point3<u32>, b: Point3<u32>) -> usize {
    (a.x as i32 - b.x as i32).abs() as usize
        + (a.y as i32 - b.y as i32).abs() as usize
        + (a.z as i32 - b.z as i32).abs() as usize
}

pub fn dijkstra(
    terrain: &TerrainChunk,
    start: Point3<u32>,
    goal: Point3<u32>,
) -> (HashMap<Point3<u32>, Point3<u32>>, HashSet<Point3<u32>>) {
    let mut frontier = BinaryHeap::new();
    frontier.push(State {
        cost: 0,
        position: start,
    });

    let mut came_from = HashMap::new();
    came_from.insert(start, start);

    let mut cost_so_far = HashMap::new();
    cost_so_far.insert(start, 0);

    while let Some(State { cost, position }) = frontier.pop() {
        if position == goal {
            break;
        }

        for neighbor in terrain.neighbors(&position) {
            let new_cost = cost + 1;

            if !cost_so_far.contains_key(&neighbor) || new_cost < cost_so_far[&neighbor] {
                cost_so_far.insert(neighbor, new_cost);
                frontier.push(State {
                    cost: new_cost + heuristic(goal, neighbor),
                    position: neighbor,
                });
                came_from.insert(neighbor, position);
            }
        }
    }

    // Reconstruct path
    let mut path = HashSet::new();
    let mut current = goal;
    while current != start {
        path.insert(current);
        current = came_from[&current];
    }
    path.insert(start);

    (came_from, path)
}
