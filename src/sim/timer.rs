// src/sim/timer.rs

use bevy::prelude::*;
use crate::config;

#[derive(Resource)]
pub struct TickTimer(pub Timer);

impl Default for TickTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(
            1.0 / config::DEFAULT_TICKS_PER_SECOND,
            TimerMode::Repeating,
        ))
    }
}