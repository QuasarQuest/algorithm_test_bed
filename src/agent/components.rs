// src/agent/components.rs
//
// ECS Components — pure data, no logic.
// These are the facts Bevy stores per agent entity.
// Systems in systems.rs read and write these.

use bevy::prelude::*;
use crate::config;

// ── Position on the grid ──────────────────────────────────────────────────────

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GridPos {
    pub x: i32,
    pub y: i32,
}

impl GridPos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn apply_delta(self, dx: i32, dy: i32) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

// ── Health ────────────────────────────────────────────────────────────────────

#[derive(Component, Clone, Copy, Debug)]
pub struct Health(pub u32);

impl Default for Health {
    fn default() -> Self {
        Self(config::AGENT_START_HEALTH)
    }
}

// ── Gold carried ──────────────────────────────────────────────────────────────

#[derive(Component, Clone, Copy, Debug, Default)]
pub struct GoldCarried(pub u32);

impl GoldCarried {
    pub fn is_full(self) -> bool {
        self.0 >= config::AGENT_MAX_GOLD
    }

    pub fn is_empty(self) -> bool {
        self.0 == 0
    }
}

// ── Score — gold successfully delivered to Base ───────────────────────────────

#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Score(pub u32);

// ── Agent label — human-readable name for UI and debug ───────────────────────

#[derive(Component, Clone, Debug)]
pub struct AgentLabel(pub String);

impl AgentLabel {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}

// ── Marker components — tag which kind of agent this entity is ───────────────
// These allow targeted queries: Query<&GridPos, With<RandomAgentMarker>>

#[derive(Component, Default)]
pub struct RandomAgentMarker;

#[derive(Component, Default)]
pub struct AStarAgentMarker;