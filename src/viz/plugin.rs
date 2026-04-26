// src/viz/plugin.rs

use bevy::prelude::*;

use super::camera::{spawn_camera, fit_camera_to_grid, init_pan_state, camera_controls};
use super::grid_offset::compute_grid_offset;
use super::tile_renderer::{spawn_tiles, sync_tile_colors};
use super::agent_renderer::{assign_agent_colours, sync_agent_transforms};
use super::algorithm::{draw_astar_debug, draw_dstar_debug};
use super::tooltip::{spawn_tooltip, update_tooltip};
use crate::viz::core_ui::theme::ThemeMode;
// Import Menu (Active)
use super::menu::components::DebugVizConfig;
use super::menu::{
    spawn_menu,
    handle_pause_button, handle_speed_buttons, handle_viz_toggle_button,
    update_button_styles, update_speed_label,
};

// Import HUD (Passive)
use super::hud::{
    spawn_hud, spawn_scoreboard, update_scoreboard, update_tick_label,
};

pub struct VizPlugin;

impl Plugin for VizPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<DebugVizConfig>()
            .init_resource::<ThemeMode>()
            .add_systems(PreStartup, init_pan_state)
            .add_systems(Startup, (
                spawn_camera,
                compute_grid_offset,
                spawn_tiles,
                fit_camera_to_grid,
            ).chain())
            // UI Generation Chain
            .add_systems(Startup, (
                spawn_menu,       // Top-Left and Center buttons
                spawn_hud,        // Top-Right tick counter
                spawn_scoreboard, // Bottom-Right scoreboard
                spawn_tooltip,
            ).chain())
            .add_systems(Update, (
                camera_controls,
                sync_tile_colors,
                sync_agent_transforms,
                assign_agent_colours,
                draw_astar_debug,
                draw_dstar_debug,

                // Menu Systems
                handle_viz_toggle_button,
                handle_pause_button,
                handle_speed_buttons,
                update_button_styles,
                update_speed_label,

                // HUD Systems
                update_tick_label,
                update_scoreboard,
                update_tooltip,
            ));
    }
}