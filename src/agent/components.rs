// src/agent/components.rs
// Pure ECS data components — no logic, no Bevy systems.
// GridPos lives in world/coords.rs — re-exported here for convenience.

use bevy::prelude::*;
use crate::config;

pub use crate::world::coords::GridPos;

// ── Gold ──────────────────────────────────────────────────────────────────────

#[derive(Component, Clone, Copy, Debug, Default)]
pub struct GoldCarried(pub u32);

impl GoldCarried {
    pub fn is_full(self)  -> bool { self.0 >= config::AGENT_MAX_GOLD }
    pub fn is_empty(self) -> bool { self.0 == 0 }
}

// ── Score ─────────────────────────────────────────────────────────────────────

#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Score(pub u32);

// ── Health ────────────────────────────────────────────────────────────────────

#[derive(Component, Clone, Copy, Debug)]
pub struct Health(pub u32);

impl Default for Health {
    fn default() -> Self {
        Self(config::AGENT_START_HEALTH)
    }
}

// ── Label — human-readable agent name ────────────────────────────────────────

#[derive(Component, Clone, Debug)]
pub struct AgentLabel(pub String);

impl AgentLabel {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}
