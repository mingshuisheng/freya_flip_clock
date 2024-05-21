#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod canvas_utils;
mod colors;

use crate::canvas_utils::CanvasUtils;
use crate::colors::Parse;
use chrono::{Local, Timelike};
use freya::dioxus::html::geometry::euclid::Size2D;
use freya::prelude::*;
use serde::{Deserialize, Serialize};
use skia_safe::textlayout::FontCollection;
#[allow(deprecated)]
use skia_safe::utils::View3D;
use skia_safe::{Color, Font, FontStyle, Paint, Point, RRect, Rect, Size, M44, V3};
use std::fs::File;
use std::io::{Read, Write};
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::time::sleep;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct AppConfig {
    dot_color: String,
    card_color: String,
    font_color: String,
    size: f64,
    x: i32,
    y: i32,
    lock: bool,
}

impl AppConfig {
    pub fn get_conf_path() -> String {
        "./FlipClock.json".to_owned()
    }

    pub fn load() -> Self {
        let conf_file = File::open(Self::get_conf_path());
        let default_app_config = AppConfig {
            dot_color: "#cccccc".to_string(),
            card_color: "#191919".to_string(),
            font_color: "#cccccc".to_string(),
            size: 700.0,
            x: 100,
            y: 100,
            lock: false,
        };

        let write_file = || {
            let conf_file = File::create(Self::get_conf_path());
            if let Ok(mut conf_file) = conf_file {
                let _ = conf_file.write_all(default_app_config.to_json().as_bytes());
            }
            default_app_config
        };

        if let Ok(mut conf_file) = conf_file {
            let mut config_str = String::new();
            let _ = conf_file.read_to_string(&mut config_str).unwrap();

            if let Ok(app_conf) = serde_json::from_str::<AppConfig>(&config_str) {
                app_conf
            } else {
                write_file()
            }
        } else {
            write_file()
        }
    }

    pub async fn save(&self) {
        let conf_file = tokio::fs::File::create(Self::get_conf_path()).await;
        if let Ok(mut conf_file) = conf_file {
            conf_file
                .write_all(self.to_json().as_bytes())
                .await
                .unwrap();
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}

#[derive(Debug, Clone, Default)]
struct AppState {
    app_conf: AppConfig,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            app_conf: AppConfig::load(),
        }
    }
}

const RATIO: f64 = 3.5;

fn main() {
    let app_state = AppState::new();

    let window_width = app_state.app_conf.size;

    let config = LaunchConfig::<AppState>::builder()
        .with_width(window_width)
        .with_height(window_width / RATIO)
        .with_position(app_state.app_conf.x, app_state.app_conf.y)
        .with_decorations(false)
        .with_transparency(true)
        .with_skip_taskbar(true)
        .with_window_level(WindowLevel::AlwaysOnTop)
        .with_resizable(false)
        .with_title("Floating window")
        .with_background("transparent")
        .with_state(app_state);

    launch_cfg(app, config.build());
}

