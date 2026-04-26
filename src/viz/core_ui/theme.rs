// src/viz/core_ui/theme.rs

use bevy::prelude::*;
use super::color::*;

// ── Theme State Resource ──────────────────────────────────────────────────────

#[derive(Resource, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThemeMode {
    #[default]
    Dark,
    Light,
}

// ── Semantic Color Enum ───────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug)]
pub enum ThemeColor {
    // Structural
    Background,
    Border,

    // Text
    TextPrimary,
    TextDim,

    // UI Elements
    ButtonIdle,
    ButtonHover,

    // Feedback / States
    Success,
    SuccessText,
    Warning,
    WarningText,
    Error,
    ErrorText,
}

impl ThemeColor {
    /// Resolves the semantic enum into an actual Bevy Color based on the current mode.
    pub fn resolve(&self, mode: ThemeMode) -> Color {
        match (self, mode) {
            // Dark Mode Resolutions
            (Self::Background,  ThemeMode::Dark) => GRAY_900,
            (Self::TextPrimary, ThemeMode::Dark) => GRAY_100,
            (Self::TextDim,     ThemeMode::Dark) => GRAY_400,
            (Self::ButtonIdle,  ThemeMode::Dark) => GRAY_800,
            (Self::ButtonHover, ThemeMode::Dark) => GRAY_700,

            // Light Mode Resolutions
            (Self::Background,  ThemeMode::Light) => WHITE_900,
            (Self::TextPrimary, ThemeMode::Light) => DARK_TEXT,
            (Self::TextDim,     ThemeMode::Light) => GRAY_400, // Same for both
            (Self::ButtonIdle,  ThemeMode::Light) => WHITE_800,
            (Self::ButtonHover, ThemeMode::Light) => WHITE_700,

            // Universal States (Often the same across light/dark, but can be tweaked)
            (Self::Success,      _) => GREEN_500,
            (Self::SuccessText,  _) => GREEN_400,
            (Self::Warning,      _) => GOLD_500,
            (Self::WarningText,  _) => GRAY_900, // Dark text on gold looks better
            (Self::Error,        _) => RED_500,
            (Self::ErrorText,    _) => RED_400,

            // Fallback for missing matches
            (Self::Border, _) => Color::srgba(0.5, 0.5, 0.5, 0.2),
        }
    }
}
#[derive(Component)]
pub struct UiRoot;
// Standard text sizes remain here
pub const SIZE_SM: f32 = 11.0;
pub const SIZE_MD: f32 = 13.0;
pub const SIZE_LG: f32 = 15.0;
pub const SIZE_XL: f32 = 16.0;