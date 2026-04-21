pub mod components;
pub mod layout;
pub mod systems;

pub use components::TickCount;
pub use layout::spawn_hud;
pub use systems::{
    count_ticks, handle_pause_button, handle_speed_buttons,
    update_button_styles, update_tick_label, update_speed_label // Add this
};