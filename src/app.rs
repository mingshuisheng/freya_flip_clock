use crate::{app_config::to_window_level, components::*, constant::RATIO, times::use_current_time};
use freya::prelude::*;
use mouce::Mouse;

#[allow(non_snake_case)]
#[component]
pub fn App() -> Element {
    rsx!(AppConfigContextProvide {
        MyApp{}
    })
}

#[allow(non_snake_case)]
#[component]
pub fn MyApp() -> Element {
    let app_config_context = use_app_conf_context();
    let mut app_conf = app_config_context.app_conf;
    let card_color = app_conf().card_color;
    let platform = use_platform();
    let scale_factor = use_scale_factor().0;

    let radius = app_conf().size as f32 / scale_factor() * 0.04285 * 0.33333;

    let mut handle_lock = move || {
        app_conf.write().lock = !app_conf().lock;
    };

    let handle_window_moved = move |e: WindowMovedEvent| {
        app_conf.write().x = e.get_x();
        app_conf.write().y = e.get_y();
    };

    let handle_size_change = move |(new_size, _): (Size2D, Size2D)| {
        app_conf.write().size = new_size.width as f64;
    };

    let mut opacity = use_signal(|| "0");

    let mouse_manager = Mouse::new();
    let handle_mouse_over = move |_| {
        let window_size = app_conf().window_size();
        let window_position = app_conf().window_position();
        let (x, y) = mouse_manager.get_position().unwrap();
        let x = x as f32;
        let y = y as f32;
        if x < window_position.0
            || x > (window_position.0 + window_size.0)
            || y < window_position.1
            || y > (window_position.1 + window_size.1)
        {
            opacity.set("0");
        } else {
            opacity.set("1");
        }
    };

    let window_level = to_window_level(app_conf().window_level);

    let mut handle_level = move || {
        app_conf.write().window_level = (app_conf().window_level + 1) % 3;
        platform.set_window_level(to_window_level(app_conf().window_level));
    };

    rsx!(
        WindowDragArea {
          enable: !app_conf().lock,
          WindowDragResizeArea {
            enable: !app_conf().lock,
            aspect_ratio: RATIO,
            on_size_change: handle_size_change,
            rect {
              width: "100%",
              height: "100%",
              main_align: "center",
              cross_align: "center",
              onwindowmoved: handle_window_moved,
              onglobalmouseover: handle_mouse_over,
              // border: "2 solid red",
              rect {
                width: "100%",
                height: "80%",
                direction: "horizontal",
                main_align: "center",
                cross_align: "center",
                MainArea{}
              }
              rect {
                height: "1%",
              }
              rect {
                width: "98%",
                height: "19%",
                background: card_color,
                opacity: opacity(),
                corner_radius: radius.to_string(),
                corner_smoothing: "75%",
                Tools {
                    locked: app_conf().lock,
                    window_level: window_level,
                    on_close_click: move |_| platform.exit(),
                    on_lock_click: move |_| handle_lock(),
                    on_level_click: move |_| handle_level(),
                }
              }
            }
        }
      }
    )
}

#[allow(non_snake_case)]
#[component]
pub fn MainArea() -> Element {
    let (hour, minute, second) = use_current_time();
    rsx!(
        NumGroup {
            num: hour(),
            max_num: 23,
          }
          Splitter{}
          NumGroup{
            num: minute(),
            max_num: 59,
          }
          Splitter{}
          NumGroup{
            num: second(),
            max_num: 59,
          }
    )
}
