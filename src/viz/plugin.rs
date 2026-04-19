// src/viz/plugin.rs

use bevy::prelude::*;
use super::tile_renderer::{spawn_camera, spawn_tiles, sync_tile_colors};
use super::agent_renderer::sync_agent_transforms;

pub struct VizPlugin;

impl Plugin for VizPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_camera, spawn_tiles).chain())
            .add_systems(Update, (sync_tile_colors, sync_agent_transforms));
    }
}