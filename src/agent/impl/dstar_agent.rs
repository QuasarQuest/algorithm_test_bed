// src/agent/impl/dstar_agent.rs

use crate::agent::action::{Action, Dir};
use crate::agent::brain::{Agent, DebugInfo};
use crate::agent::components::GridPos;
use crate::agent::observation::Observation;
use crate::world::tile::Tile;
use crate::algorithm::path_planning::d_star_lite::DStarLite;

pub struct DStarAgent {
    planner: Option<DStarLite>,
    target:  Option<GridPos>,
    last_pos: Option<GridPos>,
}

impl DStarAgent {
    pub fn new() -> Self {
        Self {
            planner: None,
            target:  None,
            last_pos: None,
        }
    }

    fn direction_to(&self, from: GridPos, to: GridPos) -> Option<Dir> {
        let dx = to.x - from.x;
        let dy = to.y - from.y;
        Dir::all().iter().find(|d| d.delta() == (dx, dy)).copied()
    }
}

impl Agent for DStarAgent {
    fn name(&self) -> &str {
        "D* Lite"
    }

    fn act(&mut self, obs: &Observation<'_>) -> Action {
        // ── Step 1: Immediate Interactions ────────────────────────────────────
        let on_gold = obs.is_tile(obs.pos, Tile::Gold);
        let on_base = obs.is_tile(obs.pos, Tile::Base);

        if on_gold && !obs.gold_carried.is_full() {
            self.reset(); // We got the gold, clear the planner to find base next tick
            return Action::Pickup;
        }

        if on_base && !obs.gold_carried.is_empty() {
            self.reset(); // Dropped gold, clear the planner to find gold next tick
            return Action::Drop;
        }

        // ── Step 2: Initialize Planner if needed ──────────────────────────────
        if self.planner.is_none() {
            let mut target = None;
            if !obs.gold_carried.is_full() {
                target = obs.nearest(Tile::Gold);
            } else {
                target = obs.nearest(Tile::Base);
            }

            if let Some(goal) = target {
                let mut planner = DStarLite::new(obs.pos, goal);
                planner.compute_shortest_path();

                self.planner  = Some(planner);
                self.target   = Some(goal);
                self.last_pos = Some(obs.pos);
            }
        }

        // ── Step 3: Update Vision & Replan Dynamically ────────────────────────
        if let Some(planner) = &mut self.planner {
            // If we successfully moved last tick, tell the planner to update its start key
            if let Some(last) = self.last_pos {
                if last != obs.pos {
                    planner.update_start(obs.pos);
                    self.last_pos = Some(obs.pos);
                }
            }

            // Look around us. Are there unexpected obstacles? (Walls or other agents)
            let mut changed = false;
            for dir in Dir::all() {
                let (dx, dy) = dir.delta();
                let check_pos = GridPos::new(obs.pos.x + dx, obs.pos.y + dy);

                // NOTE: In this basic implementation, we treat moving agents as permanent
                // walls. This creates "phantom walls", which is a great educational
                // demonstration of D* Lite's static-world assumptions!
                if !obs.is_walkable(check_pos) && !planner.known_obstacles.contains(&check_pos) {
                    planner.add_obstacle(check_pos);
                    changed = true;
                }
            }

            // Only recalculate if we discovered something new!
            if changed {
                planner.compute_shortest_path();
            }

            // ── Step 4: Execute next step ─────────────────────────────────────
            if let Some(next_pos) = planner.get_next_step() {
                if let Some(dir) = self.direction_to(obs.pos, next_pos) {
                    return Action::Move(dir);
                }
            }
        }

        Action::Wait
    }

    fn debug_info(&self) -> Option<DebugInfo> {
        if let Some(planner) = &self.planner {
            Some(DebugInfo::DStarLite {
                open:      planner.open_set().iter().map(|p| (p.x, p.y)).collect(),
                obstacles: planner.known_obstacles.iter().map(|p| (p.x, p.y)).collect(),
                path:      planner.generate_path().iter().map(|p| (p.x, p.y)).collect(),
            })
        } else {
            None
        }
    }

    fn reset(&mut self) {
        self.planner  = None;
        self.target   = None;
        self.last_pos = None;
    }
}