fn app() -> Element {
    let mut hour = use_signal(|| Local::now().hour());
    let mut minute = use_signal(|| Local::now().minute());
    let mut second = use_signal(|| Local::now().second());

    use_effect(move || {
        spawn(async move {
            loop {
                sleep(Duration::from_millis(1000)).await;
                let now = Local::now();
                hour.set(now.hour());
                minute.set(now.minute());
                second.set(now.second());
            }
        });
    });

    let app_state = consume_context::<AppState>();
    let mut app_conf = use_signal(|| app_state.app_conf);
    let mut task: Signal<Option<Task>> = use_signal(|| None);

    let mut save_conf = move || {
        if let Some(task) = task.write().take() {
            task.cancel();
        }
        let move_task = Some(spawn(async move {
            sleep(Duration::from_millis(1500)).await;
            app_conf().save().await;
            task.write().take();
        }));
        task.replace(move_task);
    };

    let mut locked = use_signal(|| app_conf.read().lock);

    let mut handle_lock = move || {
        locked.set(!locked());
        app_conf.write().lock = locked();
        save_conf();
    };

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

    let mut start_resize = use_signal(|| false);

    let handle_resize = move |e: WheelEvent| {
        if locked() || !start_resize() {
            return;
        }
        e.stop_propagation();
        let wheel_y = e.get_delta_y() as f32;
        let window_size = platform.info().window_size;
        let new_width = window_size.width + wheel_y;
        let new_height = new_width / RATIO as f32;
        platform.set_window_size(Size2D::new(new_width, new_height));
        app_conf.write().size = new_width as f64;
        save_conf();
    };

    let handle_keydown = move |e: KeyboardEvent| {
        if e.key == Key::Escape {
            handle_exit();
        } else if e.key == Key::Enter {
            handle_lock();
        } else if e.key == Key::Control {
            start_resize.set(true);
        }
    };

    let handle_keyup = move |e: KeyboardEvent| {
        if locked() {
            return;
        }
        if e.key == Key::Control {
            start_resize.set(false);
        }
    };

    let handle_window_moved = move |e: WindowMovedEvent| {
        app_conf.write().x = e.get_x();
        app_conf.write().y = e.get_y();
        save_conf();
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
          onkeyup: handle_keyup,
          onwheel: handle_resize,
          onwindowmoved: handle_window_moved,
          // border: "2 solid red",
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
        }
      }
    )
}

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

#[derive(Props, Clone, PartialEq, Debug)]
pub struct NumGroupProps {
    num: u32,
    max_num: u32,
}

#[allow(non_snake_case)]
#[component]
pub fn NumGroup(props: NumGroupProps) -> Element {
    rsx!(
      rect {
        direction: "horizontal",
        width: "30%",
        height: "100%",
        rect {
            width: "47.619%",
            height: "100%",
            position: "relative",
            color: "white",
            background: "transparent",
            overflow: "none",
            Num {
              num: props.num / 10,
              max: props.max_num / 10,
            }
          }
          rect {width: "4.7619%"}
          rect {
            width: "47.619%",
            height: "100%",
            position: "relative",
            color: "white",
            background: "transparent",
            overflow: "none",
            Num {
              num: props.num % 10,
              max: 9
            }
         }
      }
    )
}

#[derive(Props, Clone, PartialEq, Debug)]
pub struct NumProps {
    pub num: u32,
    pub max: u32,
}

