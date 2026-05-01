// src/agent/impl/dstar_agent.rs

use bevy::prelude::*;
use crate::agent::action::{Action, Dir};
use crate::agent::brain::Agent;
use crate::agent::components::GridPos;
use crate::agent::observation::Observation;
use crate::agent::debug::DebugDraw;
use crate::world::tile::Tile;
use crate::algorithm::path_planning::d_star_lite::DStarLite;
use crate::viz::grid_offset::GridOffset;
use crate::config;

pub struct DStarAgent {
    planner: Option<DStarLite>,
    last_pos: Option<GridPos>, // We only need to remember our last position
}

impl DStarAgent {
    pub fn new() -> Self {
        Self { planner: None, last_pos: None }
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
        let on_gold = obs.is_tile(obs.pos, Tile::Gold);
        let on_base = obs.is_tile(obs.pos, Tile::Base);

        if on_gold && !obs.gold_carried.is_full() {
            self.reset();
            return Action::Pickup;
        }

        if on_base && !obs.gold_carried.is_empty() {
            self.reset();
            return Action::Drop;
        }

        if self.planner.is_none() {
            // FIX: Idiomatic conditional assignment (no unused 'None' state)
            let target = if !obs.gold_carried.is_full() {
                obs.nearest(Tile::Gold)
            } else {
                obs.nearest(Tile::Base)
            };

            if let Some(goal) = target {
                let mut planner = DStarLite::new(obs.pos, goal);
                planner.compute_shortest_path();
                self.planner  = Some(planner);
                self.last_pos = Some(obs.pos);
            }
        }

        if let Some(planner) = &mut self.planner {
            if let Some(last) = self.last_pos {
                if last != obs.pos {
                    planner.update_start(obs.pos);
                    self.last_pos = Some(obs.pos);
                }
            }

            let mut changed = false;
            for dir in Dir::all() {
                let (dx, dy) = dir.delta();
                let check_pos = GridPos::new(obs.pos.x + dx, obs.pos.y + dy);

                if !obs.is_walkable(check_pos) && !planner.known_obstacles.contains(&check_pos) {
                    planner.add_obstacle(check_pos);
                    changed = true;
                }
            }

            if changed { planner.compute_shortest_path(); }

            if let Some(next_pos) = planner.get_next_step() {
                if let Some(dir) = self.direction_to(obs.pos, next_pos) {
                    return Action::Move(dir);
                }
            }
        }
        Action::Wait
    }

    fn debug_draw(&self) -> Option<Box<dyn DebugDraw>> {
        if let Some(planner) = &self.planner {
            Some(Box::new(DStarDebugState {
                open:      planner.open_set().iter().copied().collect(),
                obstacles: planner.known_obstacles.iter().copied().collect(),
                path:      planner.generate_path(),
            }))
        } else {
            None
        }
    }

    fn reset(&mut self) {
        self.planner  = None;
        self.last_pos = None;
    }
}

// ── The algorithm's private drawing logic ─────────────────────────────────────

pub struct DStarDebugState {
    open: Vec<GridPos>,
    obstacles: Vec<GridPos>,
    path: Vec<GridPos>,
}

impl DebugDraw for DStarDebugState {
    fn draw(&self, pos: GridPos, gizmos: &mut Gizmos, offset: &GridOffset) {
        let half = config::TILE_SIZE * 0.45;

        for p in &self.obstacles {
            let c = offset.world_pos(p.x, p.y);
            gizmos.rect_2d(Isometry2d::from_translation(c), Vec2::splat(half), Color::srgba(1.0, 0.10, 0.10, 0.6));
        }

        for p in &self.open {
            let c = offset.world_pos(p.x, p.y);
            gizmos.rect_2d(Isometry2d::from_translation(c), Vec2::splat(half), Color::srgba(0.70, 0.20, 0.95, 0.15));
        }

        if !self.path.is_empty() {
            let mut pts = Vec::with_capacity(self.path.len() + 1);
            pts.push(offset.world_pos(pos.x, pos.y));
            for p in &self.path {
                pts.push(offset.world_pos(p.x, p.y));
            }
            gizmos.linestrip_2d(pts, Color::srgb(0.85, 0.30, 1.0));
        }
    }
}