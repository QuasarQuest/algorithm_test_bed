// src/agent/impl/random_agent.rs

use crate::agent::action::{Action, Dir};
use crate::agent::brain::Agent;
use crate::agent::observation::Observation;
use crate::world::tile::Tile;

pub struct RandomAgent;

impl Agent for RandomAgent {
    fn name(&self) -> &str {
        "Random Walker"
    }

    fn act(&mut self, obs: &Observation<'_>) -> Action {
        let on_gold = obs.is_tile(obs.pos, Tile::Gold);
        let on_base = obs.is_tile(obs.pos, Tile::Base);

        if on_gold && !obs.gold_carried.is_full() {
            return Action::Pickup;
        }

        if on_base && !obs.gold_carried.is_empty() {
            return Action::Drop;
        }

        let dirs = Dir::all();
        Action::Move(dirs[rand::random_range(0..dirs.len())])
    }
}