#[allow(non_snake_case)]
#[component]
pub fn Num(props: NumProps) -> Element {
    let mut current_num = use_signal(|| props.num);
    let mut next_num = use_signal(|| props.num);

    let app_conf = consume_context::<AppState>().app_conf;

    let card_color = Color::parse(&app_conf.card_color)
        .ok()
        .unwrap_or(Color::BLACK);

    let font_color = Color::parse(&app_conf.font_color)
        .ok()
        .unwrap_or(Color::WHITE);

    let animation = use_animation(|ctx| {
        ctx.with(
            AnimNum::new(0.0, 180.0)
                .time(500)
                .ease(Ease::Out)
                .function(Function::Back),
        )
    });

    let angle = animation.get();

    if props.num != current_num() && props.num != next_num() && !animation.is_running() {
        animation.start();
        next_num.set(props.num);
    }

    if current_num() != next_num() && !animation.is_running() {
        current_num.set(next_num());
        animation.reset();
    }

    let canvas = use_canvas(
        &(current_num(), angle.read().as_f32(), props.max),
        move |(num, angle, num_max)| {
            Box::new(
                move |canvas: &skia_safe::Canvas, font_collection: &mut FontCollection, region| {
                    canvas.with_restore(|canvas| {
                        canvas.translate((region.origin.x, region.origin.y));

                        let width = region.width();
                        let height = region.height();
                        let half_height = height / 2.0;
                        let region_center = Point::new(width / 2.0, half_height);

                        let center_space = width * 0.01;
                        let card_size = Size::new(width, half_height - center_space);

                        let up_rect = Rect::from_size(card_size);
                        let down_rect = Rect::from_point_and_size(
                            Point::new(0.0, half_height + center_space),
                            card_size,
                        );

                        let current = num;
                        let next = (num + 1) % (num_max + 1);
                        let radius = width * 0.1;
                        let radii = [
                            (radius, radius).into(),
                            (radius, radius).into(),
                            (radius, radius).into(),
                            (radius, radius).into(),
                        ];

                        let mut background_paint = Paint::default();
                        background_paint.set_anti_alias(true);
                        background_paint.set_color(card_color);

                        let mut text_paint = Paint::default();
                        text_paint.set_anti_alias(true);
                        text_paint.set_color(font_color);
                        let typefaces = font_collection
                            .find_typefaces(&["Times New Roman"], FontStyle::default());
                        let font = Font::new(
                            typefaces
                                .first()
                                .expect("'Times New Roman' font not found."),
                            region.size.height,
                        );

                        let draw_card = |num: u32, rect: Rect| {
                            canvas.with_restore(|canvas| {
                                canvas.clip_rect(rect, None, true);
                                let rounded_rect = RRect::new_rect_radii(rect, &radii);
                                canvas.draw_rrect(rounded_rect, &background_paint);
                                draw_num(canvas, num, &font, &text_paint, width, height);
                            });
                        };

                        //上半部分的背后数字
                        draw_card(next, up_rect);
                        //下半部分的背后数字
                        draw_card(current, down_rect);

                        canvas.with_restore(|canvas| {
                            if angle <= 90.0 {
                                canvas.clip_rect(
                                    Rect::from_ltrb(
                                        f32::MIN,
                                        f32::MIN,
                                        f32::MAX,
                                        half_height - center_space,
                                    ),
                                    None,
                                    true,
                                );
                            } else {
                                canvas.clip_rect(
                                    Rect::from_ltrb(
                                        f32::MIN,
                                        half_height + center_space,
                                        f32::MAX,
                                        f32::MAX,
                                    ),
                                    None,
                                    true,
                                );
                            }
                            canvas.translate(region_center);

                            // x axis rotate
                            #[allow(deprecated)]
                            let mut view3d = View3D::default();
                            view3d.rotate_x(-angle);
                            canvas.concat(&view3d.matrix());

                            let rounded_rect = RRect::new_rect_radii(
                                Rect::from_point_and_size(
                                    Point::new(-width / 2.0, -half_height),
                                    card_size,
                                ),
                                &radii,
                            );
                            canvas.draw_rrect(rounded_rect, &background_paint);

                            if angle > 90.0 {
                                canvas.concat_44(&M44::rotate(
                                    V3::new(1.0, 0.0, 0.0),
                                    180f32.to_radians(),
                                ));
                            }

                            let num = if angle <= 90.0 { current } else { next };

                            draw_num_offset(
                                canvas,
                                num,
                                &font,
                                &text_paint,
                                width,
                                height,
                                -width / 2.0,
                                -half_height,
                            );
                        });
                    });
                },
            )
        },
    );

    rsx! {
      Canvas {
          canvas,
          theme: theme_with!(CanvasTheme {
              background: "transparent".into(),
              width: "100%".into(),
              height: "100%".into(),
          })
      }
    }
}

fn draw_num(
    canvas: &skia_safe::Canvas,
    num: u32,
    font: &Font,
    text_paint: &Paint,
    width: f32,
    height: f32,
) {
    draw_num_offset(canvas, num, font, text_paint, width, height, 0.0, 0.0);
}

fn draw_num_offset(
    canvas: &skia_safe::Canvas,
    num: u32,
    font: &Font,
    text_paint: &Paint,
    width: f32,
    height: f32,
    offset_x: f32,
    offset_y: f32,
) {
    let num = num.to_string();
    let (_, text_rect) = font.measure_str(&num, Some(text_paint));
    canvas.draw_str(
        &num,
        Point::new(
            (text_rect.left + width - text_rect.right) / 2.0 - text_rect.left + offset_x,
            (height - text_rect.height()) / 2.0 + text_rect.height() + offset_y,
        ),
        font,
        text_paint,
    );
}
