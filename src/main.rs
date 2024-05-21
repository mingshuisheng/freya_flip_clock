#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod canvas_utils;
mod colors;

use crate::canvas_utils::CanvasUtils;
use crate::colors::Parse;
use chrono::{Local, Timelike};
use freya::prelude::*;
use skia_safe::textlayout::FontCollection;
#[allow(deprecated)]
use skia_safe::utils::View3D;
use skia_safe::{Color, Font, FontStyle, Paint, Point, RRect, Rect, Size, M44, V3};
use std::collections::HashMap;
use std::io::Read;
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, Copy, Clone, Default)]
struct AppState {
    dot_color: &'static str,
    card_color: Color,
    font_color: Color,
}

fn main() {
    let default_app_state = AppState {
        dot_color: "white",
        font_color: Color::WHITE,
        card_color: Color::new(0xff191919),
    };

    let mut default_dot_color = default_app_state.dot_color;

    let ratio: f64 = 3.5;
    let mut window_width: f64 = 700.0;
    let mut window_height: f64 = window_width / ratio;

    let conf_file = std::fs::File::open("./conf.txt");

    let app_state = if let Ok(mut conf_file) = conf_file {
        let mut config_str = String::new();
        let _ = conf_file.read_to_string(&mut config_str).unwrap();
        let configs = config_str
            .trim()
            .split("\n")
            .map(|s| {
                let name_value: Vec<&str> = s.trim().split("=").collect();
                (
                    *name_value.get(0).unwrap_or(&""),
                    *name_value.get(1).unwrap_or(&""),
                )
            })
            .fold(HashMap::<String, String>::new(), |mut map, current| {
                map.insert(current.0.to_string(), current.1.to_string());
                map
            });

        if let Some(dot_color) = configs.get("dot_color") {
            default_dot_color = Box::leak(dot_color.clone().into_boxed_str());
        }

        if let Some(width_str) = configs.get("size") {
            if let Ok(width) = width_str.parse::<f64>() {
                window_width = width;
                window_height = width / ratio;
            }
        }

        AppState {
            dot_color: default_dot_color,
            font_color: Color::parse(configs.get("font_color").unwrap_or(&("white".to_string())))
                .unwrap(),
            card_color: Color::parse(
                configs
                    .get("card_color")
                    .unwrap_or(&("#191919".to_string())),
            )
            .unwrap(),
        }
    } else {
        default_app_state
    };

    launch_cfg(
        app,
        LaunchConfig::<AppState>::builder()
            .with_width(window_width)
            .with_height(window_height)
            .with_decorations(false)
            .with_transparency(true)
            .with_skip_taskbar(true)
            .with_window_level(WindowLevel::AlwaysOnTop)
            .with_resizable(false)
            .with_title("Floating window")
            .with_background("transparent")
            .with_state(app_state)
            .build(),
    );
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

    let mut locked = use_signal(|| false);

    let mut handle_lock = move || locked.set(!locked());

    let handle_keydown = move |e: KeyboardEvent| {
        if e.key == Key::Escape {
            handle_exit();
        } else if e.key == Key::Enter {
            handle_lock();
        }
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

    let AppState { dot_color, .. } = consume_context::<AppState>();

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
            background: dot_color
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
    let AppState {
        card_color,
        font_color,
        ..
    } = consume_context::<AppState>();

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
                            let mut view3d = View3D::new();
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
