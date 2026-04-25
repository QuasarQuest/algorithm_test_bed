// src/viz/hud/mod.rs

pub mod components;
pub mod layout;
pub mod scoreboard;
pub mod systems;

pub use components::TickCount;
pub use layout::spawn_hud;
pub use scoreboard::{spawn_scoreboard, update_scoreboard};
pub use systems::{
    count_ticks, handle_pause_button, handle_speed_buttons,
    update_button_styles, update_tick_label, update_speed_label,
};
