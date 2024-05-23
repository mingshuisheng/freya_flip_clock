#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod app;
mod app_config;
mod app_state;
mod canvas_utils;
mod colors;
mod components;
mod constant;
mod hooks;
mod times;

use app::App;
use app_config::to_window_level;
use app_state::AppState;
use constant::RATIO;
use freya::{launch::launch_cfg, prelude::LaunchConfig};

fn main() {
    let app_state = AppState::new();

    let window_width = app_state.app_conf.size;
    let window_level = to_window_level(app_state.app_conf.window_level);

    let config = LaunchConfig::<AppState>::builder()
        .with_width(window_width)
        .with_height(window_width / RATIO as f64)
        .with_position(app_state.app_conf.x, app_state.app_conf.y)
        .with_decorations(false)
        .with_transparency(true)
        .with_skip_taskbar(true)
        .with_window_level(window_level)
        .with_resizable(false)
        .with_title("Flip clock window")
        .with_background("transparent")
        .with_state(app_state);

    launch_cfg(App, config.build());
}
