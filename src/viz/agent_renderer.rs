// src/viz/agent_renderer.rs

use bevy::prelude::*;
use crate::agent::components::GridPos;
use crate::config;

pub fn sync_agent_transforms(
    mut query: Query<(&GridPos, &mut Transform)>,
) {
    let step     = config::TILE_SIZE + config::TILE_GAP;
    let offset_x = -(config::GRID_W as f32 * step) / 2.0 + step / 2.0;
    let offset_y = -(config::GRID_H as f32 * step) / 2.0 + step / 2.0;

    for (pos, mut transform) in query.iter_mut() {
        transform.translation.x = offset_x + pos.x as f32 * step;
        transform.translation.y = offset_y + pos.y as f32 * step;
    }
}