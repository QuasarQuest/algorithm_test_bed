// src/sim/schedule.rs

use bevy::ecs::schedule::ScheduleLabel;

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OnSimTick;