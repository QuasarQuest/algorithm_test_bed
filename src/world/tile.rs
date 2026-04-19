// src/world/tile.rs

use bevy::prelude::Color;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tile {
    Free,
    Obstacle,
    Gold,
    Base,
}

impl Tile {
    pub fn color(self) -> Color {
        match self {
            Tile::Free     => Color::srgb(0.12, 0.12, 0.12),
            Tile::Obstacle => Color::srgb(0.35, 0.35, 0.35),
            Tile::Gold     => Color::srgb(0.90, 0.75, 0.10),
            Tile::Base     => Color::srgb(0.10, 0.45, 0.90),
        }
    }

    pub fn is_walkable(self) -> bool {
        !matches!(self, Tile::Obstacle)
    }
}