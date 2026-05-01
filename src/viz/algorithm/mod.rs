// src/viz/algorithm/mod.rs

use bevy::prelude::*;
use crate::agent::brain::Brain;
use crate::agent::components::GridPos;
use crate::viz::grid_offset::GridOffset;
use crate::viz::menu::components::HideViz; // <-- Import marker

pub fn draw_agent_debug(
    mut gizmos: Gizmos,
    offset: Res<GridOffset>,
    query: Query<(&GridPos, &Brain), Without<HideViz>>, // <-- Filter out hidden agents
) {
    for (pos, brain) in query.iter() {
        if let Some(drawer) = brain.debug_draw() {
            drawer.draw(*pos, &mut gizmos, &offset);
        }
    }
}