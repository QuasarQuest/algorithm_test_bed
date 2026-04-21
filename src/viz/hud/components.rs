// src/viz/hud/components.rs

use bevy::prelude::*;

#[derive(Component)] pub struct PauseButtonMarker;
#[derive(Component)] pub struct PauseLabelMarker;

// New Speed Controls
#[derive(Component)] pub struct SpeedDecreaseButton;
#[derive(Component)] pub struct SpeedIncreaseButton;
#[derive(Component)] pub struct SpeedResetButton;
#[derive(Component)] pub struct CurrentSpeedLabel;

#[derive(Component)] pub struct TickLabelMarker;

#[derive(Resource, Default)]
pub struct TickCount(pub u64);