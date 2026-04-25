// src/agent.rs

pub mod action;
pub mod brain;
pub mod components;
pub mod observation;
pub mod spawn;
pub mod systems;
pub mod r#impl;

use bevy::prelude::*;
use crate::sim::OnSimTick;
// Make sure TickCount is imported!
use systems::{tick_agents, apply_actions, spawn_agents, TickCount};

pub struct AgentPlugin;

impl Plugin for AgentPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TickCount>()
            .add_systems(Startup, spawn_agents)
            .add_systems(OnSimTick, (tick_agents, apply_actions).chain());
    }
}