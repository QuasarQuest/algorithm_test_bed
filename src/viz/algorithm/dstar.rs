// src/viz/algorithm/dstar.rs
// D* Lite debug overlay — open set, discovered obstacles, planned path.

use bevy::prelude::*;
use crate::agent::brain::{Brain, DebugInfo};
use crate::world::coords::GridPos;
use crate::config;
use super::super::grid_offset::GridOffset;

pub fn draw_dstar_debug(
    mut gizmos: Gizmos,
    offset:     Res<GridOffset>,
    query:      Query<(&GridPos, &Brain)>,
) {
    let half = config::TILE_SIZE * 0.45;

    for (agent_pos, brain) in query.iter() {
        let Some(DebugInfo::DStarLite { open, obstacles, path }) = brain.debug_info() else {
            continue;
        };

        // 1. Draw dynamically discovered obstacles (Red)
        for (x, y) in obstacles.iter() {
            let c = offset.world_pos(*x, *y);
            gizmos.rect_2d(
                Isometry2d::from_translation(c),
                Vec2::splat(half),
                Color::srgba(1.0, 0.10, 0.10, 0.6), // Stronger red to highlight memory
            );
        }

        // 2. Draw the priority queue / open set (Faint Purple)
        for (x, y) in open.iter() {
            let c = offset.world_pos(*x, *y);
            gizmos.rect_2d(
                Isometry2d::from_translation(c),
                Vec2::splat(half),
                Color::srgba(0.70, 0.20, 0.95, 0.15),
            );
        }

        // 3. Draw the planned path (Bright Purple)
        if !path.is_empty() {
            let mut pts = Vec::with_capacity(path.len() + 1);
            pts.push(offset.world_pos(agent_pos.x, agent_pos.y));
            for (x, y) in path.iter() {
                pts.push(offset.world_pos(*x, *y));
            }
            gizmos.linestrip_2d(pts, Color::srgb(0.85, 0.30, 1.0));
        }
    }
}