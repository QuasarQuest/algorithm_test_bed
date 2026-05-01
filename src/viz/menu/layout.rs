// src/viz/menu/layout.rs

use bevy::prelude::*;
use crate::viz::core_ui::*;
use crate::viz::hud::components::TickLabelMarker;
use super::components::*;

const TOOLBAR_H: f32 = 48.0;
const DRAWER_W:  f32 = 240.0;

pub fn spawn_menu(
    mut commands: Commands,
    theme: Res<ThemeMode>,
    menu:  Res<MenuState>,
) {
    build_toolbar(&mut commands, *theme);
    if menu.is_open {
        build_drawer(&mut commands, *theme);
    }
}

pub fn build_toolbar(commands: &mut Commands, mode: ThemeMode) {
    let bg     = ThemeColor::Background.resolve(mode);
    let border = ThemeColor::Border.resolve(mode);

    commands.spawn((
        UiRoot,
        Node {
            width:           Val::Percent(100.0),
            height:          Val::Px(TOOLBAR_H),
            position_type:   PositionType::Absolute,
            top:             Val::Px(0.0),
            left:            Val::Px(0.0),
            flex_direction:  FlexDirection::Row,
            align_items:     AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            padding:         UiRect::axes(Val::Px(8.0), Val::Px(0.0)),
            border:          UiRect::bottom(Val::Px(1.0)),
            ..default()
        },
        BackgroundColor(bg),
        BorderColor::all(border),
        ZIndex(100),
    )).with_children(|root| {

        // Left: hamburger
        spawn_icon_button(root, mode, "=", HamburgerButton);

        // Center: speed controls
        spawn_button_group(root, mode, |bg| {
            spawn_icon_button(bg, mode, "<", SpeedDecreaseButton);
            bg.spawn((
                Button,
                SpeedResetButton,
                Node {
                    padding:         UiRect::axes(Val::Px(14.0), Val::Px(6.0)),
                    justify_content: JustifyContent::Center,
                    align_items:     AlignItems::Center,
                    border_radius:   BorderRadius::all(Val::Px(4.0)),
                    min_width:       Val::Px(52.0),
                    ..default()
                },
                BackgroundColor(ThemeColor::ButtonIdle.resolve(mode)),
            )).with_children(|btn| {
                btn.spawn((
                    Text::new("10x"),
                    TextFont  { font_size: SIZE_MD, ..default() },
                    TextColor(ThemeColor::TextPrimary.resolve(mode)),
                    CurrentSpeedLabel,
                ));
            });
            spawn_icon_button(bg, mode, ">", SpeedIncreaseButton);
        });

        // Right: pause + tick
        root.spawn(Node {
            flex_direction: FlexDirection::Row,
            align_items:    AlignItems::Center,
            column_gap:     Val::Px(12.0),
            ..default()
        }).with_children(|right| {
            spawn_labeled_button(
                right, mode,
                "Running",
                ThemeColor::Success,
                ThemeColor::SuccessText,
                PauseButtonMarker,
            );

            right.spawn(Node {
                flex_direction: FlexDirection::Row,
                align_items:    AlignItems::Center,
                column_gap:     Val::Px(5.0),
                ..default()
            }).with_children(|tick| {
                spawn_label(tick, "TICK", ThemeColor::TextDim.resolve(mode), SIZE_SM);
                spawn_marked_label(tick, "0", ThemeColor::TextPrimary.resolve(mode), SIZE_LG, TickLabelMarker);
            });
        });
    });
}

pub fn build_drawer(commands: &mut Commands, mode: ThemeMode) {
    let bg     = ThemeColor::Background.resolve(mode);
    let border = ThemeColor::Border.resolve(mode);
    let dim    = ThemeColor::TextDim.resolve(mode);
    let text   = ThemeColor::TextPrimary.resolve(mode);

    // Invisible overlay — closes drawer on click
    commands.spawn((
        UiRoot,
        DrawerOverlay,
        Interaction::default(), // <--- CRITICAL FIX: No `Button` component, so it doesn't turn gray!
        Node {
            position_type: PositionType::Absolute,
            top:           Val::Px(TOOLBAR_H),
            left:          Val::Px(0.0),
            width:         Val::Percent(100.0),
            height:        Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.4)), // <--- Dimming the background to emulate focus
        ZIndex(98),
    ));

    // Drawer panel
    commands.spawn((
        UiRoot,
        DrawerPanel,
        Node {
            position_type:  PositionType::Absolute,
            top:            Val::Px(TOOLBAR_H),
            left:           Val::Px(0.0),
            width:          Val::Px(DRAWER_W),
            height:         Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            padding:        UiRect::all(Val::Px(12.0)),
            row_gap:        Val::Px(4.0),
            border:         UiRect::right(Val::Px(1.0)),
            ..default()
        },
        BackgroundColor(bg.with_alpha(0.85)), // <--- SEMI-TRANSPARENT MENU
        BorderColor::all(border),
        ZIndex(99),
    )).with_children(|drawer| {
        // Header row
        drawer.spawn(Node {
            flex_direction:  FlexDirection::Row,
            align_items:     AlignItems::Center,
            justify_content: JustifyContent::SpaceBetween,
            padding:         UiRect::bottom(Val::Px(12.0)),
            border:          UiRect::bottom(Val::Px(1.0)),
            margin:          UiRect::bottom(Val::Px(8.0)),
            ..default()
        }).with_children(|header| {
            header.spawn((
                Text::new("Menu"),
                TextFont  { font_size: SIZE_LG, ..default() },
                TextColor(text),
            ));
            spawn_icon_button(header, mode, "X", HamburgerButton); // Clicking this or the overlay closes it
        });

        drawer_section(drawer, "Simulation", dim);
        drawer_section(drawer, "Visualization", dim);
        drawer_section(drawer, "Agents", dim);
        drawer_section(drawer, "Theme", dim);
    });
}

fn drawer_section(parent: &mut ChildSpawnerCommands, label: &str, color: Color) {
    parent.spawn(Node {
        flex_direction: FlexDirection::Column,
        padding:        UiRect::vertical(Val::Px(6.0)),
        ..default()
    }).with_children(|s| {
        s.spawn((
            Text::new(label),
            TextFont  { font_size: SIZE_SM, ..default() },
            TextColor(color),
        ));
    });
}