// src/viz/core_ui/text.rs

use bevy::prelude::*;
use bevy::ecs::hierarchy::ChildSpawnerCommands;

/// Spawns standard text.
pub fn spawn_label(parent: &mut ChildSpawnerCommands, text: &str, color: Color, size: f32) {
    parent.spawn((
        Text::new(text),
        TextFont { font_size: size, ..default() },
        TextColor(color),
    ));
}

/// Spawns text with an ECS Component attached so it can be updated dynamically.
pub fn spawn_marked_label<T: Component>(
    parent: &mut ChildSpawnerCommands,
    text: &str,
    color: Color,
    size: f32,
    marker: T
) {
    parent.spawn((
        Text::new(text),
        TextFont { font_size: size, ..default() },
        TextColor(color),
        marker,
    ));
}