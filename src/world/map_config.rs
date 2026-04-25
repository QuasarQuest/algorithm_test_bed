// src/world/map_config.rs

use bevy::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum TileKind {
    Free,
    Obstacle,
    Gold,
    Base,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FixedTile {
    pub x:    usize,
    pub y:    usize,
    pub tile: TileKind,
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum AgentKind {
    Random,
    AStar,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AgentSpawn {
    pub x:    i32,
    pub y:    i32,
    pub kind: AgentKind,
}

// ── Obstacle clusters ─────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum ObstacleKind {
    /// Solid filled rectangle
    Block,
    /// Single-cell-thick line, horizontal or vertical (chosen randomly)
    Wall,
    /// Individual scattered cells
    Scatter,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ObstacleCluster {
    pub kind:  ObstacleKind,
    /// How many clusters/walls/dots to place
    pub count: usize,
    /// Bounding box — Block uses both, Wall uses width as length, Scatter ignores
    pub size:  (usize, usize),
}

// ── Root config ───────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize, Clone, Resource)]
pub struct MapConfig {
    pub width:              usize,
    pub height:             usize,
    pub random_gold:        usize,
    pub fixed:              Vec<FixedTile>,
    pub agents:             Vec<AgentSpawn>,
    pub obstacle_clusters:  Vec<ObstacleCluster>,
}

impl MapConfig {
    pub fn load(path: &str) -> Self {
        let text = std::fs::read_to_string(path)
            .unwrap_or_else(|_| panic!("Cannot read map config: {path}"));
        ron::from_str(&text)
            .unwrap_or_else(|e| panic!("Cannot parse map config {path}: {e}"))
    }
}
