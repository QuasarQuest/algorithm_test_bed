// src/viz/grid_offset.rs
// Single source of truth for grid → world coordinate conversion.
// Computed once at Startup from Grid. Used by tile_renderer, agent_renderer,
// algorithm debug overlay, and tooltip.

use bevy::prelude::*;
use crate::world::Grid;
use crate::config;

#[derive(Resource, Clone, Copy)]
pub struct GridOffset {
    pub x:    f32,
    pub y:    f32,
    pub step: f32,
}

impl GridOffset {
    pub fn world_pos(&self, gx: i32, gy: i32) -> Vec2 {
        Vec2::new(
            self.x + gx as f32 * self.step,
            self.y + gy as f32 * self.step,
        )
    }
}

pub fn compute_grid_offset(mut commands: Commands, grid: Res<Grid>) {
    let step = config::TILE_SIZE + config::TILE_GAP;
    commands.insert_resource(GridOffset {
        x:    -(grid.width  as f32 * step) / 2.0 + step / 2.0,
        y:    -(grid.height as f32 * step) / 2.0 + step / 2.0,
        step,
    });
}