// src/agent/brain.rs
//
// The Agent trait — the single boundary every algorithm must implement.
// No Grid or rendering knowledge in here.

use bevy::prelude::Component;
use super::action::Action;
use super::observation::Observation;
use super::debug::DebugDraw; // <-- Import the new trait

pub trait Agent: Send + Sync {
    fn name(&self) -> &str;
    fn act(&mut self, obs: &Observation<'_>) -> Action;

    /// Expose internal algorithm state for debug overlays.
    /// Default: no debug info.
    fn debug_draw(&self) -> Option<Box<dyn DebugDraw>> { None } // <-- Clean boundary

    fn reset(&mut self) {}
}

#[derive(Component)]
pub struct Brain(pub Box<dyn Agent>);

impl Brain {
    pub fn new(agent: impl Agent + 'static) -> Self { Self(Box::new(agent)) }
    pub fn new_boxed(agent: Box<dyn Agent>) -> Self { Self(agent) }

    pub fn act(&mut self, obs: &Observation<'_>) -> Action { self.0.act(obs) }
    pub fn name(&self) -> &str { self.0.name() }

    // Pass the trait object through
    pub fn debug_draw(&self) -> Option<Box<dyn DebugDraw>> { self.0.debug_draw() }

    pub fn reset(&mut self) { self.0.reset() }
}