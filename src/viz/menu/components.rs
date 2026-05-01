// src/viz/menu/components.rs

use bevy::prelude::*;

// ── Resources ─────────────────────────────────────────────────────────────────

#[derive(Resource, Default)]
pub struct DebugVizConfig {
    pub show_global: bool,
}

#[derive(Resource, Default)]
pub struct MenuState {
    pub is_open: bool,
}

// ── Toolbar markers ───────────────────────────────────────────────────────────

#[derive(Component)] pub struct HamburgerButton;
#[derive(Component)] pub struct PauseButtonMarker;
#[derive(Component)] pub struct SpeedDecreaseButton;
#[derive(Component)] pub struct SpeedIncreaseButton;
#[derive(Component)] pub struct SpeedResetButton;
#[derive(Component)] pub struct CurrentSpeedLabel;

// ── Drawer markers ────────────────────────────────────────────────────────────

#[derive(Component)] pub struct DrawerPanel;
#[derive(Component)] pub struct DrawerOverlay;   // invisible click-catcher
#[derive(Component)] pub struct ThemeToggleButton;