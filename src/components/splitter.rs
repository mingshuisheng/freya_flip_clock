use crate::AppState;
use freya::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn Splitter() -> Element {
    let platform = use_platform();
    let PlatformInformation { window_size } = platform.info();

    let radius = window_size.width * 0.04285 * 0.33333;

    let app_conf = consume_context::<AppState>().app_conf;
    let dot_color = app_conf.dot_color;

    rsx!(
      rect {
        width: "4.285%",
        height: "25%",
        direction: "horizontal",
        rect {width: "33.333%"}
        rect {
          width: "33.333%",
          height: "100%",
          main_align: "center",
          cross_align: "center",
          rect {
            width: "100%",
            height: "20%",
            corner_radius: radius.to_string(),
            corner_smoothing: "75%",
            background: dot_color.clone()
          }
          rect {height: "60%"}
          rect {
            width: "100%",
            height: "20%",
            corner_radius: radius.to_string(),
            corner_smoothing: "75%",
            background: dot_color
          }
        }
        rect {width: "33.333%"}
      }
    )
}
