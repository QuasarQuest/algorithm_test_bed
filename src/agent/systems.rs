// src/agent/systems.rs
//
// Bevy systems — the only place where Agent logic touches the ECS world.
// Order per tick:
//   1. build_observations  — read Grid + Components → Observation per agent
//   2. run_brains          — call Brain::act(obs) → Action per agent
//   3. apply_actions       — write Actions back into Grid + Components

use bevy::prelude::*;
use crate::world::{Grid, tile::Tile};
use crate::sim::{SimTimer, SimConfig};
use super::action::Action;
use super::brain::Brain;
use super::components::{GridPos, GoldCarried, Health, Score};
use super::observation::{Observation, VisibleCell};

// ── Tick counter resource ─────────────────────────────────────────────────────

#[derive(Resource, Default)]
pub struct TickCount(pub u64);

// ── Pending action — written by run_brains, read by apply_actions ─────────────
// Stored as a Component so we avoid borrow conflicts between systems.

#[derive(Component, Default)]
pub struct PendingAction(pub Option<Action>);

// ── System 1: build observations and run brains ───────────────────────────────
//
// Combined into one system to avoid storing Observation as a Component
// (it contains a Vec which would allocate per-entity per-frame otherwise).

pub fn tick_agents(
    time:       Res<Time>,
    mut timer:  ResMut<SimTimer>,
    cfg:        Res<SimConfig>,
    grid:       Res<Grid>,
    mut tick:   ResMut<TickCount>,
    mut query:  Query<(
        &GridPos,
        &GoldCarried,
        &Health,
        &Score,
        &mut Brain,
        &mut PendingAction,
    )>,
) {
    if cfg.paused {
        return;
    }
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    tick.0 += 1;
    let current_tick = tick.0;

    for (pos, gold, health, score, mut brain, mut pending) in query.iter_mut() {
        let obs = build_observation(*pos, *gold, *health, *score, &grid, current_tick);
        let action = brain.act(&obs);
        pending.0 = Some(action);
    }
}

// ── System 2: apply pending actions to the world ──────────────────────────────

pub fn apply_actions(
    cfg:      Res<SimConfig>,
    mut grid: ResMut<Grid>,
    mut query: Query<(
        &mut GridPos,
        &mut GoldCarried,
        &mut Score,
        &mut PendingAction,
    )>,
) {
    if cfg.paused {
        return;
    }

    for (mut pos, mut gold, mut score, mut pending) in query.iter_mut() {
        let Some(action) = pending.0.take() else { continue };

        match action {
            Action::Move(dir) => {
                let (dx, dy) = dir.delta();
                let next = pos.apply_delta(dx, dy);

                if grid.is_walkable(next.x, next.y) {
                    *pos = next;
                }
                // Invalid moves are silently ignored — agent stays put.
            }

            Action::Pickup => {
                let tile = grid.get(pos.x, pos.y);
                if tile == Tile::Gold && !gold.is_full() {
                    grid.set(pos.x, pos.y, Tile::Free);
                    gold.0 += 1;
                }
            }

            Action::Drop => {
                let tile = grid.get(pos.x, pos.y);
                if tile == Tile::Base && !gold.is_empty() {
                    score.0 += gold.0;
                    gold.0 = 0;
                }
            }

            Action::Wait => {}
        }
    }
}

// ── Helper: build an Observation for one agent ────────────────────────────────
//
// Fog of War: currently disabled — agent sees full grid.
// To enable FoW later: filter visible_cells by radius around pos.

fn build_observation(
    pos:   GridPos,
    gold:  GoldCarried,
    health: Health,
    score:  Score,
    grid:  &Grid,
    tick:  u64,
) -> Observation {
    // Full visibility — replace this with radius filter for Fog of War
    let visible_cells = grid.all_cells()
        .map(|(x, y, tile)| VisibleCell {
            pos: GridPos::new(x, y),
            tile,
        })
        .collect();

    Observation {
        pos,
        gold_carried: gold,
        health,
        score,
        visible_cells,
        tick,
    }
}