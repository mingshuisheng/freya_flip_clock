use freya::prelude::*;

use crate::{app_config::AppConfig, app_state::use_app_conf};

#[allow(non_snake_case)]
#[component]
pub fn AppConfigContextProvide(children: Element) -> Element {
    let app_conf = use_app_conf();
    provide_context(AppConfigContext { app_conf });
    rsx!({ children })
}

pub fn use_app_conf_context() -> AppConfigContext {
    consume_context::<AppConfigContext>()
}

#[derive(Clone, Copy, PartialEq)]
pub struct AppConfigContext {
    pub app_conf: Signal<AppConfig>,
}
