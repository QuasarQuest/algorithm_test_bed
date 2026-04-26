// src/agent/spawn.rs
//
// Single authoritative spawn site for all agent entities.
// AgentBundle is the canonical "everything a new agent needs" definition.
// systems.rs calls spawn_agents() which uses AgentBundle — nowhere else
// should raw agent component tuples be assembled.

use bevy::prelude::*;
use crate::config;
use crate::world::map_config::MapConfig;
use super::brain::Brain;
use super::components::{AgentLabel, GoldCarried, GridPos, Health, Score};
use super::systems::PendingAction;
use super::registry::{agent_color, agent_name_prefix, make_brain};

// ── Agent bundle ──────────────────────────────────────────────────────────────

#[derive(Bundle)]
pub struct AgentBundle {
    pub pos:       GridPos,
    pub health:    Health,
    pub gold:      GoldCarried,
    pub score:     Score,
    pub label:     AgentLabel,
    pub brain:     Brain,
    pub pending:   PendingAction,
    pub sprite:    Sprite,
    pub transform: Transform,
    pub visibility: Visibility,
}

impl AgentBundle {
    pub fn new(x: i32, y: i32, label: AgentLabel, brain: Brain, color: Color) -> Self {
        Self {
            pos:        GridPos::new(x, y),
            health:     Health::default(),
            gold:       GoldCarried::default(),
            score:      Score::default(),
            label,
            brain,
            pending:    PendingAction::default(),
            sprite:     Sprite {
                color,
                custom_size: Some(Vec2::splat(config::TILE_SIZE * 0.8)),
                ..default()
            },
            transform:  Transform::from_xyz(0.0, 0.0, 1.0),
            visibility: Visibility::default(),
        }
    }
}

// ── Spawn system ──────────────────────────────────────────────────────────────

pub fn spawn_agents(mut commands: Commands, map: Res<MapConfig>) {
    for (i, cfg) in map.agents.iter().enumerate() {
        let id     = i + 1;
        let prefix = agent_name_prefix(cfg.kind);
        let label  = AgentLabel::new(format!("{prefix} #{id}"));
        let brain  = Brain::new_boxed(make_brain(cfg.kind));
        let color  = agent_color(cfg.kind);

        commands.spawn(AgentBundle::new(cfg.x, cfg.y, label, brain, color));
    }
}
