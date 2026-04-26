// src/agent/systems.rs

use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
use crate::world::{Grid, tile::Tile};
use crate::world::coords::GridPos;
use crate::sim::config::SimConfig;
use super::action::Action;
use super::brain::Brain;
use super::components::{GoldCarried, Health, Score};
use super::observation::{Observation, VisibleAgent};

#[derive(Component, Default)]
pub struct PendingAction(pub Option<Action>);

pub fn tick_agents(
    grid:      Res<Grid>,
    sim_cfg:   Res<SimConfig>,
    mut query: Query<(
        &GridPos, &GoldCarried, &Health, &Score,
        &mut Brain, &mut PendingAction,
    )>,
) {
    let tick = sim_cfg.tick;

    let occupied: HashSet<GridPos> = query.iter()
        .map(|(pos, ..)| *pos)
        .collect();

    let all_agents: Vec<VisibleAgent> = query.iter()
        .map(|(pos, ..)| VisibleAgent { pos: *pos })
        .collect();

    for (pos, gold, health, score, mut brain, mut pending) in query.iter_mut() {
        let others: Vec<VisibleAgent> = all_agents.iter()
            .filter(|a| a.pos != *pos)
            .copied()
            .collect();

        let obs = Observation::new(
            *pos, *gold, *health, *score,
            &grid, &occupied, &others, tick,
        );
        pending.0 = Some(brain.act(&obs));
    }
}

pub fn apply_actions(
    mut grid:  ResMut<Grid>,
    mut query: Query<(
        Entity, &mut GridPos, &mut GoldCarried,
        &mut Score, &mut PendingAction,
    )>,
) {
    let mut move_requests:   HashMap<GridPos, Vec<Entity>> = HashMap::new();
    let mut pickup_requests: HashMap<GridPos, Vec<Entity>> = HashMap::new();

    for (entity, pos, _, _, pending) in query.iter() {
        if let Some(action) = &pending.0 {
            match action {
                Action::Move(dir) => {
                    let (dx, dy) = dir.delta();
                    move_requests
                        .entry(pos.apply_delta(dx, dy))
                        .or_default()
                        .push(entity);
                }
                Action::Pickup => {
                    pickup_requests.entry(*pos).or_default().push(entity);
                }
                _ => {}
            }
        }
    }

    for v in pickup_requests.values_mut() { v.sort(); }
    for v in move_requests.values_mut()   { v.sort(); }

    for (entity, mut pos, mut gold, mut score, mut pending) in query.iter_mut() {
        let Some(action) = pending.0.take() else { continue };

        match action {
            Action::Move(dir) => {
                let (dx, dy) = dir.delta();
                let next = pos.apply_delta(dx, dy);
                if grid.is_walkable(next.x, next.y) {
                    if move_requests.get(&next).map(|v| v.len()) == Some(1) {
                        *pos = next;
                    }
                }
            }
            Action::Pickup => {
                if grid.get(pos.x, pos.y) == Some(Tile::Gold) && !gold.is_full() {
                    if pickup_requests.get(&pos).and_then(|v| v.first()) == Some(&entity) {
                        grid.set(pos.x, pos.y, Tile::Free);
                        gold.0 += 1;
                        info!("Picked up gold — carrying: {}", gold.0);
                    }
                }
            }
            Action::Drop => {
                if grid.get(pos.x, pos.y) == Some(Tile::Base) && !gold.is_empty() {
                    score.0 += gold.0;
                    gold.0   = 0;
                    info!("Delivered gold — total score: {}", score.0);
                }
            }
            Action::Wait => {}
        }
    }
}