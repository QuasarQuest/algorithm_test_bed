// src/viz/menu/components.rs

use bevy::prelude::*;

// ── Resources ─────────────────────────────────────────────────────────────────

#[derive(Resource)]
pub struct DebugVizConfig {
    pub show_global: bool,
}

impl Default for DebugVizConfig {
    fn default() -> Self {
        Self { show_global: true }
    }
}

// ── Markers ───────────────────────────────────────────────────────────────────

#[derive(Component)] pub struct PauseButtonMarker;
#[derive(Component)] pub struct PauseLabelMarker;

#[derive(Component)] pub struct SpeedDecreaseButton;
#[derive(Component)] pub struct SpeedIncreaseButton;
#[derive(Component)] pub struct SpeedResetButton;
#[derive(Component)] pub struct CurrentSpeedLabel;

#[derive(Component)] pub struct VizToggleButton;
#[derive(Component)] pub struct ThemeToggleButton;