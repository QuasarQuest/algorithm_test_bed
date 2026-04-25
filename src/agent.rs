// src/agent.rs

pub mod action;
pub mod brain;
pub mod components;
pub mod observation;
pub mod spawn;
pub mod systems;
pub mod r#impl;

pub use components::{AgentLabel, GoldCarried, GridPos, Score};

use bevy::prelude::*;
use crate::sim::OnSimTick; // Import your custom schedule
use systems::{tick_agents, apply_actions, spawn_agents, TickCount};

pub struct AgentPlugin;

impl Plugin for AgentPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TickCount>()
            .add_systems(Startup, spawn_agents)
            // Move from Update to OnSimTick!
            .add_systems(OnSimTick, (tick_agents, apply_actions).chain());
    }
}