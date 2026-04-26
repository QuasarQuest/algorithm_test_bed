// src/viz/menu/systems.rs

use bevy::prelude::*;
use crate::sim::config::{SimConfig, AVAILABLE_SPEEDS};
use crate::sim::timer::TickTimer;
use crate::viz::core_ui::theme::{ThemeMode, ThemeColor, UiRoot};
use super::components::*;

// ── Hamburger & Menu State ────────────────────────────────────────────────────

pub fn handle_hamburger_button(
    query: Query<&Interaction, (Changed<Interaction>, With<HamburgerButton>)>,
    mut menu: ResMut<MenuState>,
) {
    for interaction in query.iter() {
        if *interaction == Interaction::Pressed {
            menu.is_open = !menu.is_open;
        }
    }
}

// Renamed from react_to_theme_change to react_to_ui_changes
pub fn react_to_ui_changes(
    mut commands: Commands,
    theme: Res<ThemeMode>,
    menu: Res<MenuState>,
    ui_roots: Query<Entity, With<UiRoot>>,
) {
    let theme_changed = theme.is_changed() && !theme.is_added();
    let menu_changed  = menu.is_changed() && !menu.is_added();

    if theme_changed || menu_changed {
        // Unmount old UI safely
        for entity in ui_roots.iter() {
            commands.entity(entity).despawn_related::<Children>();
            commands.entity(entity).despawn();
        }

        // Remount new UI
        crate::viz::hud::layout::build_hud(&mut commands, *theme);
        crate::viz::menu::layout::build_menu(&mut commands, *theme, menu.is_open);
        crate::viz::hud::scoreboard::build_scoreboard(&mut commands, *theme);
    }
}

pub fn handle_theme_toggle_button(
    query: Query<&Interaction, (Changed<Interaction>, With<ThemeToggleButton>)>,
    mut theme: ResMut<ThemeMode>,
) {
    for interaction in query.iter() {
        if *interaction == Interaction::Pressed {
            *theme = match *theme {
                ThemeMode::Dark  => ThemeMode::Light,
                ThemeMode::Light => ThemeMode::Dark,
            };
        }
    }
}

pub fn handle_viz_toggle_button(
    mut query:  Query<&Interaction, (Changed<Interaction>, With<VizToggleButton>)>,
    mut config: ResMut<DebugVizConfig>,
) {
    for interaction in query.iter_mut() {
        if *interaction == Interaction::Pressed {
            config.show_global = !config.show_global;
        }
    }
}

pub fn handle_pause_button(
    query:   Query<&Interaction, (Changed<Interaction>, With<PauseButtonMarker>)>,
    mut cfg: ResMut<SimConfig>,
) {
    for interaction in query.iter() {
        if *interaction == Interaction::Pressed {
            cfg.paused = !cfg.paused;
        }
    }
}

pub fn handle_speed_buttons(
    dec_q:     Query<&Interaction, (Changed<Interaction>, With<SpeedDecreaseButton>)>,
    inc_q:     Query<&Interaction, (Changed<Interaction>, With<SpeedIncreaseButton>)>,
    rst_q:     Query<&Interaction, (Changed<Interaction>, With<SpeedResetButton>)>,
    mut cfg:   ResMut<SimConfig>,
    mut timer: ResMut<TickTimer>,
) {
    let mut changed = false;

    for i in dec_q.iter() {
        if *i == Interaction::Pressed {
            let idx = AVAILABLE_SPEEDS.iter().position(|&s| (s - cfg.ticks_per_second).abs() < f32::EPSILON).unwrap_or(0);
            cfg.ticks_per_second = AVAILABLE_SPEEDS[idx.saturating_sub(1)];
            changed = true;
        }
    }
    for i in inc_q.iter() {
        if *i == Interaction::Pressed {
            let idx = AVAILABLE_SPEEDS.iter().position(|&s| (s - cfg.ticks_per_second).abs() < f32::EPSILON).unwrap_or(0);
            cfg.ticks_per_second = AVAILABLE_SPEEDS[(idx + 1).min(AVAILABLE_SPEEDS.len() - 1)];
            changed = true;
        }
    }
    for i in rst_q.iter() {
        if *i == Interaction::Pressed {
            cfg.ticks_per_second = 10.0;
            changed = true;
        }
    }

    if changed {
        timer.0 = Timer::from_seconds(1.0 / cfg.ticks_per_second, TimerMode::Repeating);
    }
}

pub fn update_speed_label(
    cfg:       Res<SimConfig>,
    mut query: Query<&mut Text, With<CurrentSpeedLabel>>,
) {
    if !cfg.is_changed() { return; }
    for mut text in query.iter_mut() {
        *text = Text::new(format!("{}x", cfg.ticks_per_second as i32));
    }
}

pub fn update_button_styles(
    mut query: Query<(
        &Interaction,
        &mut BackgroundColor,
        Option<&PauseButtonMarker>,
        Option<&VizToggleButton>,
        Option<&HamburgerButton> // <-- Add Hamburger to the style check
    ), (Changed<Interaction>, With<Button>)>,
    cfg:        Res<SimConfig>,
    viz_config: Res<DebugVizConfig>,
    theme:      Res<ThemeMode>,
    menu:       Res<MenuState>,      // <-- Read menu state
) {
    let idle    = ThemeColor::ButtonIdle.resolve(*theme);
    let hover   = ThemeColor::ButtonHover.resolve(*theme);
    let running = ThemeColor::Success.resolve(*theme);
    let paused  = ThemeColor::Error.resolve(*theme);

    for (interaction, mut color, pause_btn, viz_btn, ham_btn) in query.iter_mut() {
        if pause_btn.is_some() {
            color.0 = match (*interaction, cfg.paused) {
                (_, true)                     => paused,
                (Interaction::Hovered, false) => hover,
                (Interaction::Pressed, false) => hover,
                (Interaction::None,    false) => running,
            };
        } else if viz_btn.is_some() {
            color.0 = match (*interaction, viz_config.show_global) {
                (Interaction::Hovered, _)  => hover,
                (Interaction::Pressed, _)  => hover,
                (Interaction::None, true)  => idle,
                (Interaction::None, false) => paused,
            };
        } else if ham_btn.is_some() {
            color.0 = match (*interaction, menu.is_open) {
                (Interaction::Hovered, _)  => hover,
                (Interaction::Pressed, _)  => hover,
                (Interaction::None, true)  => hover, // Keep highlighted when open
                (Interaction::None, false) => idle,
            };
        } else {
            color.0 = match *interaction {
                Interaction::Hovered | Interaction::Pressed => hover,
                Interaction::None                           => idle,
            };
        }
    }
}