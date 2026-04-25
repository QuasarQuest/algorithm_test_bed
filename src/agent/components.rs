// src/agent/components.rs

use bevy::prelude::*;
use crate::config;

// ── Position ──────────────────────────────────────────────────────────────────

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GridPos {
    pub x: i32,
    pub y: i32,
}

impl GridPos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    // THIS is the method the compiler is looking for!
    pub fn apply_delta(&self, dx: i32, dy: i32) -> Self {
        Self::new(self.x + dx, self.y + dy)
    }
}

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

// ── Label ─────────────────────────────────────────────────────────────────────

#[derive(Component, Clone, Debug)]
pub struct AgentLabel(pub String);

impl AgentLabel {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}