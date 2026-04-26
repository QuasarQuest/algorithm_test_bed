// src/agent/registry.rs
//
// Maps AgentKind → Box<dyn Agent>.
// Adding a new algorithm = adding one line here.
// systems.rs and spawn.rs never need to change for new agent types.

use crate::world::map_config::AgentKind;
use super::brain::Agent;
use super::r#impl::random_agent::RandomAgent;
use super::r#impl::astar_agent::AStarAgent;
use super::r#impl::dstar_agent::DStarAgent;

// Agent colours — indexed by AgentKind discriminant order
// Add a colour here when you add a new AgentKind variant.
pub fn agent_color(kind: AgentKind) -> bevy::prelude::Color {
    match kind {
        AgentKind::Random    => bevy::prelude::Color::srgb(0.95, 0.35, 0.25),
        AgentKind::AStar     => bevy::prelude::Color::srgb(0.25, 0.65, 0.95),
        AgentKind::DStarLite => bevy::prelude::Color::srgb(0.70, 0.20, 0.95), // Purple
    }
}

/// Returns a fresh brain for the given agent kind.
/// To add D* Lite: add `AgentKind::DStarLite` to map_config and one arm here.
pub fn make_brain(kind: AgentKind) -> Box<dyn Agent> {
    match kind {
        AgentKind::Random    => Box::new(RandomAgent),
        AgentKind::AStar     => Box::new(AStarAgent::new()),
        AgentKind::DStarLite => Box::new(DStarAgent::new()),
    }
}

/// Human-readable prefix for agent labels.
pub fn agent_name_prefix(kind: AgentKind) -> &'static str {
    match kind {
        AgentKind::Random    => "Random",
        AgentKind::AStar     => "A*",
        AgentKind::DStarLite => "D* Lite",
    }
}
