use freya::prelude::*;

use crate::{app_config::AppConfig, app_state::use_app_conf};

#[allow(non_snake_case)]
#[component]
pub fn AppConfigContextProvide(children: Element) -> Element {
    let mut app_conf = use_app_conf();
    provide_context(AppConfigContext { app_conf });
    let platform = use_platform();

    let mut scale_factor = use_signal(|| platform.info().window_scale_factor);

    provide_context(ScaleFactorContext(scale_factor));

    let handle_scale_change = move |e: ScaleFactorEvent| {
        let current_scale_factor = e.get_scale_factor();
        app_conf.write().size =
            app_conf().size / scale_factor() as f64 * current_scale_factor as f64;
        scale_factor.set(current_scale_factor);
    };

    rsx!(rect {
        onglobalscalefactorchange: handle_scale_change,
        {children}
    })
}

pub fn use_app_conf_context() -> AppConfigContext {
    consume_context::<AppConfigContext>()
}

#[derive(Clone, Copy, PartialEq)]
pub struct AppConfigContext {
    pub app_conf: Signal<AppConfig>,
}

#[derive(Clone, Copy, PartialEq)]
pub struct ScaleFactorContext(pub Signal<f32>);

pub fn use_scale_factor() -> ScaleFactorContext {
    consume_context::<ScaleFactorContext>()
}
