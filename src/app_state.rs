use std::time::Duration;

use freya::prelude::*;
use tokio::time::sleep;

use crate::app_config::AppConfig;

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

pub fn use_app_conf() -> Signal<AppConfig> {
    let app_state = use_app_state();
    let app_conf = use_signal(|| app_state.app_conf);
    let mut task: Signal<Option<Task>> = use_signal(|| None);

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

    app_conf
}
