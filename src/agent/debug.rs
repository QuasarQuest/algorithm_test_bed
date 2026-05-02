// src/agent/debug.rs

use bevy::prelude::Color;
use crate::agent::components::GridPos;

#[derive(Clone, Debug)]
pub struct DebugRect {
    pub pos: GridPos,
    pub color: Color,
}

#[derive(Clone, Debug)]
pub struct DebugLine {
    pub start: GridPos,
    pub end: GridPos,
    pub color: Color,
}

/// Agents that want to visualize their internal state implement this trait
/// to return geometric primitives in raw grid coordinates.
pub trait DebugDraw: Send + Sync {
    fn draw_rects(&self) -> Vec<DebugRect> { Vec::new() }
    fn draw_lines(&self, _agent_pos: GridPos) -> Vec<DebugLine> { Vec::new() }
}