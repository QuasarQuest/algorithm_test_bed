// src/sim/plugin.rs

use bevy::prelude::*;
use super::config::{SimConfig, AVAILABLE_SPEEDS};
use super::schedule::OnSimTick;
use super::timer::TickTimer;

fn fire_sim_tick(world: &mut World) {
    let delta = world.resource::<Time>().delta();

    let ticks_to_run = {
        let paused = world.resource::<SimConfig>().paused;
        if paused {
            0
        } else {
            let mut timer = world.resource_mut::<TickTimer>();
            timer.0.tick(delta);
            timer.0.times_finished_this_tick()
        }
    };

    // Cap the maximum ticks per frame to prevent the game from freezing
    // if the simulation gets too heavy to process in real-time.
    let safe_ticks = ticks_to_run.min(100);

    for _ in 0..safe_ticks {
        world.run_schedule(OnSimTick);
    }
}

fn handle_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut cfg: ResMut<SimConfig>,
    mut timer: ResMut<TickTimer>,
) {
    if keys.just_pressed(KeyCode::Space) {
        cfg.paused = !cfg.paused;
    }

    if keys.just_pressed(KeyCode::KeyF) {
        let current_idx = AVAILABLE_SPEEDS.iter()
            .position(|&s| (s - cfg.ticks_per_second).abs() < f32::EPSILON)
            .unwrap_or(0);
        let next_idx = (current_idx + 1).min(AVAILABLE_SPEEDS.len() - 1);

        cfg.ticks_per_second = AVAILABLE_SPEEDS[next_idx];
        timer.0 = Timer::from_seconds(1.0 / cfg.ticks_per_second, TimerMode::Repeating);
    }

    if keys.just_pressed(KeyCode::KeyS) {
        let current_idx = AVAILABLE_SPEEDS.iter()
            .position(|&s| (s - cfg.ticks_per_second).abs() < f32::EPSILON)
            .unwrap_or(0);
        let next_idx = current_idx.saturating_sub(1);

        cfg.ticks_per_second = AVAILABLE_SPEEDS[next_idx];
        timer.0 = Timer::from_seconds(1.0 / cfg.ticks_per_second, TimerMode::Repeating);
    }
}

pub struct SimPlugin;

impl Plugin for SimPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SimConfig>()
            .init_resource::<TickTimer>()
            .init_schedule(OnSimTick)
            .add_systems(Update, handle_input)
            .add_systems(Update, fire_sim_tick);
    }
}