// src/viz/hud/layout.rs

use bevy::prelude::*;
use bevy::ecs::hierarchy::ChildSpawnerCommands;
use super::components::*;

pub const BG:           Color = Color::srgba(0.06, 0.06, 0.08, 0.97);
pub const BTN_IDLE:     Color = Color::srgb(0.12, 0.12, 0.14);
pub const BTN_HOVER:    Color = Color::srgb(0.20, 0.20, 0.22);
pub const BTN_ACTIVE:   Color = Color::srgb(0.14, 0.45, 0.90);
pub const BTN_PAUSED:   Color = Color::srgb(0.85, 0.25, 0.20);
pub const TEXT_PRIMARY: Color = Color::srgb(0.95, 0.95, 0.98);
pub const TEXT_DIM:     Color = Color::srgb(0.55, 0.55, 0.60);
pub const TEXT_ACCENT:  Color = Color::srgb(0.40, 0.78, 0.55);
pub const BORDER:       Color = Color::srgba(1.0, 1.0, 1.0, 0.08);

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
    )).with_children(|parent| {
        build_left_section(parent);
        build_center_section(parent);
        build_right_section(parent);
    });
}

fn build_left_section(parent: &mut ChildSpawnerCommands<'_>) {
    parent.spawn((
        Text::new("algo_sandbox"),
        TextFont  { font_size: 15.0, ..default() },
        TextColor(TEXT_ACCENT),
    ));
}

fn build_center_section(parent: &mut ChildSpawnerCommands<'_>) {
    parent.spawn(Node {
        flex_direction: FlexDirection::Row,
        align_items:    AlignItems::Center,
        column_gap:     Val::Px(16.0),
        ..default()
    }).with_children(|center| {
        build_speed_button_group(center);
        build_pause_button(center);
    });
}

fn build_speed_button_group(parent: &mut ChildSpawnerCommands<'_>) {
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
        // Decrease Speed (-)
        bg.spawn((
            Button,
            SpeedDecreaseButton,
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
                Text::new("-"),
                TextFont  { font_size: 16.0, ..default() },
                TextColor(TEXT_PRIMARY),
            ));
        });

        // Reset Speed / Display (10x)
        bg.spawn((
            Button,
            SpeedResetButton,
            Node {
                padding:         UiRect::axes(Val::Px(16.0), Val::Px(6.0)),
                justify_content: JustifyContent::Center,
                align_items:     AlignItems::Center,
                border_radius:   BorderRadius::all(Val::Px(4.0)),
                min_width:       Val::Px(48.0), // Prevents button from resizing when text changes
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

        // Increase Speed (+)
        bg.spawn((
            Button,
            SpeedIncreaseButton,
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
                Text::new("+"),
                TextFont  { font_size: 16.0, ..default() },
                TextColor(TEXT_PRIMARY),
            ));
        });
    });
}

fn build_pause_button(parent: &mut ChildSpawnerCommands<'_>) {
    parent.spawn((
        Button,
        PauseButtonMarker,
        Node {
            padding:         UiRect::axes(Val::Px(16.0), Val::Px(8.0)),
            border:          UiRect::all(Val::Px(1.0)),
            align_items:     AlignItems::Center,
            border_radius:   BorderRadius::all(Val::Px(6.0)),
            ..default()
        },
        BackgroundColor(BTN_IDLE),
        BorderColor::all(BORDER),
    )).with_children(|btn| {
        btn.spawn((
            Text::new("Running"),
            TextFont  { font_size: 13.0, ..default() },
            TextColor(TEXT_PRIMARY),
            PauseLabelMarker,
        ));
    });
}

fn build_right_section(parent: &mut ChildSpawnerCommands<'_>) {
    parent.spawn(Node {
        flex_direction: FlexDirection::Row,
        align_items:    AlignItems::Center,
        column_gap:     Val::Px(8.0),
        ..default()
    }).with_children(|right| {
        right.spawn((
            Text::new("TICK"),
            TextFont  { font_size: 11.0, ..default() },
            TextColor(TEXT_DIM),
        ));
        right.spawn((
            Text::new("0"),
            TextFont  { font_size: 15.0, ..default() },
            TextColor(TEXT_PRIMARY),
            TickLabelMarker,
        ));
    });
}