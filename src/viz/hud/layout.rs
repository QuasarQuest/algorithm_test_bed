// src/viz/hud/layout.rs

use bevy::prelude::*;
use crate::viz::core_ui::*;
use super::components::*;

// 1. The Bevy System (Runs at Startup)
pub fn spawn_hud(mut commands: Commands, theme: Res<ThemeMode>) {
    build_hud(&mut commands, *theme);
}

// 2. The Logic (Can be called anytime!)
pub fn build_hud(commands: &mut Commands, mode: ThemeMode) {
    spawn_floating_panel(commands, mode, Val::Px(12.0), Val::Px(12.0), Val::Auto, Val::Auto, |p| {
        // Resolve text colors dynamically based on the mode
        spawn_label(p, "TICK", ThemeColor::TextDim.resolve(mode), SIZE_SM);
        spawn_marked_label(p, "0", ThemeColor::TextPrimary.resolve(mode), SIZE_LG, TickLabelMarker);
    });
}