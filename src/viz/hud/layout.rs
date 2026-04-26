// src/viz/hud/layout.rs

use bevy::prelude::*;
use crate::viz::core_ui::*;
use super::components::*;
use crate::viz::menu::components::{SpeedDecreaseButton, SpeedIncreaseButton, SpeedResetButton, CurrentSpeedLabel};

pub fn spawn_hud(mut commands: Commands, theme: Res<ThemeMode>) {
    build_hud(&mut commands, *theme);
}

pub fn build_hud(commands: &mut Commands, mode: ThemeMode) {
    // Top-Right Panel: Now contains Speed AND Ticks
    spawn_floating_panel(commands, mode, Val::Px(12.0), Val::Px(12.0), Val::Auto, Val::Auto, |p| {

        // Moved Speed Controls
        spawn_button_group(p, mode, |bg| {
            spawn_icon_button(bg, mode, "−", SpeedDecreaseButton);
            spawn_speed_label(bg, mode);
            spawn_icon_button(bg, mode, "+", SpeedIncreaseButton);
        });

        // Tick Counter
        spawn_label(p, "TICK", ThemeColor::TextDim.resolve(mode), SIZE_SM);
        spawn_marked_label(p, "0", ThemeColor::TextPrimary.resolve(mode), SIZE_LG, TickLabelMarker);
    });
}

// Custom builder for the speed label (Moved from menu layout)
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