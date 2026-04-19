// src/agent/spawn.rs
//
// Clean API for spawning agents into the Bevy world.
// Callers never touch raw ECS bundles — they call spawn_agent().

use bevy::prelude::*;
use super::brain::{Agent, Brain};
use super::components::{
    AgentLabel, GoldCarried, GridPos, Health, Score,
};
use super::systems::PendingAction;

// ── Agent bundle — everything a new agent entity needs ───────────────────────

#[derive(Bundle)]
pub struct AgentBundle {
    pub pos:     GridPos,
    pub health:  Health,
    pub gold:    GoldCarried,
    pub score:   Score,
    pub label:   AgentLabel,
    pub brain:   Brain,
    pub pending: PendingAction,
    pub sprite:  Sprite,
    pub transform: Transform,
}

impl AgentBundle {
    pub fn new(
        x:     i32,
        y:     i32,
        color: Color,
        agent: impl Agent + 'static,
    ) -> Self {
        let name = agent.name().to_string();
        Self {
            pos:       GridPos::new(x, y),
            health:    Health::default(),
            gold:      GoldCarried::default(),
            score:     Score::default(),
            label:     AgentLabel::new(name),
            brain:     Brain::new(agent),
            pending:   PendingAction::default(),
            sprite:    Sprite {
                color,
                custom_size: Some(Vec2::splat(crate::config::TILE_SIZE * 0.8)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 1.0), // z=1 renders above tiles
        }
    }
}