// src/viz/plugin.rs

use bevy::prelude::*;
use super::tile_renderer::{spawn_camera, spawn_tiles, sync_tile_colors};
use super::agent_renderer::sync_agent_transforms;
use super::hud::{
    spawn_hud, count_ticks, TickCount,
    spawn_scoreboard, update_scoreboard,
    handle_pause_button, handle_speed_buttons,
    update_button_styles, update_tick_label, update_speed_label,
};
use super::tooltip::{spawn_tooltip, update_tooltip};
use crate::sim::OnSimTick;

pub struct VizPlugin;

impl Plugin for VizPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TickCount>()
           .add_systems(Startup, (
               spawn_camera,
               spawn_tiles,
               spawn_hud,
               spawn_scoreboard,
               spawn_tooltip,
           ))
           .add_systems(Update, (
               sync_tile_colors,
               sync_agent_transforms,
               handle_pause_button,
               handle_speed_buttons,
               update_button_styles,
               update_tick_label,
               update_speed_label,
               update_scoreboard,
               update_tooltip,
           ))
           .add_systems(OnSimTick, count_ticks);
    }
}
