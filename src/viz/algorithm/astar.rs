// src/viz/algorithm/astar.rs
// A* debug overlay — open set, closed set, planned path.

use bevy::prelude::*;
use crate::agent::brain::{Brain, DebugInfo};
use crate::world::coords::GridPos;
use crate::config;
use super::super::grid_offset::GridOffset;

pub fn draw_astar_debug(
    mut gizmos: Gizmos,
    offset:     Res<GridOffset>,
    query:      Query<(&GridPos, &Brain)>,
) {
    let half = config::TILE_SIZE * 0.45;

    for (agent_pos, brain) in query.iter() {
        let Some(DebugInfo::AStar { open, closed, path }) = brain.debug_info() else {
            continue;
        };

        for (x, y) in closed.iter() {
            let c = offset.world_pos(*x, *y);
            gizmos.rect_2d(
                Isometry2d::from_translation(c),
                Vec2::splat(half),
                Color::srgba(0.85, 0.20, 0.20, 0.18),
            );
        }

        for (x, y) in open.iter() {
            let c = offset.world_pos(*x, *y);
            gizmos.rect_2d(
                Isometry2d::from_translation(c),
                Vec2::splat(half),
                Color::srgba(0.20, 0.85, 0.20, 0.28),
            );
        }

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
