use crate::{app_state::use_app_conf, components::*, times::use_current_time};
use chrono::Local;
use freya::prelude::*;

#[allow(non_snake_case)]
#[component]
pub fn App() -> Element {
    let (mut app_conf, mut update_size) = use_app_conf();

    let platform = use_platform();
    let mut exit_record = use_signal(|| 0i64);
    let mut handle_exit = move || {
        if exit_record() == 0 {
            exit_record.set(Local::now().timestamp_millis());
        } else {
            let now = Local::now().timestamp_millis();
            let diff = now - exit_record();
            if diff > 500 {
                exit_record.set(now);
            } else {
                platform.exit();
            }
        }
    };

    let mut locked = use_signal(|| app_conf.read().lock);
    let mut handle_lock = move || {
        locked.set(!locked());
        app_conf.write().lock = locked();
    };

    let handle_resize = move |e: WheelEvent| {
        if locked() {
            return;
        }
        e.stop_propagation();
        update_size(e.get_delta_y() as f32);
    };

    let handle_keydown = move |e: KeyboardEvent| {
        if e.key == Key::Escape {
            handle_exit();
        } else if e.key == Key::Enter {
            handle_lock();
        }
    };

    let handle_window_moved = move |e: WindowMovedEvent| {
        app_conf.write().x = e.get_x();
        app_conf.write().y = e.get_y();
    };

    rsx!(
      WindowDragArea {
        enable: !locked(),
        rect {
          width: "100%",
          height: "100%",
          direction: "horizontal",
          main_align: "center",
          cross_align: "center",
          onkeydown: handle_keydown,
          onwheel: handle_resize,
          onwindowmoved: handle_window_moved,
          // border: "2 solid red",
          MainArea{}
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
