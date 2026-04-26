// src/world/coords.rs
//
// GridPos is world geometry, not agent data.
// Pathfinders, the grid, and agents all use it — it belongs here.

use bevy::prelude::Component;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GridPos {
    pub x: i32,
    pub y: i32,
}

impl GridPos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn apply_delta(&self, dx: i32, dy: i32) -> Self {
        Self::new(self.x + dx, self.y + dy)
    }

    /// Squared Chebyshev distance — cheap, no sqrt, good for comparisons.
    pub fn dist_sq(self, other: GridPos) -> i32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx * dx + dy * dy
    }
}
