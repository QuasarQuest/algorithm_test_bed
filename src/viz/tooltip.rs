// src/viz/tooltip.rs

use bevy::prelude::*;
use crate::world::coords::GridPos;
use crate::agent::components::{AgentLabel, GoldCarried, Score};
use super::grid_offset::GridOffset;
use super::camera::MainCamera;

#[derive(Component)] pub struct TooltipPanel;
#[derive(Component)] pub struct TooltipName;
#[derive(Component)] pub struct TooltipCarry;
#[derive(Component)] pub struct TooltipScore;
#[derive(Component)] pub struct TooltipPos;

const PANEL_BG:     Color = Color::srgba(0.06, 0.07, 0.10, 0.96);
const ACCENT_GREEN: Color = Color::srgb(0.40, 0.90, 0.55);
const TEXT_HEAD:    Color = Color::srgb(0.55, 0.55, 0.62);
const TEXT_BODY:    Color = Color::srgb(0.92, 0.92, 0.95);
const BORDER_COLOR: Color = Color::srgba(1.0, 1.0, 1.0, 0.07);

pub fn spawn_tooltip(mut commands: Commands) {
    commands.spawn((
        TooltipPanel,
        Node {
            position_type:  PositionType::Absolute,
            display:        Display::None,
            flex_direction: FlexDirection::Column,
            padding:        UiRect::all(Val::Px(12.0)),
            row_gap:        Val::Px(4.0),
            border:         UiRect::all(Val::Px(1.0)),
            border_radius:  BorderRadius::all(Val::Px(8.0)),
            min_width:      Val::Px(180.0),
            ..default()
        },
        BackgroundColor(PANEL_BG),
        BorderColor::all(BORDER_COLOR),
        ZIndex(100),
    )).with_children(|p| {
        p.spawn((
            Text::new("—"),
            TextFont  { font_size: 13.0, ..default() },
            TextColor(ACCENT_GREEN),
            TooltipName,
        ));
        tooltip_row(p, "Gold:",     "0",      TooltipCarry);
        tooltip_row(p, "Score:",    "0",      TooltipScore);
        tooltip_row(p, "Position:", "(0, 0)", TooltipPos);
    });
}

fn tooltip_row(parent: &mut ChildSpawnerCommands, label: &str, value: &str, marker: impl Bundle) {
    parent.spawn(Node {
        flex_direction: FlexDirection::Row,
        column_gap:     Val::Px(6.0),
        ..default()
    }).with_children(|r| {
        r.spawn((
            Text::new(label),
            TextFont  { font_size: 11.0, ..default() },
            TextColor(TEXT_HEAD),
        ));
        r.spawn((
            Text::new(value),
            TextFont  { font_size: 11.0, ..default() },
            TextColor(TEXT_BODY),
            marker,
        ));
    });
}

pub fn update_tooltip(
    windows:     Query<&Window>,
    agents:      Query<(&AgentLabel, &GoldCarried, &Score, &GridPos, &Transform)>,
    offset:      Res<GridOffset>,
    cam_q:       Query<(&Transform, &Projection), With<MainCamera>>,
    mut panel_q: Query<(&mut Node, &mut Visibility), With<TooltipPanel>>,
    mut name_q:  Query<&mut Text, (With<TooltipName>,  Without<TooltipCarry>, Without<TooltipScore>, Without<TooltipPos>)>,
    mut carry_q: Query<&mut Text, (With<TooltipCarry>, Without<TooltipName>,  Without<TooltipScore>, Without<TooltipPos>)>,
    mut score_q: Query<&mut Text, (With<TooltipScore>, Without<TooltipName>,  Without<TooltipCarry>, Without<TooltipPos>)>,
    mut pos_q:   Query<&mut Text, (With<TooltipPos>,   Without<TooltipName>,  Without<TooltipCarry>, Without<TooltipScore>)>,
) {
    let Ok(window)              = windows.single()     else { return };
    let Ok((mut node, mut vis)) = panel_q.single_mut() else { return };

    let Some(cursor_screen) = window.cursor_position() else {
        hide(&mut node, &mut vis);
        return;
    };

    // Convert screen cursor → world space accounting for camera pan and zoom
    let cursor_world = {
        let Ok((cam_tf, projection)) = cam_q.single() else {
            hide(&mut node, &mut vis);
            return;
        };
        let Projection::Orthographic(ref ortho) = *projection else {
            hide(&mut node, &mut vis);
            return;
        };
        let win      = Vec2::new(window.width(), window.height());
        let ndc      = (cursor_screen / win - 0.5) * 2.0;
        let scale    = ortho.scale;
        cam_tf.translation.truncate()
            + Vec2::new(ndc.x * win.x / 2.0 * scale,
                        -ndc.y * win.y / 2.0 * scale)
    };

    let hover_r = offset.step * 0.6;

    let hovered = agents.iter().find(|(_, _, _, _, t)| {
        // Agent world position is stored directly in Transform
        let agent_world = t.translation.truncate();
        cursor_world.distance(agent_world) < hover_r
    });

    if let Some((label, gold, score, pos, _)) = hovered {
        node.display = Display::Flex;
        *vis         = Visibility::Visible;
        // Tooltip position stays in screen space (UI coordinates)
        node.left    = Val::Px((cursor_screen.x + 14.0).min(window.width()  - 200.0));
        node.top     = Val::Px((cursor_screen.y - 10.0).max(0.0));

        if let Ok(mut t) = name_q.single_mut()  { *t = Text::new(&label.0); }
        if let Ok(mut t) = carry_q.single_mut() { *t = Text::new(gold.0.to_string()); }
        if let Ok(mut t) = score_q.single_mut() { *t = Text::new(score.0.to_string()); }
        if let Ok(mut t) = pos_q.single_mut()   { *t = Text::new(format!("({}, {})", pos.x, pos.y)); }
    } else {
        hide(&mut node, &mut vis);
    }
}

fn hide(node: &mut Node, vis: &mut Visibility) {
    node.display = Display::None;
    *vis         = Visibility::Hidden;
}