// src/agent/action.rs
//
// Pure data — no Bevy, no Grid, no Agent.
// Dir and Action are the complete vocabulary an agent can express.

// ── Direction — 8-way grid movement ──────────────────────────────────────────
//
//  NW  N  NE
//   W  @  E
//  SW  S  SE
//
// delta() returns (dx, dy) where +x = right, +y = up.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Dir {
    N,
    S,
    E,
    W,
    NE,
    NW,
    SE,
    SW,
}

impl Dir {
    pub fn all() -> &'static [Dir; 8] {
        &[
            Dir::N,
            Dir::S,
            Dir::E,
            Dir::W,
            Dir::NE,
            Dir::NW,
            Dir::SE,
            Dir::SW,
        ]
    }

    /// (dx, dy) — one grid step in this direction.
    /// Diagonal moves still advance exactly one cell on each axis.
    pub fn delta(self) -> (i32, i32) {
        match self {
            Dir::N  => ( 0,  1),
            Dir::S  => ( 0, -1),
            Dir::E  => ( 1,  0),
            Dir::W  => (-1,  0),
            Dir::NE => ( 1,  1),
            Dir::NW => (-1,  1),
            Dir::SE => ( 1, -1),
            Dir::SW => (-1, -1),
        }
    }

    pub fn is_diagonal(self) -> bool {
        matches!(self, Dir::NE | Dir::NW | Dir::SE | Dir::SW)
    }

    /// Opposite direction — useful for backtracking.
    pub fn opposite(self) -> Dir {
        match self {
            Dir::N  => Dir::S,
            Dir::S  => Dir::N,
            Dir::E  => Dir::W,
            Dir::W  => Dir::E,
            Dir::NE => Dir::SW,
            Dir::NW => Dir::SE,
            Dir::SE => Dir::NW,
            Dir::SW => Dir::NE,
        }
    }
}

// ── Action — complete vocabulary of what an agent can do in one tick ──────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    /// Move one cell in the given direction.
    /// Diagonal = one step on both axes simultaneously.
    /// Invalid moves (out of bounds, obstacle) are silently ignored by the sim.
    Move(Dir),

    /// Pick up gold from the current cell. No-op if cell has no gold.
    Pickup,

    /// Drop all carried gold. No-op if not on Base tile.
    Drop,

    /// Do nothing this tick.
    Wait,
}