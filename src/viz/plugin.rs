use bevy::prelude::*;
use super::tile_renderer::{spawn_camera, spawn_tiles, sync_tile_colors};
use super::agent_renderer::sync_agent_transforms;
use super::hud::{
    spawn_hud, handle_pause_button, handle_speed_buttons,
    update_button_styles, update_tick_label, update_speed_label, count_ticks, TickCount,
};
use crate::sim::OnSimTick;

pub struct VizPlugin;

impl Plugin for VizPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TickCount>()
            .add_systems(Startup, (spawn_camera, spawn_tiles, spawn_hud))
            .add_systems(Update, (
                sync_tile_colors,
                sync_agent_transforms,
                handle_pause_button,
                handle_speed_buttons,
                update_button_styles,
                update_tick_label,
                update_speed_label, // Register the new system here
            ))
            .add_systems(OnSimTick, count_ticks);
    }
}