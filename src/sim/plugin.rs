// src/sim/plugin.rs

use bevy::prelude::*;
use super::config::SimConfig;
use super::schedule::OnSimTick;
use super::timer::TickTimer;
use crate::config;

fn fire_sim_tick(world: &mut World) {
    let delta = world.resource::<Time>().delta();

    let should_tick = {
        let cfg = world.resource::<SimConfig>();
        if cfg.paused {
            false
        } else {
            world.resource_mut::<TickTimer>().0.tick(delta).just_finished()
        }
    };

    if should_tick {
        world.resource_mut::<SimConfig>().tick += 1;
        world.run_schedule(OnSimTick);
    }
}

fn handle_input(
    keys:      Res<ButtonInput<KeyCode>>,
    mut cfg:   ResMut<SimConfig>,
    mut timer: ResMut<TickTimer>,
) {
    if keys.just_pressed(KeyCode::Space) {
        cfg.paused = !cfg.paused;
    }
    if keys.just_pressed(KeyCode::KeyF) {
        cfg.ticks_per_second =
            (cfg.ticks_per_second + config::SPEED_STEP).min(config::MAX_TICKS_PER_SECOND);
        timer.0 = Timer::from_seconds(1.0 / cfg.ticks_per_second, TimerMode::Repeating);
    }
    if keys.just_pressed(KeyCode::KeyS) {
        cfg.ticks_per_second =
            (cfg.ticks_per_second - config::SPEED_STEP).max(config::MIN_TICKS_PER_SECOND);
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
