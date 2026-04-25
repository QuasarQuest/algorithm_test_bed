// src/viz/algorithm.rs
// Debug overlay for algorithm internals (A* open/closed/path).
// Uses GridOffset so gizmos land on the correct world positions.

use bevy::prelude::*;
use crate::agent::brain::{Brain, DebugInfo};
use crate::agent::components::GridPos;
use super::grid_offset::GridOffset;
use crate::config;

pub fn draw_algorithm_debug(
    mut gizmos: Gizmos,
    offset:     Res<GridOffset>,
    query:      Query<(&GridPos, &Brain)>,
) {
    let half = config::TILE_SIZE * 0.45;

    for (agent_pos, brain) in query.iter() {
        let Some(DebugInfo::AStar { open, closed, path }) = brain.debug_info() else {
            continue;
        };

        // Closed set — faint red squares
        for (x, y) in closed.iter() {
            let c = offset.world_pos(*x, *y);
            gizmos.rect_2d(
                Isometry2d::from_translation(c),
                Vec2::splat(half),
                Color::srgba(0.85, 0.20, 0.20, 0.18),
            );
        }

        // Open set — faint green squares
        for (x, y) in open.iter() {
            let c = offset.world_pos(*x, *y);
            gizmos.rect_2d(
                Isometry2d::from_translation(c),
                Vec2::splat(half),
                Color::srgba(0.20, 0.85, 0.20, 0.28),
            );
        }

        // Planned path — yellow line from agent through waypoints
        if !path.is_empty() {
            let mut pts = Vec::with_capacity(path.len() + 1);
            pts.push(offset.world_pos(agent_pos.x, agent_pos.y));
            for (x, y) in path.iter() {
                pts.push(offset.world_pos(*x, *y));
            }
            gizmos.linestrip_2d(pts, Color::srgb(1.0, 0.90, 0.10));
        }
    }
}