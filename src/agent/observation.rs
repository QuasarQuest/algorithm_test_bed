// src/agent/observation.rs

use std::collections::HashSet;
use crate::world::Grid;
use crate::world::coords::GridPos;
use crate::world::tile::Tile;
use super::components::{GoldCarried, Health, Score};
use super::team::Team;

// ── A visible agent ───────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug)]
pub struct VisibleAgent {
    pub pos:          GridPos,
    pub team:         Team,
    pub health:       Health,
    pub gold_carried: GoldCarried,
}

impl VisibleAgent {
    pub fn is_enemy(&self, my_team: Team) -> bool {
        self.team != my_team
    }
    pub fn is_ally(&self, my_team: Team) -> bool {
        self.team == my_team
    }
}

// ── Observation ───────────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct Observation<'a> {
    pub pos:          GridPos,
    pub gold_carried: GoldCarried,
    pub health:       Health,
    pub score:        Score,
    pub team:         Team,
    pub tick:         u64,
    /// Scalar reward for this tick — populated by sim for RL agents, 0.0 otherwise.
    pub reward:       f32,

    grid:         &'a Grid,
    occupied:     &'a HashSet<GridPos>,
    pub other_agents: &'a [VisibleAgent],
}

impl<'a> Observation<'a> {
    pub fn new(
        pos:          GridPos,
        gold_carried: GoldCarried,
        health:       Health,
        score:        Score,
        team:         Team,
        grid:         &'a Grid,
        occupied:     &'a HashSet<GridPos>,
        other_agents: &'a [VisibleAgent],
        tick:         u64,
        reward:       f32,
    ) -> Self {
        Self { pos, gold_carried, health, score, team, grid, occupied, other_agents, tick, reward }
    }

    pub fn is_tile(&self, pos: GridPos, tile: Tile) -> bool {
        self.grid.get(pos.x, pos.y) == Some(tile)
    }

    pub fn nearest(&self, target_tile: Tile) -> Option<GridPos> {
        self.grid.iter()
            .filter(|(_, _, tile)| *tile == target_tile)
            .min_by_key(|(x, y, _)| GridPos::new(*x as i32, *y as i32).dist_sq(self.pos))
            .map(|(x, y, _)| GridPos::new(x as i32, y as i32))
    }

    pub fn is_walkable(&self, pos: GridPos) -> bool {
        self.grid.is_walkable(pos.x, pos.y)
            && (!self.occupied.contains(&pos) || pos == self.pos)
    }

    pub fn visible_agents(&self) -> &[VisibleAgent] { self.other_agents }

    pub fn nearest_enemy(&self) -> Option<&VisibleAgent> {
        self.other_agents.iter()
            .filter(|a| a.is_enemy(self.team))
            .min_by_key(|a| a.pos.dist_sq(self.pos))
    }

    pub fn nearest_ally(&self) -> Option<&VisibleAgent> {
        self.other_agents.iter()
            .filter(|a| a.is_ally(self.team) && a.pos != self.pos)
            .min_by_key(|a| a.pos.dist_sq(self.pos))
    }

    pub fn nearest_agent(&self) -> Option<&VisibleAgent> {
        self.other_agents.iter()
            .filter(|a| a.pos != self.pos)
            .min_by_key(|a| a.pos.dist_sq(self.pos))
    }
}