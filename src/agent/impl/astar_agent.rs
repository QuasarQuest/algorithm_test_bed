// src/agent/impl/astar_agent.rs

use crate::agent::action::{Action, Dir};
use crate::agent::brain::{Agent, DebugInfo};
use crate::agent::components::GridPos;
use crate::agent::observation::Observation;
use crate::world::tile::Tile;
use crate::algorithm::path_planning::a_star::compute_path;

pub struct AStarAgent {
    /// The currently planned sequence of steps to the target.
    path: Vec<GridPos>,

    // Stored for the viz overlay
    debug_open:   Vec<GridPos>,
    debug_closed: Vec<GridPos>,
}

impl AStarAgent {
    pub fn new() -> Self {
        Self {
            path:         Vec::new(),
            debug_open:   Vec::new(),
            debug_closed: Vec::new(),
        }
    }

    /// Determines the direction from one adjacent cell to another.
    fn direction_to(&self, from: GridPos, to: GridPos) -> Option<Dir> {
        let dx = to.x - from.x;
        let dy = to.y - from.y;
        Dir::all().iter().find(|d| d.delta() == (dx, dy)).copied()
    }
}

impl Agent for AStarAgent {
    fn name(&self) -> &str {
        "A* Search"
    }

    fn act(&mut self, obs: &Observation) -> Action {
        // 1. Immediate interactions
        let on_gold = obs.visible_cells.iter().any(|c| c.pos == obs.pos && c.tile == Tile::Gold);
        let on_base = obs.visible_cells.iter().any(|c| c.pos == obs.pos && c.tile == Tile::Base);

        if on_gold && !obs.gold_carried.is_full() {
            self.path.clear();
            return Action::Pickup;
        }

        if on_base && !obs.gold_carried.is_empty() {
            self.path.clear();
            return Action::Drop;
        }

        // 2. Validate current path
        if let Some(&next_step) = self.path.first() {
            if !obs.is_walkable(next_step) && next_step != obs.nearest(Tile::Gold).unwrap_or(obs.pos) {
                self.path.clear();
            }
        }

        // 3. Generate new path if we don't have one
        if self.path.is_empty() {
            let target = if !obs.gold_carried.is_full() {
                obs.nearest(Tile::Gold)
            } else {
                obs.nearest(Tile::Base)
            };

            if let Some(goal) = target {
                // Call the pure algorithm module, passing a closure for visibility
                let result = compute_path(obs.pos, goal, |pos| obs.is_walkable(pos));

                self.path         = result.path;
                self.debug_closed = result.closed_set.into_iter().collect();
                self.debug_open   = result.open_set;
            }
        }

        // 4. Execute the next step in the path
        if !self.path.is_empty() {
            let next_pos = self.path.remove(0);
            if let Some(dir) = self.direction_to(obs.pos, next_pos) {
                return Action::Move(dir);
            }
        }

        Action::Wait
    }

    fn debug_info(&self) -> Option<DebugInfo> {
        Some(DebugInfo::AStar {
            open:   self.debug_open.iter().map(|p| (p.x, p.y)).collect(),
            closed: self.debug_closed.iter().map(|p| (p.x, p.y)).collect(),
            path:   self.path.iter().map(|p| (p.x, p.y)).collect(),
        })
    }

    fn reset(&mut self) {
        self.path.clear();
        self.debug_open.clear();
        self.debug_closed.clear();
    }
}