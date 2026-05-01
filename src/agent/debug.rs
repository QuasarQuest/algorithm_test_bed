// src/agent/debug.rs

use bevy::prelude::*;
use crate::viz::grid_offset::GridOffset;
use crate::agent::components::GridPos;

/// Agents that want to visualize their internal state implement this trait
/// on a separate struct, and return it via `Agent::debug_draw()`.
pub trait DebugDraw: Send + Sync {
    fn draw(&self, pos: GridPos, gizmos: &mut Gizmos, offset: &GridOffset);
}