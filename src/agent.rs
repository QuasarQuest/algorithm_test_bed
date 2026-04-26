// src/agent.rs

pub mod action;
pub mod brain;
pub mod components;
pub mod observation;
pub mod registry;
pub mod spawn;
pub mod systems;
pub mod r#impl;
mod debug;

use bevy::prelude::*;
use crate::sim::OnSimTick;
use systems::{tick_agents, apply_actions};
use spawn::spawn_agents;

pub struct AgentPlugin;

impl Plugin for AgentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_agents)
           .add_systems(OnSimTick, (tick_agents, apply_actions).chain());
    }
}
