// src/agent/observation.rs
//
// Observation is the filtered world-view passed to Agent::act().
// The agent sees only what it is allowed to see — Fog of War lives here.
//
// Pure data. No Bevy, no Grid mutation.

use crate::world::tile::Tile;
use super::components::{GridPos, GoldCarried, Health, Score};

// ── A single visible cell ─────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug)]
pub struct VisibleCell {
    pub pos:  GridPos,
    pub tile: Tile,
}

// ── Observation ───────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct Observation {
    /// This agent's current grid position.
    pub pos: GridPos,

    /// How much gold this agent is currently carrying.
    pub gold_carried: GoldCarried,

    /// This agent's health.
    pub health: Health,

    /// This agent's cumulative score (gold delivered).
    pub score: Score,

    /// All cells this agent can currently see.
    /// With Fog of War off: the entire grid.
    /// With Fog of War on: cells within visibility radius.
    pub visible_cells: Vec<VisibleCell>,

    /// Current tick number — useful for time-based decisions.
    pub tick: u64,
}

impl Observation {
    /// Find the nearest visible cell matching a tile type.
    pub fn nearest(&self, tile: Tile) -> Option<GridPos> {
        self.visible_cells
            .iter()
            .filter(|c| c.tile == tile)
            .min_by_key(|c| {
                let dx = c.pos.x - self.pos.x;
                let dy = c.pos.y - self.pos.y;
                dx * dx + dy * dy // squared distance — no sqrt needed for comparison
            })
            .map(|c| c.pos)
    }

    /// Check if a specific position is visible and walkable.
    pub fn is_walkable(&self, pos: GridPos) -> bool {
        self.visible_cells
            .iter()
            .find(|c| c.pos == pos)
            .map(|c| c.tile.is_walkable())
            .unwrap_or(false)
    }
}