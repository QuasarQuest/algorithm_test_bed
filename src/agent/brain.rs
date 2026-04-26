// src/agent/brain.rs
//
// The Agent trait — the single boundary every algorithm must implement.
// No Bevy, no Grid, no rendering knowledge in here.

use bevy::prelude::Component;
use super::action::Action;
use super::observation::Observation;

// ── Agent trait ───────────────────────────────────────────────────────────────

pub trait Agent: Send + Sync {
    fn name(&self) -> &str;
    fn act(&mut self, obs: &Observation<'_>) -> Action;

    /// Expose internal algorithm state for debug overlays.
    /// Default: no debug info.
    fn debug_info(&self) -> Option<DebugInfo> { None }

    /// Reset internal state — called at episode boundaries.
    fn reset(&mut self) {}
}

// ── Debug info ────────────────────────────────────────────────────────────────
//
// Each variant carries exactly what its viz renderer needs.
// The viz layer matches on this enum — no downcasting.
// Add a variant for each new algorithm that has visual state to expose.

#[derive(Clone, Debug)]
pub enum DebugInfo {
    AStar {
        open:   Vec<(i32, i32)>,
        closed: Vec<(i32, i32)>,
        path:   Vec<(i32, i32)>,
    },
    DStarLite {
        open:      Vec<(i32, i32)>,
        obstacles: Vec<(i32, i32)>,
        path:      Vec<(i32, i32)>,
    },
    ParticleFilter {
        /// (x, y, weight) — weight in [0,1] for heatmap intensity
        particles: Vec<(i32, i32, f32)>,
    },
    BehaviorTree {
        active_node: String,
    },
    MarkovChain {
        /// (state_pos, probability) for the current distribution
        distribution: Vec<(i32, i32, f32)>,
    },
}

// ── Brain component ───────────────────────────────────────────────────────────

#[derive(Component)]
pub struct Brain(pub Box<dyn Agent>);

impl Brain {
    /// Wrap a concrete agent — used when you know the type at compile time.
    pub fn new(agent: impl Agent + 'static) -> Self {
        Self(Box::new(agent))
    }

    /// Wrap an already-boxed agent — used by the registry.
    pub fn new_boxed(agent: Box<dyn Agent>) -> Self {
        Self(agent)
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

    pub fn reset(&mut self) {
        self.0.reset()
    }
}
