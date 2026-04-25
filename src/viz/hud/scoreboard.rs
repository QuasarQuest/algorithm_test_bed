// src/viz/hud/scoreboard.rs
//
// Scoreboard panel — bottom-right corner, always visible.
// Reads agent data (AgentLabel, GoldCarried, Score) from ECS.
// Renders a compact table: AGENT | ◆ CARRY | ★ SCORE

use bevy::prelude::*;
use crate::agent::components::{AgentLabel, GoldCarried, Score};

// ── Markers ───────────────────────────────────────────────────────────────────

#[derive(Component)] pub struct ScoreboardPanel;
#[derive(Component)] pub struct ScoreboardContent;

// ── Colours ───────────────────────────────────────────────────────────────────

const PANEL_BG:     Color = Color::srgba(0.06, 0.07, 0.10, 0.96);
const ROW_ODD:      Color = Color::srgba(1.0, 1.0, 1.0, 0.03);
const ACCENT_GOLD:  Color = Color::srgb(0.95, 0.78, 0.20);
const ACCENT_GREEN: Color = Color::srgb(0.40, 0.90, 0.55);
const TEXT_HEAD:    Color = Color::srgb(0.55, 0.55, 0.62);
const TEXT_BODY:    Color = Color::srgb(0.92, 0.92, 0.95);
const BORDER_COLOR: Color = Color::srgba(1.0, 1.0, 1.0, 0.07);

// ── Spawn ─────────────────────────────────────────────────────────────────────

pub fn spawn_scoreboard(mut commands: Commands) {
    commands.spawn((
        ScoreboardPanel,
        Node {
            position_type:  PositionType::Absolute,
            bottom:         Val::Px(12.0),
            right:          Val::Px(12.0),
            flex_direction: FlexDirection::Column,
            min_width:      Val::Px(240.0),
            border:         UiRect::all(Val::Px(1.0)),
            border_radius:  BorderRadius::all(Val::Px(8.0)),
            padding:        UiRect::all(Val::Px(1.0)),
            ..default()
        },
        BackgroundColor(PANEL_BG),
        BorderColor::all(BORDER_COLOR),
    )).with_children(|panel| {
        // Header
        panel.spawn(Node {
            flex_direction:  FlexDirection::Row,
            padding:         UiRect::new(Val::Px(12.0), Val::Px(12.0), Val::Px(8.0), Val::Px(6.0)),
            column_gap:      Val::Px(8.0),
            ..default()
        }).with_children(|h| {
            cell(h, "AGENT",       120.0, TEXT_HEAD,   10.0);
            cell(h, "Gold",        32.0, ACCENT_GOLD, 11.0);
            cell(h, "SCORE",       60.0, TEXT_HEAD,   10.0);
        });

        // Content — rebuilt each frame by update_scoreboard
        panel.spawn((
            ScoreboardContent,
            Node { flex_direction: FlexDirection::Column, ..default() },
        ));
    });
}

// ── Update ────────────────────────────────────────────────────────────────────

pub fn update_scoreboard(
    agents:       Query<(&AgentLabel, &GoldCarried, &Score)>,
    content_q:    Query<Entity, With<ScoreboardContent>>,
    mut commands: Commands,
) {
    let Ok(content) = content_q.single() else { return };

    commands.entity(content).despawn_related::<Children>();

    let mut rows: Vec<_> = agents.iter().collect();
    rows.sort_by_key(|(label, _, _)| label.0.clone());

    commands.entity(content).with_children(|c| {
        for (i, (label, gold, score)) in rows.iter().enumerate() {
            let bg = if i % 2 == 0 { Color::NONE } else { ROW_ODD };
            c.spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    padding:        UiRect::axes(Val::Px(12.0), Val::Px(5.0)),
                    column_gap:     Val::Px(8.0),
                    border_radius:  BorderRadius::all(Val::Px(4.0)),
                    ..default()
                },
                BackgroundColor(bg),
            )).with_children(|row| {
                cell(row, &label.0,            120.0, TEXT_BODY,    12.0);
                cell(row, &gold.0.to_string(),   32.0, ACCENT_GOLD,  12.0);
                cell(row, &score.0.to_string(),  60.0, ACCENT_GREEN, 12.0);
            });
        }
    });
}

// ── Helper ────────────────────────────────────────────────────────────────────

fn cell(parent: &mut ChildSpawnerCommands, text: &str, width: f32, color: Color, size: f32) {
    parent.spawn((
        Text::new(text),
        TextFont  { font_size: size, ..default() },
        TextColor(color),
        Node { width: Val::Px(width), ..default() },
    ));
}
