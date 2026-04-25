// src/main.rs

mod agent;
mod algorithm;
mod config;
mod sim;
mod viz;
mod world;

use bevy::prelude::*;
use sim::SimPlugin;
use viz::VizPlugin;
use world::WorldPlugin;
use agent::AgentPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title:      config::WINDOW_TITLE.into(),
                // Pass the u32 constants directly for Bevy 0.15+
                resolution: bevy::window::WindowResolution::new(
                    config::WINDOW_WIDTH,
                    config::WINDOW_HEIGHT,
                ),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((WorldPlugin, SimPlugin, VizPlugin, AgentPlugin))
        .run();
}