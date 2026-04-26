// src/agent/observation.rs
//
// Observation is the filtered world-view passed to Agent::act().
// Zero-copy — holds references into the grid and the agent occupancy set.
// Fog of War would live here: filter visible_cells by radius around pos.

use std::collections::HashSet;
use crate::world::Grid;
use crate::world::coords::GridPos;
use crate::world::tile::Tile;
use super::components::{AgentLabel, GoldCarried, Health, Score};

// ── A visible agent ───────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug)]
pub struct VisibleAgent {
    pub pos:   GridPos,
    // Extend later: team, health, carrying, etc.
}

// ── Observation ───────────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct Observation<'a> {
    pub pos:          GridPos,
    pub gold_carried: GoldCarried,
    pub health:       Health,
    pub score:        Score,
    pub tick:         u64,

    // Zero-copy references — no Vec allocation per agent per tick
    grid:         &'a Grid,
    occupied:     &'a HashSet<GridPos>,

    // Other agents visible to this agent
    // Enables game theory, cooperation, avoidance strategies
    pub other_agents: &'a [VisibleAgent],
}

impl<'a> Observation<'a> {
    pub fn new(
        pos:          GridPos,
        gold_carried: GoldCarried,
        health:       Health,
        score:        Score,
        grid:         &'a Grid,
        occupied:     &'a HashSet<GridPos>,
        other_agents: &'a [VisibleAgent],
        tick:         u64,
    ) -> Self {
        Self { pos, gold_carried, health, score, grid, occupied, other_agents, tick }
    }

    /// Check if a specific position contains a specific tile.
    pub fn is_tile(&self, pos: GridPos, tile: Tile) -> bool {
        self.grid.get(pos.x, pos.y) == Some(tile)
    }

    /// Find the nearest tile of a given type.
    pub fn nearest(&self, target_tile: Tile) -> Option<GridPos> {
        self.grid.iter()
            .filter(|(_, _, tile)| *tile == target_tile)
            .min_by_key(|(x, y, _)| {
                let p = GridPos::new(*x as i32, *y as i32);
                p.dist_sq(self.pos)
            })
            .map(|(x, y, _)| GridPos::new(x as i32, y as i32))
    }

    /// Walkable = not an obstacle AND no other agent standing there.
    pub fn is_walkable(&self, pos: GridPos) -> bool {
        self.grid.is_walkable(pos.x, pos.y)
            && (!self.occupied.contains(&pos) || pos == self.pos)
    }

    /// All other agents this agent can see.
    /// With Fog of War: filter by radius. Currently: full visibility.
    pub fn visible_agents(&self) -> &[VisibleAgent] {
        self.other_agents
    }

    /// Nearest visible agent (excluding self). Useful for pursuit, avoidance.
    pub fn nearest_agent(&self) -> Option<&VisibleAgent> {
        self.other_agents.iter()
            .filter(|a| a.pos != self.pos)
            .min_by_key(|a| a.pos.dist_sq(self.pos))
    }
}
