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

    fn act(&mut self, obs: &Observation) -> Action {
        let on_gold = obs.visible_cells.iter().any(|c| c.pos == obs.pos && c.tile == Tile::Gold);
        let on_base = obs.visible_cells.iter().any(|c| c.pos == obs.pos && c.tile == Tile::Base);

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