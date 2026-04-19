// src/viz/tile_renderer.rs

use bevy::prelude::*;
use crate::world::Grid;
use crate::config;

#[derive(Component)]
pub struct TileMarker {
    pub x: usize,
    pub y: usize,
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

pub fn spawn_tiles(mut commands: Commands, grid: Res<Grid>) {
    let step     = config::TILE_SIZE + config::TILE_GAP;
    let offset_x = -(grid.width  as f32 * step) / 2.0 + step / 2.0;
    let offset_y = -(grid.height as f32 * step) / 2.0 + step / 2.0;

    for (x, y, tile) in grid.iter() {
        commands.spawn((
            Sprite {
                color:       tile.color(),
                custom_size: Some(Vec2::splat(config::TILE_SIZE)),
                ..default()
            },
            Transform::from_xyz(
                offset_x + x as f32 * step,
                offset_y + y as f32 * step,
                0.0,
            ),
            Visibility::default(),
            TileMarker { x, y },
        ));
    }
}

pub fn sync_tile_colors(
    grid:      Res<Grid>,
    mut query: Query<(&TileMarker, &mut Sprite)>,
) {
    if !grid.is_changed() {
        return;
    }
    for (marker, mut sprite) in query.iter_mut() {
        if let Some(tile) = grid.get(marker.x as i32, marker.y as i32) {
            sprite.color = tile.color();
        }
    }
}