// src/viz/hud/systems.rs

use bevy::prelude::*;
use crate::sim::config::SimConfig;
use super::components::TickLabelMarker;

pub fn update_tick_label(
    cfg: Res<SimConfig>,
    mut query: Query<&mut Text, With<TickLabelMarker>>,
) {
    if !cfg.is_changed() { return; }
    for mut text in query.iter_mut() {
        *text = Text::new(format!("{}", cfg.tick));
    }
}