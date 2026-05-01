// src/viz/algorithm/mod.rs

use bevy::prelude::*;
use crate::agent::brain::Brain;
use crate::agent::components::GridPos;
use crate::viz::grid_offset::GridOffset;
use crate::viz::menu::components::DebugVizConfig;

/// A universal system that draws any agent's debug info without needing
/// to know what algorithm is running inside its brain.
pub fn draw_agent_debug(
    mut gizmos: Gizmos,
    offset: Res<GridOffset>,
    config: Res<DebugVizConfig>,
    query: Query<(&GridPos, &Brain)>,
) {
    if !config.show_global { return; }

    for (pos, brain) in query.iter() {
        if let Some(drawer) = brain.debug_draw() {
            drawer.draw(*pos, &mut gizmos, &offset);
        }
    }
}