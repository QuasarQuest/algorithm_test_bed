// src/agent/systems.rs

use bevy::prelude::*;
use std::collections::HashMap;
use crate::world::{Grid, tile::Tile};
use crate::config;
use super::action::Action;
use super::brain::{Agent, Brain};
use super::components::{AgentLabel, GridPos, GoldCarried, Health, Score};
use super::observation::Observation;
use crate::world::map_config::{AgentKind, MapConfig};
use crate::agent::r#impl::random_agent::RandomAgent;
use crate::agent::r#impl::astar_agent::AStarAgent;

// ── Constants ─────────────────────────────────────────────────────────────────

const COLOR_RANDOM_AGENT: Color = Color::srgb(0.9, 0.3, 0.2);
const COLOR_ASTAR_AGENT:  Color = Color::srgb(0.2, 0.8, 0.4);
const AGENT_Z_INDEX:      f32   = 1.0;
const AGENT_SPRITE_SCALE: f32   = 0.8;

// ── Resources & Components ────────────────────────────────────────────────────

#[derive(Resource, Default)]
pub struct TickCount(pub u64);

// PendingAction — written by tick_agents, consumed by apply_actions
#[derive(Component, Default)]
pub struct PendingAction(pub Option<Action>);

// ── Spawn ─────────────────────────────────────────────────────────────────────

pub fn spawn_agents(mut commands: Commands, map: Res<MapConfig>) {
    for (i, agent_config) in map.agents.iter().enumerate() {
        let id = i + 1;

        let (name, brain, color) = match agent_config.kind {
            AgentKind::Random => (
                format!("Random #{id}"),
                Brain::new(RandomAgent),
                COLOR_RANDOM_AGENT,
            ),
            AgentKind::AStar => (
                format!("A* #{id}"),
                Brain::new(AStarAgent::new()),
                COLOR_ASTAR_AGENT,
            ),
        };

        commands.spawn((
            GridPos::new(agent_config.x, agent_config.y),
            GoldCarried::default(),
            Score::default(),
            Health::default(),
            AgentLabel::new(name),
            brain,
            PendingAction::default(),
            Sprite {
                color,
                custom_size: Some(Vec2::splat(config::TILE_SIZE as f32 * AGENT_SPRITE_SCALE)),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, AGENT_Z_INDEX),
            Visibility::default(),
        ));
    }
}

// ── Tick: build observations and run brains ───────────────────────────────────

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

    // 1. Build the Spatial Hash
    let mut occupied = std::collections::HashSet::new();
    for (pos, _, _, _, _, _) in query.iter() {
        occupied.insert(*pos);
    }

    // 2. Run the brains with zero-copy observations
    for (pos, gold, health, score, mut brain, mut pending) in query.iter_mut() {
        let obs = Observation::new(*pos, *gold, *health, *score, &grid, &occupied, current_tick);
        let action = brain.act(&obs);
        pending.0 = Some(action);
    }
}

// ── Apply: execute pending actions ────────────────────────────────────────────

pub fn apply_actions(
    mut grid:  ResMut<Grid>,
    mut query: Query<(
        Entity, &mut GridPos, &mut GoldCarried,
        &mut Score, &mut PendingAction,
    )>,
) {
    // Phase 1 — Gather Intentions (Move AND Pickup)
    let mut move_requests: HashMap<GridPos, Vec<Entity>> = HashMap::new();
    let mut pickup_requests: HashMap<GridPos, Vec<Entity>> = HashMap::new();

    for (entity, pos, _, _, pending) in query.iter() {
        if let Some(action) = &pending.0 {
            match action {
                Action::Move(dir) => {
                    let (dx, dy) = dir.delta();
                    let next = GridPos::new(pos.x + dx, pos.y + dy);
                    move_requests.entry(next).or_default().push(entity);
                }
                Action::Pickup => {
                    pickup_requests.entry(*pos).or_default().push(entity);
                }
                _ => {} // Drop and Wait don't need conflict resolution
            }
        }
    }

    // Phase 1.5 — Resolve Ties Deterministically
    for contenders in pickup_requests.values_mut() {
        contenders.sort();
    }

    // Phase 2 — Execute
    for (entity, mut pos, mut gold, mut score, mut pending) in query.iter_mut() {
        let Some(action) = pending.0.take() else { continue };

        match action {
            Action::Move(dir) => {
                let (dx, dy) = dir.delta();
                let next = GridPos::new(pos.x + dx, pos.y + dy);

                if grid.is_walkable(next.x, next.y) {
                    if move_requests.get(&next).map(|v| v.len()) == Some(1) {
                        *pos = next;
                    }
                }
            }
            Action::Pickup => {
                if grid.get(pos.x, pos.y) == Some(Tile::Gold) && !gold.is_full() {
                    // Only pick up if this entity is the deterministic winner
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
                    info!("Dropped gold — total score: {}", score.0);
                }
            }
            Action::Wait => {}
        }
    }
}