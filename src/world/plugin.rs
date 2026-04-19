// src/world/plugin.rs

use bevy::prelude::*;
use super::grid::Grid;
use super::map_config::{MapConfig, TileKind};
use super::tile::Tile;
use crate::config;

fn spawn_world(mut grid: ResMut<Grid>) {
    let cfg = MapConfig::load("assets/maps/default.ron");

    for fixed in &cfg.fixed {
        let tile = match fixed.tile {
            TileKind::Free     => Tile::Free,
            TileKind::Obstacle => Tile::Obstacle,
            TileKind::Gold     => Tile::Gold,
            TileKind::Base     => Tile::Base,
        };
        grid.set(fixed.x as i32, fixed.y as i32, tile);
    }

    let mut placed = 0;
    while placed < cfg.random_obstacles {
        let x = rand::random_range(0..cfg.width as i32);
        let y = rand::random_range(0..cfg.height as i32);
        if grid.get(x, y) == Some(Tile::Free) {
            grid.set(x, y, Tile::Obstacle);
            placed += 1;
        }
    }

    let mut placed = 0;
    while placed < cfg.random_gold {
        let x = rand::random_range(0..cfg.width as i32);
        let y = rand::random_range(0..cfg.height as i32);
        if grid.get(x, y) == Some(Tile::Free) {
            grid.set(x, y, Tile::Gold);
            placed += 1;
        }
    }
}

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Grid::new(config::GRID_W, config::GRID_H))
            .add_systems(Startup, spawn_world);
    }
}