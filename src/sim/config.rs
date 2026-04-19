// src/sim/config.rs

use bevy::prelude::*;
use crate::config;

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