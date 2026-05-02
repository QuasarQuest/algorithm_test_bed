// src/world/map_config.rs

use bevy::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum TileKind { Free, Obstacle, Gold, Base }

#[derive(Debug, Deserialize, Clone)]
pub struct FixedTile {
    pub x:    usize,
    pub y:    usize,
    pub tile: TileKind,
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum AgentKind { Random, AStar, DStarLite }

#[derive(Debug, Deserialize, Clone)]
pub struct AgentSpawn {
    pub x:    i32,
    pub y:    i32,
    pub kind: AgentKind,
    /// Optional team id — defaults to 0 (Red) if absent.
    #[serde(default)]
    pub team: Option<u8>,
}

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum ObstacleKind { Block, Wall, Scatter }

#[derive(Debug, Deserialize, Clone)]
pub struct ObstacleCluster {
    pub kind:  ObstacleKind,
    pub count: usize,
    pub size:  (usize, usize),
}

#[derive(Debug, Deserialize, Clone, Resource)]
pub struct MapConfig {
    pub width:             usize,
    pub height:            usize,
    pub random_gold:       usize,
    pub fixed:             Vec<FixedTile>,
    pub agents:            Vec<AgentSpawn>,
    pub obstacle_clusters: Vec<ObstacleCluster>,
}

impl MapConfig {
    pub fn load(path: &str) -> Self {
        let text = std::fs::read_to_string(path)
            .unwrap_or_else(|_| panic!("Cannot read map config: {path}"));
        ron::from_str(&text)
            .unwrap_or_else(|e| panic!("Cannot parse map config {path}: {e}"))
    }
}