// src/agent/impl/random_agent.rs
// Self-contained random agent.
// Reactive behaviour: pickup gold when standing on it, drop at base, otherwise move randomly.

use bevy::prelude::*;
use crate::world::{Grid, Tile};
use crate::world::map_config::{AgentKind, MapConfig};
use crate::agent::action::{Action, Dir};
use crate::agent::components::{GridPos, GoldCarried, Score};
use crate::sim::OnSimTick;
use crate::config;

#[derive(Component, Default)]
pub struct PendingAction(pub Option<Action>);

fn spawn(mut commands: Commands, map: Res<MapConfig>) {
    for agent in &map.agents {
        if agent.kind != AgentKind::Random {
            continue;
        }
        commands.spawn((
            GridPos::new(agent.x, agent.y),
            PendingAction::default(),
            GoldCarried::default(),
            Score::default(),
            Sprite {
                color:       Color::srgb(0.9, 0.3, 0.2),
                custom_size: Some(Vec2::splat(config::TILE_SIZE * 0.8)),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 1.0),
            Visibility::default(),
        ));
    }
}

// Agent decides each tick:
// - On Gold  → Pickup
// - On Base  → Drop (if carrying)
// - Otherwise → random move
fn decide(
    grid:      Res<Grid>,
    mut query: Query<(&GridPos, &GoldCarried, &mut PendingAction)>,
) {
    for (pos, gold, mut pending) in query.iter_mut() {
        let action = match grid.get(pos.x, pos.y) {
            Some(Tile::Gold) if !gold.is_full() => Action::Pickup,
            Some(Tile::Base) if !gold.is_empty() => Action::Drop,
            _ => {
                let dirs = Dir::all();
                let idx  = rand::random_range(0..dirs.len());
                Action::Move(dirs[idx])
            }
        };
        pending.0 = Some(action);
    }
}

// Sim applies the decision to the world
fn apply(
    mut grid:  ResMut<Grid>,
    mut query: Query<(&mut GridPos, &mut PendingAction, &mut GoldCarried, &mut Score)>,
) {
    for (mut pos, mut pending, mut gold, mut score) in query.iter_mut() {
        let Some(action) = pending.0.take() else { continue };

        match action {
            Action::Move(dir) => {
                let (dx, dy) = dir.delta();
                if grid.is_walkable(pos.x + dx, pos.y + dy) {
                    pos.x += dx;
                    pos.y += dy;
                }
            }
            Action::Pickup => {
                if grid.get(pos.x, pos.y) == Some(Tile::Gold) && !gold.is_full() {
                    grid.set(pos.x, pos.y, Tile::Free);
                    gold.0 += 1;
                    info!("Picked up gold — carrying: {}", gold.0);
                }
            }
            Action::Drop => {
                if grid.get(pos.x, pos.y) == Some(Tile::Base) && !gold.is_empty() {
                    score.0 += gold.0;
                    gold.0 = 0;
                    info!("Dropped gold — total score: {}", score.0);
                }
            }
            Action::Wait => {}
        }
    }
}

pub struct RandomAgentPlugin;

impl Plugin for RandomAgentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn)
            .add_systems(OnSimTick, (decide, apply).chain());
    }
}