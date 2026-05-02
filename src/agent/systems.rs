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
use super::team::{Team, TeamScore};

#[derive(Component, Default)]
pub struct PendingAction(pub Option<Action>);

pub fn tick_agents(
    grid:      Res<Grid>,
    sim_cfg:   Res<SimConfig>,
    mut query: Query<(
        &GridPos, &GoldCarried, &Health, &Score, &Team,
        &mut Brain, &mut PendingAction,
    )>,
) {
    let tick = sim_cfg.tick;

    let occupied: HashSet<GridPos> = query.iter().map(|(pos, ..)| *pos).collect();

    let all_agents: Vec<VisibleAgent> = query.iter()
        .map(|(pos, gold, health, _, team, _, _)| VisibleAgent {
            pos:          *pos,
            team:         *team,
            health:       *health,
            gold_carried: *gold,
        })
        .collect();

    for (pos, gold, health, score, team, mut brain, mut pending) in query.iter_mut() {
        let others: Vec<VisibleAgent> = all_agents.iter()
            .filter(|a| a.pos != *pos)
            .copied()
            .collect();

        let obs = Observation::new(
            *pos, *gold, *health, *score, *team,
            &grid, &occupied, &others, tick,
            0.0, // reward — 0.0 for non-RL agents
        );
        pending.0 = Some(brain.act(&obs));
    }
}

pub fn apply_actions(
    mut grid:       ResMut<Grid>,
    mut team_score: ResMut<TeamScore>,
    mut query: Query<(
        Entity, &mut GridPos, &mut GoldCarried,
        &mut Score, &Team, &mut PendingAction,
    )>,
) {
    let mut move_requests:   HashMap<GridPos, Vec<Entity>> = HashMap::new();
    let mut pickup_requests: HashMap<GridPos, Vec<Entity>> = HashMap::new();

    for (entity, pos, _, _, _, pending) in query.iter() {
        if let Some(action) = &pending.0 {
            match action {
                Action::Move(dir) => {
                    let (dx, dy) = dir.delta();
                    move_requests.entry(pos.apply_delta(dx, dy)).or_default().push(entity);
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

    for (entity, mut pos, mut gold, mut score, team, mut pending) in query.iter_mut() {
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
                    }
                }
            }
            Action::Drop => {
                if grid.get(pos.x, pos.y) == Some(Tile::Base) && !gold.is_empty() {
                    let delivered = gold.0;
                    score.0 += delivered;
                    team_score.add(*team, delivered);
                    gold.0 = 0;
                    info!("Team {} delivered {} gold — team total: {}",
                        team.name(), delivered, team_score.get(*team));
                }
            }
            Action::Attack(_) => {
                // Handled by combat::resolve_combat — skip here
            }
            Action::Wait => {}
        }
    }
}