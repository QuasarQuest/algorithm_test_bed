// src/viz/agent_renderer.rs

use bevy::prelude::*;
use crate::world::coords::GridPos;
use crate::agent::components::AgentLabel;
use super::grid_offset::GridOffset;

const AGENT_COLOURS: &[Color] = &[
    Color::srgb(0.95, 0.35, 0.25), // red-orange  (Random)
    Color::srgb(0.25, 0.65, 0.95), // sky blue     (A*)
    Color::srgb(0.95, 0.75, 0.20), // gold
    Color::srgb(0.55, 0.90, 0.40), // lime green
    Color::srgb(0.85, 0.35, 0.90), // violet
];

/// Assigns a stable colour on spawn — runs once per agent via Added<AgentLabel>.
pub fn assign_agent_colours(
    mut query: Query<(&AgentLabel, &mut Sprite), Added<AgentLabel>>,
) {
    for (label, mut sprite) in query.iter_mut() {
        let idx = label.0.bytes()
            .fold(0usize, |a, b| a.wrapping_add(b as usize))
            % AGENT_COLOURS.len();
        sprite.color = AGENT_COLOURS[idx];
    }
}

/// Syncs GridPos → world Transform every frame.
pub fn sync_agent_transforms(
    offset:    Res<GridOffset>,
    mut query: Query<(&GridPos, &mut Transform)>,
) {
    for (pos, mut transform) in query.iter_mut() {
        let world = offset.world_pos(pos.x, pos.y);
        transform.translation.x = world.x;
        transform.translation.y = world.y;
        transform.translation.z = 1.0;
    }
}
