// src/agent/systems.rs

use bevy::prelude::*;
use crate::world::{Grid, tile::Tile};
// Notice we no longer need Time, TickTimer, or SimConfig here!
use super::action::Action;
use super::brain::Brain;
use super::components::{GridPos, GoldCarried, Health, Score};
use super::observation::{Observation, VisibleCell};

use crate::world::map_config::{AgentKind, MapConfig};
use crate::agent::r#impl::random_agent::RandomAgent;
use crate::agent::r#impl::astar_agent::AStarAgent;
use crate::agent::spawn::AgentBundle;

#[derive(Resource, Default)]
pub struct TickCount(pub u64);

#[derive(Component, Default)]
pub struct PendingAction(pub Option<Action>);

pub fn spawn_agents(mut commands: Commands, map: Res<MapConfig>) {
    for agent_config in map.agents.iter() {
        match agent_config.kind {
            AgentKind::Random => {
                commands.spawn(AgentBundle::new(
                    agent_config.x,
                    agent_config.y,
                    Color::srgb(0.9, 0.3, 0.2),
                    RandomAgent
                ));
            }
            AgentKind::AStar => {
                commands.spawn(AgentBundle::new(
                    agent_config.x,
                    agent_config.y,
                    Color::srgb(0.2, 0.8, 0.4),
                    AStarAgent::new()
                ));
            }
        }
    }
}

pub fn tick_agents(
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
    tick.0 += 1;
    let current_tick = tick.0;

    for (pos, gold, health, score, mut brain, mut pending) in query.iter_mut() {
        let obs = build_observation(*pos, *gold, *health, *score, &grid, current_tick);
        let action = brain.act(&obs);
        pending.0 = Some(action);
    }
}

pub fn apply_actions(
    mut grid: ResMut<Grid>,
    mut query: Query<(
        &mut GridPos,
        &mut GoldCarried,
        &mut Score,
        &mut PendingAction,
    )>,
) {
    for (mut pos, mut gold, mut score, mut pending) in query.iter_mut() {
        let Some(action) = pending.0.take() else { continue };

        match action {
            Action::Move(dir) => {
                let (dx, dy) = dir.delta();
                let next = pos.apply_delta(dx, dy);

                if grid.is_walkable(next.x, next.y) {
                    *pos = next;
                }
            }
            Action::Pickup => {
                let tile = grid.get(pos.x, pos.y);
                if tile == Some(Tile::Gold) && !gold.is_full() {
                    grid.set(pos.x, pos.y, Tile::Free);
                    gold.0 += 1;
                }
            }
            Action::Drop => {
                let tile = grid.get(pos.x, pos.y);
                if tile == Some(Tile::Base) && !gold.is_empty() {
                    score.0 += gold.0;
                    gold.0 = 0;
                }
            }
            Action::Wait => {}
        }
    }
}

fn build_observation(
    pos:    GridPos,
    gold:   GoldCarried,
    health: Health,
    score:  Score,
    grid:   &Grid,
    tick:   u64,
) -> Observation {
    let visible_cells = grid.iter()
        .map(|(x, y, tile)| VisibleCell {
            pos: GridPos::new(x as i32, y as i32),
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