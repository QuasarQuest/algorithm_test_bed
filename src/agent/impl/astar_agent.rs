// src/agent/impl/astar_agent.rs

use crate::agent::action::{Action, Dir};
use crate::agent::brain::{Agent, DebugInfo};
use crate::agent::components::GridPos;
use crate::agent::observation::Observation;
use crate::world::tile::Tile;
use crate::algorithm::path_planning::a_star::compute_path;

pub struct AStarAgent {
    path:         Vec<GridPos>,
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

    fn act(&mut self, obs: &Observation<'_>) -> Action {
        // ── Step 0: Closed-Loop Confirmation ──────────────────────────────────
        // If our physical position matches the next planned step, the move succeeded!
        if let Some(&next_step) = self.path.first() {
            if obs.pos == next_step {
                self.path.remove(0);
            }
        }

        // ── Step 1: Immediate interactions ────────────────────────────────────
        let on_gold = obs.is_tile(obs.pos, Tile::Gold);
        let on_base = obs.is_tile(obs.pos, Tile::Base);

        if on_gold && !obs.gold_carried.is_full() {
            self.path.clear();
            return Action::Pickup;
        }

        if on_base && !obs.gold_carried.is_empty() {
            self.path.clear();
            return Action::Drop;
        }

        // ── Step 2: Validate current path ─────────────────────────────────────
        if let Some(&next_step) = self.path.first() {
            // If another agent walked into our path, abandon it and replan immediately.
            if !obs.is_walkable(next_step) {
                self.path.clear();
            }
        }

        // ── Step 3: Generate new path if we don't have one ────────────────────
        if self.path.is_empty() {
            let mut target = None;

            if !obs.gold_carried.is_full() {
                target = obs.nearest(Tile::Gold);
            }

            if target.is_none() && !obs.gold_carried.is_empty() {
                target = obs.nearest(Tile::Base);
            }

            if let Some(goal) = target {
                let result = compute_path(obs.pos, goal, |pos| {
                    if !obs.is_walkable(pos) && pos != goal {
                        return false;
                    }
                    if obs.is_tile(pos, Tile::Base) && pos != goal {
                        return false;
                    }
                    true
                });

                self.path         = result.path;
                self.debug_closed = result.closed_set.into_iter().collect();
                self.debug_open   = result.open_set;
            }
        }

        // ── Step 4: Execute next step ─────────────────────────────────────────
        if let Some(&next_pos) = self.path.first() {
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