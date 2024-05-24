use freya::prelude::*;

use super::svg::*;
use crate::{
    app_state::use_app_conf,
    components::{use_app_conf_context, use_scale_factor},
};

#[derive(Props, Clone, PartialEq)]
pub struct ToolsProps {
    pub locked: bool,
    pub window_level: WindowLevel,
    pub on_close_click: Option<EventHandler<()>>,
    pub on_lock_click: Option<EventHandler<()>>,
    pub on_level_click: Option<EventHandler<()>>,
}

#[allow(non_snake_case)]
#[component]
pub fn Tools(props: ToolsProps) -> Element {
    let app_conf = use_app_conf();
    let font_color = app_conf().font_color;
    let app_config_context = use_app_conf_context();
    let app_conf = app_config_context.app_conf;
    let scale_factor = use_scale_factor().0;
    let margin = app_conf().size as f32 / scale_factor() * 0.02;

    let handle_close = move |e: MouseEvent| {
        e.stop_propagation();
        props.on_close_click.as_ref().map(|f| f.call(()));
    };

    let handle_lock = move |e: MouseEvent| {
        e.stop_propagation();
        props.on_lock_click.as_ref().map(|f| f.call(()));
    };

    let handle_level = move |e: MouseEvent| {
        e.stop_propagation();
        props.on_level_click.as_ref().map(|f| f.call(()));
    };

    let platform = use_platform();
    let mut is_hovering = use_signal(|| false);

    let onmouseover = move |e: MouseEvent| {
        e.stop_propagation();
        *is_hovering.write() = true;
        platform.set_cursor(CursorIcon::Pointer);
    };

    let onmouseleave = move |_| {
        *is_hovering.write() = false;
        platform.set_cursor(CursorIcon::default());
    };

    use_drop(move || {
        if *is_hovering.peek() {
            platform.set_cursor(CursorIcon::default());
        }
    });

    let icon_width = "5.714%";
    let icon_height = "80%";

    rsx!(
      rect{
        width: "100%",
        height: "100%",
        direction: "horizontal",
        color: "red",
        position: "absolute",
        rect {
          height: "100%",
          direction: "horizontal",
          cross_align: "center",
          position_right: (margin * 4.0).to_string(),
          rect {
            width: icon_width,
            height: icon_height,
            onclick: handle_close,
            onmouseover,
            onmouseleave,
            CloseSvg {
              stroke_color: font_color.clone()
            }
          }
        }
        rect {
          height: "100%",
          position_left: margin.to_string(),
          direction: "horizontal",
          main_align: "center",
          cross_align: "center",
          rect {
            width: icon_width,
            height: icon_height,
            onclick: handle_lock,
            onmouseover,
            onmouseleave,
            if props.locked {
                LockedSvg {
                  stroke_color: font_color.clone()
                }
            }else {
                UnLockedSvg {
                  stroke_color: font_color.clone()
                }
            }
          }
          rect {
            width: icon_width,
            height: icon_height,
            onclick: handle_level,
            onmouseover,
            onmouseleave,
            WindowLevelIcon{ window_level: props.window_level, stroke_color: font_color.clone() }
          }
        }
      }
    )
}

#[allow(non_snake_case)]
#[component]
pub fn WindowLevelIcon(window_level: WindowLevel, stroke_color: String) -> Element {
    match window_level {
        WindowLevel::AlwaysOnBottom => rsx!(ToBottomSvg { stroke_color }),
        WindowLevel::Normal => rsx!(ToNormalSvg { stroke_color }),
        WindowLevel::AlwaysOnTop => rsx!(ToTopSvg { stroke_color }),
    }
}
