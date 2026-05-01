// src/viz/menu/components.rs

use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct DebugVizConfig {
    pub show_global: bool,
}

#[derive(Resource, Default)]
pub struct MenuState {
    pub is_open: bool,
}

// ── Markers ───────────────────────────────────────────────────────────────────

#[derive(Component)] pub struct HamburgerButton;
#[derive(Component)] pub struct PauseButtonMarker;
#[derive(Component)] pub struct SpeedDecreaseButton;
#[derive(Component)] pub struct SpeedIncreaseButton;
#[derive(Component)] pub struct SpeedResetButton;
#[derive(Component)] pub struct CurrentSpeedLabel;

#[derive(Component)] pub struct DrawerPanel;
#[derive(Component)] pub struct DrawerOverlay;
#[derive(Component)] pub struct ThemeToggleButton;

// Attached to individual agents to hide their debug rendering via Tooltip
#[derive(Component)] pub struct HideViz;