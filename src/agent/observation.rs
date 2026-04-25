// src/agent/observation.rs

use std::collections::HashSet;
use crate::world::Grid;
use crate::world::tile::Tile;
use super::components::{GridPos, GoldCarried, Health, Score};

#[derive(Clone)]
pub struct Observation<'a> {
    pub pos: GridPos,
    pub gold_carried: GoldCarried,
    pub health: Health,
    pub score: Score,
    pub tick: u64,
    // Zero-copy references!
    grid: &'a Grid,
    occupied: &'a HashSet<GridPos>,
}

impl<'a> Observation<'a> {
    pub fn new(
        pos: GridPos,
        gold_carried: GoldCarried,
        health: Health,
        score: Score,
        grid: &'a Grid,
        occupied: &'a HashSet<GridPos>,
        tick: u64,
    ) -> Self {
        Self { pos, gold_carried, health, score, grid, occupied, tick }
    }

    /// Check if a specific position is a specific tile without allocating memory.
    pub fn is_tile(&self, pos: GridPos, tile: Tile) -> bool {
        self.grid.get(pos.x, pos.y) == Some(tile)
    }

    /// Find the nearest visible cell matching a tile type.
    pub fn nearest(&self, target_tile: Tile) -> Option<GridPos> {
        self.grid.iter()
            .filter(|(_, _, tile)| *tile == target_tile)
            .min_by_key(|(x, y, _)| {
                let dx = *x as i32 - self.pos.x;
                let dy = *y as i32 - self.pos.y;
                dx * dx + dy * dy
            })
            .map(|(x, y, _)| GridPos::new(x as i32, y as i32))
    }

    /// Walkable means it's not a wall AND no other agent is standing there.
    pub fn is_walkable(&self, pos: GridPos) -> bool {
        self.grid.is_walkable(pos.x, pos.y) && (!self.occupied.contains(&pos) || pos == self.pos)
    }
}