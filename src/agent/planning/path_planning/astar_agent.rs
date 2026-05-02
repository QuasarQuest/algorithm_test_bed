// src/agent/planning/path_planning/astar_agent.rs

use bevy::prelude::Color;
use crate::agent::action::{Action, Dir};
use crate::agent::brain::Agent;
use crate::agent::components::GridPos;
use crate::agent::observation::Observation;
use crate::agent::debug::{DebugDraw, DebugLine, DebugRect}; // <-- Clean imports
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
    fn name(&self) -> &str { "A* Search" }

    fn act(&mut self, obs: &Observation<'_>) -> Action {
        if let Some(&next_step) = self.path.first() {
            if obs.pos == next_step { self.path.remove(0); }
        }

        if obs.is_tile(obs.pos, Tile::Gold) && !obs.gold_carried.is_full() {
            self.path.clear();
            return Action::Pickup;
        }
        if obs.is_tile(obs.pos, Tile::Base) && !obs.gold_carried.is_empty() {
            self.path.clear();
            return Action::Drop;
        }

        if let Some(&next_step) = self.path.first() {
            if !obs.is_walkable(next_step) { self.path.clear(); }
        }

        if self.path.is_empty() {
            let target =
                if !obs.gold_carried.is_full() { obs.nearest(Tile::Gold) } else { None }
                    .or_else(|| if !obs.gold_carried.is_empty() { obs.nearest(Tile::Base) } else { None });

            if let Some(goal) = target {
                let result = compute_path(obs.pos, goal, |pos| {
                    if !obs.is_walkable(pos) && pos != goal { return false; }
                    if obs.is_tile(pos, Tile::Base) && pos != goal { return false; }
                    true
                });
                self.path         = result.path;
                self.debug_closed = result.closed_set.into_iter().collect();
                self.debug_open   = result.open_set;
            }
        }

        if let Some(&next_pos) = self.path.first() {
            if let Some(dir) = self.direction_to(obs.pos, next_pos) {
                return Action::Move(dir);
            }
        }
        Action::Wait
    }

    fn debug_draw(&self) -> Option<Box<dyn DebugDraw>> {
        Some(Box::new(AStarDebugState {
            open:   self.debug_open.clone(),
            closed: self.debug_closed.clone(),
            path:   self.path.clone(),
        }))
    }

    fn reset(&mut self) {
        self.path.clear();
        self.debug_open.clear();
        self.debug_closed.clear();
    }
}

pub struct AStarDebugState {
    open:   Vec<GridPos>,
    closed: Vec<GridPos>,
    path:   Vec<GridPos>,
}

impl DebugDraw for AStarDebugState {
    fn draw_rects(&self) -> Vec<DebugRect> {
        let mut rects = Vec::new();
        for &p in &self.closed {
            rects.push(DebugRect { pos: p, color: Color::srgba(0.85, 0.20, 0.20, 0.18) });
        }
        for &p in &self.open {
            rects.push(DebugRect { pos: p, color: Color::srgba(0.20, 0.85, 0.20, 0.28) });
        }
        rects
    }

    fn draw_lines(&self, agent_pos: GridPos) -> Vec<DebugLine> {
        let mut lines = Vec::new();
        if !self.path.is_empty() {
            let mut current = agent_pos;
            for &next in &self.path {
                lines.push(DebugLine { start: current, end: next, color: Color::srgb(1.0, 0.90, 0.10) });
                current = next;
            }
        }
        lines
    }
}