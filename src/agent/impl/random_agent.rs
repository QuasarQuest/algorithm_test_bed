// src/agent/impl/random_agent.rs

use bevy::prelude::*;
use crate::world::Grid;
use crate::agent::action::{Action, Dir};
use crate::agent::components::GridPos;
use crate::sim::OnSimTick;
use crate::config;

// Component that holds the agent's decision for this tick
#[derive(Component, Default)]
pub struct PendingAction(pub Option<Action>);

fn spawn(mut commands: Commands) {
    commands.spawn((
        GridPos::new(
            (config::GRID_W / 2) as i32,
            (config::GRID_H / 2) as i32,
        ),
        PendingAction::default(),
        Sprite {
            color:       Color::srgb(0.9, 0.3, 0.2),
            custom_size: Some(Vec2::splat(config::TILE_SIZE * 0.8)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
        Visibility::default(),
    ));
}

// Step 1 — agent decides, writes Action into PendingAction
fn decide(mut query: Query<&mut PendingAction, With<GridPos>>) {
    for mut pending in query.iter_mut() {
        let walkable: Vec<Dir> = Dir::all()
            .iter()
            .copied()
            .collect();
        let idx = rand::random_range(0..walkable.len());
        pending.0 = Some(Action::Move(walkable[idx]));
    }
}

// Step 2 — sim applies PendingAction to the world
fn apply(
    grid:      Res<Grid>,
    mut query: Query<(&mut GridPos, &mut PendingAction)>,
) {
    for (mut pos, mut pending) in query.iter_mut() {
        let Some(action) = pending.0.take() else { continue };

        match action {
            Action::Move(dir) => {
                let (dx, dy) = dir.delta();
                if grid.is_walkable(pos.x + dx, pos.y + dy) {
                    pos.x += dx;
                    pos.y += dy;
                }
                // invalid move — agent stays put, no panic
            }
            Action::Pickup | Action::Drop | Action::Wait => {
                // not yet implemented
            }
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