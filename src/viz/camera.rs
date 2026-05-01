// src/viz/camera.rs

use bevy::prelude::*;
use bevy::input::mouse::{AccumulatedMouseScroll, MouseScrollUnit};
use crate::world::Grid;
use crate::config;

#[derive(Component)]
pub struct MainCamera;

const ZOOM_MIN:   f32 = 0.1;
const ZOOM_MAX:   f32 = 10.0;
const ZOOM_SPEED: f32 = 0.08; // smaller = finer steps

#[derive(Resource, Default)]
pub struct PanState {
    dragging: bool,
    last_pos: Vec2,
}

pub fn init_pan_state(mut commands: Commands) {
    commands.insert_resource(PanState::default());
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2d::default(), MainCamera));
}

pub fn fit_camera_to_grid(
    grid:       Res<Grid>,
    windows:    Query<&Window>,
    mut cam:    Single<&mut Transform, With<MainCamera>>,
    mut proj:   Single<&mut Projection, With<MainCamera>>,
) {
    let Ok(window) = windows.single() else { return };
    let step      = config::TILE_SIZE + config::TILE_GAP;
    let scale_x   = grid.width  as f32 * step / window.width()  * 1.10;
    let scale_y   = grid.height as f32 * step / window.height() * 1.10;

    if let Projection::Orthographic(ref mut ortho) = **proj {
        ortho.scale = scale_x.max(scale_y).clamp(ZOOM_MIN, ZOOM_MAX);
    }
    cam.translation = Vec3::ZERO;
}

pub fn camera_controls(
    windows:  Query<&Window>,
    mut cam:  Single<(&mut Transform, &mut Projection), With<MainCamera>>,
    scroll:   Res<AccumulatedMouseScroll>,
    mouse:    Res<ButtonInput<MouseButton>>,
    mut pan:  ResMut<PanState>,
) {
    let Ok(window) = windows.single() else { return };
    let (ref mut tf, ref mut projection) = *cam;
    let Projection::Orthographic(ref mut ortho) = **projection else { return };

    // ── Zoom ──────────────────────────────────────────────────────────────────
    // Clamp the raw delta so one frame can never jump more than ~1 step
    let raw = match scroll.unit {
        MouseScrollUnit::Line  => scroll.delta.y,
        MouseScrollUnit::Pixel => scroll.delta.y / 20.0, // normalise pixels
    };
    let clamped = raw.clamp(-1.0, 1.0); // max one "tick" per frame

    if clamped.abs() > f32::EPSILON {
        let old_scale = ortho.scale;
        // Multiplicative zoom: each tick multiplies scale by a fixed factor
        let factor    = if clamped > 0.0 {
            1.0 - ZOOM_SPEED   // scroll up   → zoom in  → smaller scale
        } else {
            1.0 + ZOOM_SPEED   // scroll down → zoom out → larger scale
        };
        let new_scale = (old_scale * factor).clamp(ZOOM_MIN, ZOOM_MAX);

        // Cursor-centred: keep the world point under cursor fixed
        if let Some(cursor) = window.cursor_position() {
            let win      = Vec2::new(window.width(), window.height());
            let ndc      = (cursor / win - 0.5) * 2.0;
            let world    = tf.translation.truncate()
                + Vec2::new(ndc.x * win.x / 2.0 * old_scale,
                            -ndc.y * win.y / 2.0 * old_scale);
            let new_cam  = world
                - Vec2::new(ndc.x * win.x / 2.0 * new_scale,
                            -ndc.y * win.y / 2.0 * new_scale);
            tf.translation.x = new_cam.x;
            tf.translation.y = new_cam.y;
        }
        ortho.scale = new_scale;
    }

    // ── Pan (middle mouse) ────────────────────────────────────────────────────
    if mouse.just_pressed(MouseButton::Middle) {
        pan.dragging = true;
        pan.last_pos = window.cursor_position().unwrap_or(Vec2::ZERO);
    }
    if mouse.just_released(MouseButton::Middle) {
        pan.dragging = false;
    }
    if pan.dragging {
        if let Some(cursor) = window.cursor_position() {
            let delta = cursor - pan.last_pos;
            tf.translation.x -= delta.x * ortho.scale;
            tf.translation.y += delta.y * ortho.scale;
            pan.last_pos = cursor;
        }
    }
}