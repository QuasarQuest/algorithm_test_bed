// src/sim/config.rs
// SimConfig owns simulation state — tick count lives here, not in agent/systems.

use bevy::prelude::*;
use crate::config;

pub const AVAILABLE_SPEEDS: &[f32] = &[1.0, 2.0, 5.0, 10.0, 20.0, 30.0, 60.0];

#[derive(Resource)]
pub struct SimConfig {
    pub ticks_per_second: f32,
    pub paused:           bool,
    pub tick:             u64,   // ← moved from agent/systems::TickCount
}

impl Default for SimConfig {
    fn default() -> Self {
        Self {
            ticks_per_second: config::DEFAULT_TICKS_PER_SECOND,
            paused:           false,
            tick:             0,
        }
    }
}
