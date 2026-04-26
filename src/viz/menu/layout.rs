// src/viz/menu/layout.rs

use bevy::prelude::*;
use crate::viz::core_ui::*;
use super::components::*;

// 1. The Bevy System (Runs at Startup)
pub fn spawn_menu(mut commands: Commands, theme: Res<ThemeMode>) {
    build_menu(&mut commands, *theme);
}

// 2. The Logic (Can be called anytime to rebuild the shell)
pub fn build_menu(commands: &mut Commands, mode: ThemeMode) {
    // Top-Left Panel
    spawn_floating_panel(commands, mode, Val::Px(12.0), Val::Auto, Val::Auto, Val::Px(12.0), |p| {
        spawn_labeled_button(p, mode, "Viz: ON", ThemeColor::ButtonIdle, ThemeColor::SuccessText, VizToggleButton);
        spawn_labeled_button(p, mode, "Theme", ThemeColor::ButtonIdle, ThemeColor::TextPrimary, ThemeToggleButton);
        spawn_label(p, "Algorithm Test Bed", ThemeColor::TextDim.resolve(mode), SIZE_LG);
    });

    // Top-Center Panel
    spawn_floating_panel(commands, mode, Val::Px(12.0), Val::Auto, Val::Auto, Val::Percent(40.0), |p| {
        spawn_button_group(p, mode, |bg| {
            spawn_icon_button(bg, mode, "-", SpeedDecreaseButton);
            spawn_speed_label(bg, mode);
            spawn_icon_button(bg, mode, "+", SpeedIncreaseButton);
        });

        spawn_labeled_button(p, mode, "Running", ThemeColor::Success, ThemeColor::SuccessText, PauseButtonMarker);
    });
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