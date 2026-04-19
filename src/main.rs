// src/main.rs

mod agent;
mod config;
mod sim;
mod viz;
mod world;

use bevy::prelude::*;
use agent::r#impl::random_agent::RandomAgentPlugin;
use sim::SimPlugin;
use viz::VizPlugin;
use world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title:      config::WINDOW_TITLE.into(),
                resolution: bevy::window::WindowResolution::new(
                    config::WINDOW_WIDTH,
                    config::WINDOW_HEIGHT,
                ),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((WorldPlugin, SimPlugin, VizPlugin, RandomAgentPlugin))
        .run();
}