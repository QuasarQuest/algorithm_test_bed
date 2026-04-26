// src/viz/menu/layout.rs

use bevy::prelude::*;
use crate::viz::core_ui::*;
use super::components::*;

// 1. The Bevy System (Runs at Startup)
pub fn spawn_menu(mut commands: Commands, theme: Res<ThemeMode>, menu: Res<MenuState>) {
    build_menu(&mut commands, *theme, menu.is_open);
}
// 2. The Logic (Can be called anytime to rebuild the shell)

pub fn build_menu(commands: &mut Commands, mode: ThemeMode, is_open: bool) {
    // 1. Top-Left Panel (Just the Hamburger)
    spawn_floating_panel(commands, mode, Val::Px(12.0), Val::Auto, Val::Auto, Val::Px(12.0), |p| {
        spawn_icon_button(p, mode, "≡", HamburgerButton);
    });

    // 2. Dropdown Panel (Only appears if is_open is true)
    if is_open {
        // Spawned just below the hamburger button
        spawn_floating_panel(commands, mode, Val::Px(64.0), Val::Auto, Val::Auto, Val::Px(12.0), |p| {
            // Wrap the buttons in a column layout
            p.spawn(Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                ..default()
            }).with_children(|col| {
                spawn_labeled_button(col, mode, "▶  Running", ThemeColor::Success, ThemeColor::SuccessText, PauseButtonMarker);
                spawn_labeled_button(col, mode, "👁  Viz: ON", ThemeColor::ButtonIdle, ThemeColor::SuccessText, VizToggleButton);
                spawn_labeled_button(col, mode, "🌗 Theme", ThemeColor::ButtonIdle, ThemeColor::TextPrimary, ThemeToggleButton);
            });
        });
    }
}

// Custom builder for the speed label
fn spawn_speed_label(parent: &mut ChildSpawnerCommands, mode: ThemeMode) {
    parent.spawn((
        Button,
        SpeedResetButton,
        Node {
            padding: UiRect::axes(Val::Px(16.0), Val::Px(6.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border_radius: BorderRadius::all(Val::Px(4.0)),
            min_width: Val::Px(52.0),
            ..default()
        },
        BackgroundColor(ThemeColor::ButtonIdle.resolve(mode)),
        BorderColor::all(ThemeColor::Border.resolve(mode)),
    )).with_children(|btn| {
        btn.spawn((
            Text::new("10x"),
            TextFont { font_size: SIZE_MD, ..default() },
            TextColor(ThemeColor::TextPrimary.resolve(mode)),
            CurrentSpeedLabel,
        ));
    });
}