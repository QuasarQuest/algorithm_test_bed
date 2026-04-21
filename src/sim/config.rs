// src/sim/config.rs

use bevy::prelude::*;
use crate::config;

pub const AVAILABLE_SPEEDS: &[f32] = &[2.0, 5.0, 10.0, 15.0, 25.0, 50.0];

#[derive(Resource)]
pub struct SimConfig {
    pub ticks_per_second: f32,
    pub paused:           bool,
}

impl Default for SimConfig {
    fn default() -> Self {
        Self {
            ticks_per_second: config::DEFAULT_TICKS_PER_SECOND,
            paused:           false,
        }
    }
}