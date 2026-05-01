// src/viz/hud/layout.rs
// Tick label and controls live in the toolbar (menu/layout.rs).
// This is kept as a shell so react_to_ui_changes can call build_hud().

use bevy::prelude::*;
use crate::viz::core_ui::ThemeMode;

pub fn spawn_hud(mut commands: Commands, theme: Res<ThemeMode>) {
    build_hud(&mut commands, *theme);
}

pub fn build_hud(_commands: &mut Commands, _mode: ThemeMode) {
    // Extend here for additional HUD panels in the future.
}