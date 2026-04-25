// src/algorithm/path_planning/a_star.rs

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use crate::agent::action::Dir;
use crate::agent::components::GridPos;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    pos: GridPos,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse order for min-heap
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct AStarResult {
    pub path:       Vec<GridPos>,
    pub closed_set: HashSet<GridPos>,
    pub open_set:   Vec<GridPos>,
}

/// Computes an A* path from start to goal.
/// Uses a closure for walkability to remain fully decoupled from the simulation state.
pub fn compute_path<F>(start: GridPos, goal: GridPos, is_walkable: F) -> AStarResult
where
    F: Fn(GridPos) -> bool,
{
    let mut frontier = BinaryHeap::new();
    let mut came_from: HashMap<GridPos, GridPos> = HashMap::new();
    let mut cost_so_far: HashMap<GridPos, i32> = HashMap::new();
    let mut closed_set = HashSet::new();

    frontier.push(State { cost: 0, pos: start });
    cost_so_far.insert(start, 0);

    while let Some(State { pos: current, .. }) = frontier.pop() {
        closed_set.insert(current);

        if current == goal {
            break;
        }

        for dir in Dir::all() {
            let (dx, dy) = dir.delta();
            let next = GridPos::new(current.x + dx, current.y + dy);

            // Ensure the tile is walkable, or it is our final target
            if !is_walkable(next) && next != goal {
                continue;
            }

            let new_cost = cost_so_far[&current] + 1;

            if !cost_so_far.contains_key(&next) || new_cost < cost_so_far[&next] {
                cost_so_far.insert(next, new_cost);
                // Chebyshev distance heuristic: max(|dx|, |dy|)
                let priority = new_cost + (goal.x - next.x).abs().max((goal.y - next.y).abs());

                frontier.push(State { cost: priority, pos: next });
                came_from.insert(next, current);
            }
        }
    }

    let open_set = frontier.into_iter().map(|s| s.pos).collect();
    let mut path = Vec::new();

    if came_from.contains_key(&goal) {
        let mut current = goal;
        while current != start {
            path.push(current);
            current = came_from[&current];
        }
        path.reverse();
    }

    AStarResult {
        path,
        closed_set,
        open_set,
    }
}