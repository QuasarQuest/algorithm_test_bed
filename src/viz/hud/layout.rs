// src/viz/hud/layout.rs

use bevy::prelude::*;
use bevy::ecs::hierarchy::ChildSpawnerCommands;
use super::components::*;

// ── Colours ───────────────────────────────────────────────────────────────────

pub const BG:           Color = Color::srgba(0.06, 0.06, 0.08, 0.97);
pub const BTN_IDLE:     Color = Color::srgb(0.12, 0.12, 0.14);
pub const BTN_HOVER:    Color = Color::srgb(0.20, 0.20, 0.22);
pub const BTN_RUNNING:  Color = Color::srgb(0.12, 0.42, 0.24); // green — sim running
pub const BTN_PAUSED:   Color = Color::srgb(0.85, 0.25, 0.20); // red   — sim paused
pub const TEXT_RUNNING: Color = Color::srgb(0.40, 0.90, 0.55); // green text
pub const TEXT_PAUSED:  Color = Color::srgb(1.00, 0.70, 0.60); // warm text on red
pub const TEXT_PRIMARY: Color = Color::srgb(0.95, 0.95, 0.98);
pub const TEXT_DIM:     Color = Color::srgb(0.55, 0.55, 0.60);
pub const TEXT_ACCENT:  Color = Color::srgb(0.40, 0.78, 0.55);
pub const BORDER:       Color = Color::srgba(1.0, 1.0, 1.0, 0.08);

// ── Root ─────────────────────────────────────────────────────────────────────

pub fn spawn_hud(mut commands: Commands) {
    commands.spawn((
        Node {
            width:           Val::Percent(100.0),
            height:          Val::Px(56.0),
            position_type:   PositionType::Absolute,
            top:             Val::Px(0.0),
            left:            Val::Px(0.0),
            flex_direction:  FlexDirection::Row,
            align_items:     AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            padding:         UiRect::axes(Val::Px(24.0), Val::Px(0.0)),
            ..default()
        },
        BackgroundColor(BG),
    )).with_children(|p| {
        build_left(p);
        build_center(p);
        build_right(p);
    });
}

// ── Sections ─────────────────────────────────────────────────────────────────

fn build_left(parent: &mut ChildSpawnerCommands<'_>) {
    parent.spawn((
        Text::new(crate::config::WINDOW_TITLE),
        TextFont  { font_size: 15.0, ..default() },
        TextColor(TEXT_ACCENT),
    ));
}

fn build_center(parent: &mut ChildSpawnerCommands<'_>) {
    parent.spawn(Node {
        flex_direction: FlexDirection::Row,
        align_items:    AlignItems::Center,
        column_gap:     Val::Px(16.0),
        ..default()
    }).with_children(|c| {
        build_speed_group(c);
        build_pause_btn(c);
    });
}

fn build_right(parent: &mut ChildSpawnerCommands<'_>) {
    parent.spawn(Node {
        flex_direction: FlexDirection::Row,
        align_items:    AlignItems::Center,
        column_gap:     Val::Px(8.0),
        ..default()
    }).with_children(|r| {
        r.spawn((
            Text::new("TICK"),
            TextFont  { font_size: 11.0, ..default() },
            TextColor(TEXT_DIM),
        ));
        r.spawn((
            Text::new("0"),
            TextFont  { font_size: 15.0, ..default() },
            TextColor(TEXT_PRIMARY),
            TickLabelMarker,
        ));
    });
}

// ── Speed group ───────────────────────────────────────────────────────────────

fn build_speed_group(parent: &mut ChildSpawnerCommands<'_>) {
    parent.spawn((
        Node {
            flex_direction: FlexDirection::Row,
            align_items:    AlignItems::Center,
            column_gap:     Val::Px(4.0),
            padding:        UiRect::all(Val::Px(4.0)),
            border_radius:  BorderRadius::all(Val::Px(8.0)),
            border:         UiRect::all(Val::Px(1.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.2)),
        BorderColor::all(BORDER),
    )).with_children(|bg| {
        spawn_icon_btn(bg, SpeedDecreaseButton, "−");
        spawn_speed_label_btn(bg);
        spawn_icon_btn(bg, SpeedIncreaseButton, "+");
    });
}

// ── Button builders ───────────────────────────────────────────────────────────

/// Small icon button — used for − and +
fn spawn_icon_btn(parent: &mut ChildSpawnerCommands<'_>, marker: impl Bundle, icon: &str) {
    parent.spawn((
        Button,
        marker,
        Node {
            padding:         UiRect::axes(Val::Px(12.0), Val::Px(6.0)),
            justify_content: JustifyContent::Center,
            align_items:     AlignItems::Center,
            border_radius:   BorderRadius::all(Val::Px(4.0)),
            ..default()
        },
        BackgroundColor(BTN_IDLE),
    )).with_children(|btn| {
        btn.spawn((
            Text::new(icon),
            TextFont  { font_size: 16.0, ..default() },
            TextColor(TEXT_PRIMARY),
        ));
    });
}

/// Speed label button — wider, holds CurrentSpeedLabel for reactive updates
fn spawn_speed_label_btn(parent: &mut ChildSpawnerCommands<'_>) {
    parent.spawn((
        Button,
        SpeedResetButton,
        Node {
            padding:         UiRect::axes(Val::Px(16.0), Val::Px(6.0)),
            justify_content: JustifyContent::Center,
            align_items:     AlignItems::Center,
            border_radius:   BorderRadius::all(Val::Px(4.0)),
            min_width:       Val::Px(52.0),
            ..default()
        },
        BackgroundColor(BTN_IDLE),
    )).with_children(|btn| {
        btn.spawn((
            Text::new("10x"),
            TextFont  { font_size: 13.0, ..default() },
            TextColor(TEXT_PRIMARY),
            CurrentSpeedLabel,
        ));
    });
}

/// Pause / Resume button — green when running, red when paused
fn build_pause_btn(parent: &mut ChildSpawnerCommands<'_>) {
    parent.spawn((
        Button,
        PauseButtonMarker,
        Node {
            padding:       UiRect::axes(Val::Px(18.0), Val::Px(8.0)),
            border:        UiRect::all(Val::Px(1.0)),
            align_items:   AlignItems::Center,
            border_radius: BorderRadius::all(Val::Px(6.0)),
            ..default()
        },
        BackgroundColor(BTN_RUNNING),
        BorderColor::all(BORDER),
    )).with_children(|btn| {
        btn.spawn((
            Text::new("▶  Running"),
            TextFont  { font_size: 13.0, ..default() },
            TextColor(TEXT_RUNNING),
            PauseLabelMarker,
        ));
    });
}