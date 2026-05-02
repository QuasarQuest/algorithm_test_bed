// src/agent/combat.rs
//
// Combat resolution — runs after movement, before the next tick.
// Agents that attack deal damage to the target in the adjacent cell.
// Dead agents (health == 0) are despawned and their gold dropped.
//
// Currently: Attack action not yet in action.rs vocabulary.
// This module is the home for that logic when added.
// Stub ready — wire into systems.rs and agent.rs when Action::Attack(Dir) is added.

use bevy::prelude::*;
use crate::world::{Grid, tile::Tile};
use super::action::Action;
use super::components::{GridPos, GoldCarried, Health};
use super::systems::PendingAction;

// ── Constants ─────────────────────────────────────────────────────────────────

pub const ATTACK_DAMAGE: u32 = 10;

// ── Dead marker — used to flag entities for despawn this tick ─────────────────

#[derive(Component)]
pub struct Dead;

// ── Combat phase ──────────────────────────────────────────────────────────────
// Call this AFTER apply_actions movement phase, BEFORE next tick_agents.

pub fn resolve_combat(
    mut commands:  Commands,
    mut grid:      ResMut<Grid>,
    attackers:     Query<(Entity, &GridPos, &PendingAction)>,
    mut defenders: Query<(Entity, &GridPos, &mut Health, &mut GoldCarried), Without<Dead>>,
) {
    // Build position → defender entity map
    let defender_map: std::collections::HashMap<GridPos, Entity> = defenders
        .iter()
        .map(|(e, pos, _, _)| (*pos, e))
        .collect();

    for (_attacker, pos, pending) in attackers.iter() {
        let Some(Action::Attack(dir)) = pending.0 else { continue };

        let (dx, dy) = dir.delta();
        let target_pos = pos.apply_delta(dx, dy);

        if let Some(&defender_entity) = defender_map.get(&target_pos) {
            if let Ok((entity, def_pos, mut health, mut gold)) =
                defenders.get_mut(defender_entity)
            {
                if health.0 <= ATTACK_DAMAGE {
                    // Kill — drop gold onto grid if possible, mark for despawn
                    if gold.0 > 0 && grid.get(def_pos.x, def_pos.y) == Some(Tile::Free) {
                        grid.set(def_pos.x, def_pos.y, Tile::Gold);
                        gold.0 = 0;
                    }
                    commands.entity(entity).insert(Dead);
                    info!("Agent {:?} killed at {:?}", entity, def_pos);
                } else {
                    health.0 -= ATTACK_DAMAGE;
                    info!("Agent {:?} hit for {ATTACK_DAMAGE} — {} hp remaining",
                        entity, health.0);
                }
            }
        }
    }
}

// ── Despawn dead agents ───────────────────────────────────────────────────────

pub fn despawn_dead(
    mut commands: Commands,
    query: Query<Entity, With<Dead>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}