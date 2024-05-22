use std::time::Duration;

use freya::prelude::*;
use tokio::time::sleep;

use crate::{app_config::AppConfig, constant::RATIO};

#[derive(Debug, Clone, Default)]
pub struct AppState {
    pub app_conf: AppConfig,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            app_conf: AppConfig::load(),
        }
    }
}

pub fn use_app_state() -> AppState {
    consume_context::<AppState>()
}

pub fn use_app_conf() -> (Signal<AppConfig>, impl FnMut(f32)) {
    let app_state = use_app_state();
    let mut app_conf = use_signal(|| app_state.app_conf);
    let mut task: Signal<Option<Task>> = use_signal(|| None);
    let platform = use_platform();

    use_effect(use_reactive(&app_conf.read().clone(), move |app_conf| {
        if let Some(task) = task.write().take() {
            task.cancel();
        }
        let move_task = Some(spawn(async move {
            sleep(Duration::from_millis(1500)).await;
            app_conf.save().await;
            task.write().take();
        }));
        task.replace(move_task);
    }));

    let update_size = move |delta_width: f32| {
        let window_size = platform.info().window_size;
        let new_width = window_size.width + delta_width;
        app_conf.write().size = new_width as f64;
        let new_height = new_width / RATIO as f32;
        platform.set_window_size(Size2D::new(new_width, new_height));
    };

    (app_conf, update_size)
}
