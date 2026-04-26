// src/viz/hud/scoreboard.rs
//
// Scoreboard panel — bottom-right corner, always visible.
// Reads agent data (AgentLabel, GoldCarried, Score) from ECS.
// Renders a compact table: AGENT | ◆ CARRY | ★ SCORE

use bevy::prelude::*;
use crate::agent::components::{AgentLabel, GoldCarried, Score};
use crate::viz::core_ui::*; // Import the Lego bricks and theme!

// ── Markers ───────────────────────────────────────────────────────────────────

#[derive(Component)] pub struct ScoreboardPanel;
#[derive(Component)] pub struct ScoreboardContent;

// ── Local Theme ───────────────────────────────────────────────────────────────

const ROW_ODD:     Color = Color::srgba(1.0, 1.0, 1.0, 0.03);
const ACCENT_GOLD: Color = Color::srgb(0.95, 0.78, 0.20); // Unique to scoreboard

// ── The React Builder ─────────────────────────────────────────────────────────

// 1. The Bevy System (Runs at Startup)
pub fn spawn_scoreboard(mut commands: Commands, theme: Res<ThemeMode>) {
    build_scoreboard(&mut commands, *theme);
}

// 2. The Logic (Can be called anytime to rebuild the shell)
pub fn build_scoreboard(commands: &mut Commands, mode: ThemeMode) {
    commands.spawn((
        UiRoot, // <-- The React Marker so this gets destroyed on theme change!
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
        BackgroundColor(ThemeColor::Background.resolve(mode)), // <-- Dynamic!
        BorderColor::all(ThemeColor::Border.resolve(mode)),    // <-- Dynamic!
    )).with_children(|panel| {
        // Header
        panel.spawn(Node {
            flex_direction:  FlexDirection::Row,
            padding:         UiRect::new(Val::Px(12.0), Val::Px(12.0), Val::Px(8.0), Val::Px(6.0)),
            column_gap:      Val::Px(8.0),
            ..default()
        }).with_children(|h| {
            cell(h, "AGENT", 120.0, ThemeColor::TextDim.resolve(mode), SIZE_SM);
            cell(h, "Gold",   32.0, ACCENT_GOLD, SIZE_SM);
            cell(h, "SCORE",  60.0, ThemeColor::TextDim.resolve(mode), SIZE_SM);
        });

        // Content Shell — rebuilt each frame by update_scoreboard
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
    theme:        Res<ThemeMode>, // <-- We inject the theme here too!
    mut commands: Commands,
) {
    let Ok(content) = content_q.single() else { return };

    // Clear old rows
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
                // Resolve text colors using the current theme mode
                cell(row, &label.0,            120.0, ThemeColor::TextPrimary.resolve(*theme), SIZE_MD);
                cell(row, &gold.0.to_string(),   32.0, ACCENT_GOLD,  SIZE_MD);
                // I'm assuming you want the score to be green like before (SuccessText)
                cell(row, &score.0.to_string(),  60.0, ThemeColor::SuccessText.resolve(*theme), SIZE_MD);
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