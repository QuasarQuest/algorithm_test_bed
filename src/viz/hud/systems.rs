// src/viz/hud/systems.rs

use bevy::prelude::*;
use crate::sim::config::{SimConfig, AVAILABLE_SPEEDS};
use crate::sim::timer::TickTimer;
use super::components::*;
use super::layout::{BTN_HOVER, BTN_IDLE, BTN_PAUSED};

pub fn count_ticks(mut count: ResMut<TickCount>) {
    count.0 += 1;
}

pub fn handle_pause_button(
    query: Query<&Interaction, (Changed<Interaction>, With<PauseButtonMarker>)>,
    mut text_query: Query<&mut Text, With<PauseLabelMarker>>,
    mut cfg: ResMut<SimConfig>,
) {
    for interaction in query.iter() {
        if *interaction == Interaction::Pressed {
            cfg.paused = !cfg.paused;
            for mut text in text_query.iter_mut() {
                *text = Text::new(if cfg.paused { "Paused" } else { "Running" });
            }
        }
    }
}

pub fn handle_speed_buttons(
    dec_q: Query<&Interaction, (Changed<Interaction>, With<SpeedDecreaseButton>)>,
    inc_q: Query<&Interaction, (Changed<Interaction>, With<SpeedIncreaseButton>)>,
    rst_q: Query<&Interaction, (Changed<Interaction>, With<SpeedResetButton>)>,
    mut cfg: ResMut<SimConfig>,
    mut timer: ResMut<TickTimer>,
) {
    let mut changed = false;

    // Handle Minus Button
    for interaction in dec_q.iter() {
        if *interaction == Interaction::Pressed {
            let current_idx = AVAILABLE_SPEEDS.iter()
                .position(|&s| (s - cfg.ticks_per_second).abs() < f32::EPSILON)
                .unwrap_or(0);
            let next_idx = current_idx.saturating_sub(1);
            cfg.ticks_per_second = AVAILABLE_SPEEDS[next_idx];
            changed = true;
        }
    }

    // Handle Plus Button
    for interaction in inc_q.iter() {
        if *interaction == Interaction::Pressed {
            let current_idx = AVAILABLE_SPEEDS.iter()
                .position(|&s| (s - cfg.ticks_per_second).abs() < f32::EPSILON)
                .unwrap_or(0);
            let next_idx = (current_idx + 1).min(AVAILABLE_SPEEDS.len() - 1);
            cfg.ticks_per_second = AVAILABLE_SPEEDS[next_idx];
            changed = true;
        }
    }

    // Handle Reset Button
    for interaction in rst_q.iter() {
        if *interaction == Interaction::Pressed {
            cfg.ticks_per_second = 10.0; // The defined normal speed
            changed = true;
        }
    }

    if changed {
        timer.0 = Timer::from_seconds(1.0 / cfg.ticks_per_second, TimerMode::Repeating);
    }
}

// Keeps the center label perfectly in sync whether you click UI buttons or use keyboard F/S
pub fn update_speed_label(
    cfg: Res<SimConfig>,
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
    ), (Changed<Interaction>, With<Button>)>,
    cfg: Res<SimConfig>,
) {
    for (interaction, mut color, pause_btn) in query.iter_mut() {
        if pause_btn.is_some() {
            *color = match (*interaction, cfg.paused) {
                (_, true) => BTN_PAUSED.into(),
                (Interaction::Hovered, false) => BTN_HOVER.into(),
                (Interaction::None, false) => BTN_IDLE.into(),
                (Interaction::Pressed, false) => BTN_HOVER.into(),
            };
        } else {
            // All standard buttons (+, -, Reset) simply highlight on hover
            *color = match *interaction {
                Interaction::Hovered | Interaction::Pressed => BTN_HOVER.into(),
                Interaction::None => BTN_IDLE.into(),
            };
        }
    }
}

pub fn update_tick_label(
    count: Res<TickCount>,
    mut query: Query<&mut Text, With<TickLabelMarker>>,
) {
    if !count.is_changed() { return; }
    for mut text in query.iter_mut() {
        *text = Text::new(format!("{}", count.0));
    }
}