// src/agent/brain.rs
//
// The Agent trait — the single boundary every algorithm must implement.
// Nothing in here knows about Bevy, Grid layout, or rendering.
//
// A brain is stateful (hence &mut self) so agents can remember
// planned paths, Q-tables, particle clouds, etc.

use bevy::prelude::Component;
use super::action::Action;
use super::observation::Observation;

// ── Agent trait ───────────────────────────────────────────────────────────────

pub trait Agent: Send + Sync {
    /// Human-readable name shown in UI and debug overlays.
    fn name(&self) -> &str;

    // Add the <'_> lifetime here
    fn act(&mut self, obs: &Observation<'_>) -> Action;
    fn debug_info(&self) -> Option<DebugInfo> { None }
    fn reset(&mut self) {}
}

// ── Debug info — algorithm internals for the viz overlay ─────────────────────
//
// Each algorithm variant carries exactly the data its renderer needs.
// The viz layer matches on this enum — no downcasting required.

#[derive(Clone, Debug)]
pub enum DebugInfo {
    AStar {
        /// Nodes currently in the open set.
        open: Vec<(i32, i32)>,
        /// Nodes already evaluated.
        closed: Vec<(i32, i32)>,
        /// Current planned path from agent to goal.
        path: Vec<(i32, i32)>,
    },
    ParticleFilter {
        /// (x, y, weight) — weight in [0, 1], used for heatmap intensity.
        particles: Vec<(i32, i32, f32)>,
    },
    BehaviorTree {
        /// Name of the currently active leaf node.
        active_node: String,
    },
}

// ── Brain component — wraps a boxed Agent trait object ───────────────────────
//
// This is what Bevy stores as a Component on each agent Entity.
// The sim system queries for Brain and calls act() each tick.
//
// Box<dyn Agent> means each entity can have a *different* brain type
// while still living in the same ECS query.

#[derive(Component)]
pub struct Brain(pub Box<dyn Agent>);

impl Brain {
    pub fn new(agent: impl Agent + 'static) -> Self {
        Self(Box::new(agent))
    }

    pub fn act(&mut self, obs: &Observation<'_>) -> Action {
        self.0.act(obs)
    }
    pub fn name(&self) -> &str {
        self.0.name()
    }

    pub fn debug_info(&self) -> Option<DebugInfo> {
        self.0.debug_info()
    }
}