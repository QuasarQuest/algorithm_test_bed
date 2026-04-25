// src/algorithm/path_planning/a_star.rs

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use crate::agent::action::Dir;
use crate::agent::components::GridPos;

// ── Cost constants ────────────────────────────────────────────────────────────
// Cardinal moves cost 10, diagonals cost 14 (~10*sqrt(2)).
// This integer approximation gives A* a strong preference for straight paths
// and eliminates zigzag artefacts that appear when all moves cost the same.

const CARDINAL:  i32 = 10;
const DIAGONAL:  i32 = 14;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    pos:  GridPos,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // min-heap
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
/// Uses octile distance heuristic and weighted move costs to produce smooth,
/// non-zigzag paths on 8-directional grids.
pub fn compute_path<F>(start: GridPos, goal: GridPos, is_walkable: F) -> AStarResult
where
    F: Fn(GridPos) -> bool,
{
    let mut frontier    = BinaryHeap::new();
    let mut came_from:  HashMap<GridPos, GridPos> = HashMap::new();
    let mut cost_so_far: HashMap<GridPos, i32>    = HashMap::new();
    let mut closed_set  = HashSet::new();

    frontier.push(State { cost: 0, pos: start });
    cost_so_far.insert(start, 0);

    while let Some(State { pos: current, .. }) = frontier.pop() {
        if closed_set.contains(&current) {
            continue; // skip stale entries
        }
        closed_set.insert(current);

        if current == goal {
            break;
        }

        // Dereference the Dir enum to use its methods
        for &dir in Dir::all() {
            let (dx, dy)  = dir.delta();
            let next      = GridPos::new(current.x + dx, current.y + dy);

            if !is_walkable(next) && next != goal {
                continue;
            }

            // ── CORNER CUTTING PREVENTION ─────────────────────────────────────
            // Use the semantic `is_diagonal` method from Action::Dir
            if dir.is_diagonal() {
                let check1 = GridPos::new(current.x + dx, current.y);
                let check2 = GridPos::new(current.x, current.y + dy);

                // If either adjacent tile is a wall, diagonal movement is blocked
                if !is_walkable(check1) || !is_walkable(check2) {
                    continue;
                }
            }
            // ──────────────────────────────────────────────────────────────────

            // Use the semantic method here as well!
            let step_cost = if dir.is_diagonal() { DIAGONAL } else { CARDINAL };
            let new_cost  = cost_so_far[&current] + step_cost;

            if !cost_so_far.contains_key(&next) || new_cost < cost_so_far[&next] {
                cost_so_far.insert(next, new_cost);

                // Octile distance heuristic
                let h = octile(next, goal);

                frontier.push(State { cost: new_cost + h, pos: next });
                came_from.insert(next, current);
            }
        }
    }

    // Reconstruct path
    let mut path = Vec::new();
    if came_from.contains_key(&goal) || start == goal {
        let mut current = goal;
        while current != start {
            path.push(current);
            if let Some(&prev) = came_from.get(&current) {
                current = prev;
            } else {
                break;
            }
        }
        path.reverse();
    }

    let open_set = frontier.into_iter().map(|s| s.pos).collect();

    AStarResult { path, closed_set, open_set }
}

/// Octile distance — the correct heuristic for 8-directional movement
/// with the CARDINAL/DIAGONAL cost model.
///
///   h = CARDINAL * (dx + dy) + (DIAGONAL - 2*CARDINAL) * min(dx, dy)
///     = 10*(dx+dy) - 6*min(dx,dy)
///
/// This is admissible: it never overestimates the true cost.
#[inline]
fn octile(from: GridPos, to: GridPos) -> i32 {
    let dx = (to.x - from.x).abs();
    let dy = (to.y - from.y).abs();
    CARDINAL * (dx + dy) + (DIAGONAL - 2 * CARDINAL) * dx.min(dy)
}