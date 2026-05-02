// src/agent/planning/random_agent.rs

use crate::agent::action::{Action, Dir};
use crate::agent::brain::Agent;
use crate::agent::observation::Observation;
use crate::world::tile::Tile;

pub struct RandomAgent;

impl Agent for RandomAgent {
    fn name(&self) -> &str { "Random Walker" }

    fn act(&mut self, obs: &Observation<'_>) -> Action {
        if obs.is_tile(obs.pos, Tile::Gold) && !obs.gold_carried.is_full() {
            return Action::Pickup;
        }
        if obs.is_tile(obs.pos, Tile::Base) && !obs.gold_carried.is_empty() {
            return Action::Drop;
        }
        let dirs = Dir::all();
        Action::Move(dirs[rand::random_range(0..dirs.len())])
    }
}