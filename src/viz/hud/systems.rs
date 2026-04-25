// src/viz/hud/systems.rs

use bevy::prelude::*;
use crate::sim::config::{SimConfig, AVAILABLE_SPEEDS};
use crate::sim::timer::TickTimer;
use super::components::*;
use super::layout::{
    BTN_HOVER, BTN_IDLE, BTN_PAUSED, BTN_RUNNING,
    TEXT_PAUSED, TEXT_RUNNING,
};

pub fn count_ticks(mut count: ResMut<TickCount>) {
    count.0 += 1;
}

pub fn handle_pause_button(
    query:        Query<&Interaction, (Changed<Interaction>, With<PauseButtonMarker>)>,
    mut bg:       Single<&mut BackgroundColor, With<PauseButtonMarker>>,
    mut label_q:  Query<(&mut Text, &mut TextColor), With<PauseLabelMarker>>,
    mut cfg:      ResMut<SimConfig>,
) {
    for interaction in query.iter() {
        if *interaction == Interaction::Pressed {
            cfg.paused = !cfg.paused;

            bg.0 = if cfg.paused { BTN_PAUSED } else { BTN_RUNNING };

            for (mut text, mut color) in label_q.iter_mut() {
                *text  = Text::new(if cfg.paused { "⏸  Paused" } else { "▶  Running" });
                color.0 = if cfg.paused { TEXT_PAUSED } else { TEXT_RUNNING };
            }
        }
    }
}

pub fn handle_speed_buttons(
    dec_q: Query<&Interaction, (Changed<Interaction>, With<SpeedDecreaseButton>)>,
    inc_q: Query<&Interaction, (Changed<Interaction>, With<SpeedIncreaseButton>)>,
    rst_q: Query<&Interaction, (Changed<Interaction>, With<SpeedResetButton>)>,
    mut cfg:   ResMut<SimConfig>,
    mut timer: ResMut<TickTimer>,
) {
    let mut changed = false;

    for interaction in dec_q.iter() {
        if *interaction == Interaction::Pressed {
            let idx = AVAILABLE_SPEEDS.iter()
                .position(|&s| (s - cfg.ticks_per_second).abs() < f32::EPSILON)
                .unwrap_or(0);
            cfg.ticks_per_second = AVAILABLE_SPEEDS[idx.saturating_sub(1)];
            changed = true;
        }
    }

    for interaction in inc_q.iter() {
        if *interaction == Interaction::Pressed {
            let idx = AVAILABLE_SPEEDS.iter()
                .position(|&s| (s - cfg.ticks_per_second).abs() < f32::EPSILON)
                .unwrap_or(0);
            cfg.ticks_per_second = AVAILABLE_SPEEDS[(idx + 1).min(AVAILABLE_SPEEDS.len() - 1)];
            changed = true;
        }
    }

    for interaction in rst_q.iter() {
        if *interaction == Interaction::Pressed {
            cfg.ticks_per_second = 10.0;
            changed = true;
        }
    }

    if changed {
        timer.0 = Timer::from_seconds(1.0 / cfg.ticks_per_second, TimerMode::Repeating);
    }
}

// Keeps speed label in sync whether clicked via UI or keyboard shortcuts
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
    ), (Changed<Interaction>, With<Button>)>,
    cfg: Res<SimConfig>,
) {
    for (interaction, mut color, pause_btn) in query.iter_mut() {
        color.0 = if pause_btn.is_some() {
            match (*interaction, cfg.paused) {
                (_, true)                     => BTN_PAUSED,
                (Interaction::Hovered, false) => BTN_HOVER,
                (Interaction::Pressed, false) => BTN_HOVER,
                (Interaction::None, false)    => BTN_RUNNING, // stays green when idle
            }
        } else {
            match *interaction {
                Interaction::Hovered | Interaction::Pressed => BTN_HOVER,
                Interaction::None                           => BTN_IDLE,
            }
        };
    }
}

pub fn update_tick_label(
    count:     Res<TickCount>,
    mut query: Query<&mut Text, With<TickLabelMarker>>,
) {
    if !count.is_changed() { return; }
    for mut text in query.iter_mut() {
        *text = Text::new(format!("{}", count.0));
    }
}