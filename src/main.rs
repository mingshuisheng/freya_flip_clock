#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]


mod canvas_utils;

use std::time::Duration;
use freya::prelude::*;
use skia_safe::{Color, Font, FontStyle, M44, Paint, Point, Rect, V3, Size, RRect};
use skia_safe::textlayout::FontCollection;
#[allow(deprecated)]
use skia_safe::utils::View3D;
use tokio::time::sleep;
use crate::canvas_utils::CanvasUtils;
use chrono::{Local, Timelike};

const DOT_COLOR: &str = "white";
// const FONT_COLOR: Color = Color::WHITE;
// static CARD_COLOR: Color = Color::new(0xff161923);

const FONT_COLOR: Color = Color::WHITE;
static CARD_COLOR: Color = Color::new(0xff191919);

// const FONT_COLOR: Color = Color::new(0xff161923);
// static CARD_COLOR: Color = Color::WHITE;

const WINDOW_WIDTH: f64 = 1400.0;
const WINDOW_HEIGHT: f64 = 400.0;

fn main() {
  launch_cfg(app, LaunchConfig::<()>::builder()
    .with_width(WINDOW_WIDTH)
    .with_height(WINDOW_HEIGHT)
    .with_decorations(false)
    .with_transparency(true)
    .with_title("Floating window")
    .with_background("transparent")
    .build());
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

  rsx!(
    WindowDragArea {
      rect {
        width: "100%",
        height: "100%",
        direction: "horizontal",
        main_align: "center",
        cross_align: "center",
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
          background: DOT_COLOR
        }
        rect {height: "60%"}
        rect {
          width: "100%",
          height: "20%",
          corner_radius: radius.to_string(),
          corner_smoothing: "75%",
          background: DOT_COLOR
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

  let canvas = use_canvas(&(current_num(), angle.read().as_f32(), props.max), |(num, angle, num_max)| {
    Box::new(move |canvas: &skia_safe::Canvas, font_collection: &mut FontCollection, region| {
      canvas.with_restore(|canvas| {
        canvas.translate((region.origin.x, region.origin.y));

        let width = region.width();
        let height = region.height();
        let half_height = height / 2.0;
        let region_center = Point::new(width / 2.0, half_height);

        let center_space = width * 0.01;
        let card_size = Size::new(width, half_height - center_space);

        let up_rect = Rect::from_size(card_size);
        let down_rect = Rect::from_point_and_size(Point::new(0.0, half_height + center_space), card_size);

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
        background_paint.set_color(CARD_COLOR);

        let mut text_paint = Paint::default();
        text_paint.set_anti_alias(true);
        text_paint.set_color(FONT_COLOR);
        let typefaces =
          font_collection.find_typefaces(&["Times New Roman"], FontStyle::default());
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
            canvas.clip_rect(Rect::from_ltrb(f32::MIN, f32::MIN, f32::MAX, half_height - center_space), None, true);
          } else {
            canvas.clip_rect(Rect::from_ltrb(f32::MIN, half_height + center_space, f32::MAX, f32::MAX), None, true);
          }
          canvas.translate(region_center);

          // x axis rotate
          #[allow(deprecated)]
            let mut view3d = View3D::new();
          view3d.rotate_x(-angle);
          canvas.concat(&view3d.matrix());

          let rounded_rect = RRect::new_rect_radii(Rect::from_point_and_size(Point::new(-width / 2.0, -half_height), card_size), &radii);
          canvas.draw_rrect(rounded_rect, &background_paint);

          if angle > 90.0 {
            canvas.concat_44(&M44::rotate(V3::new(1.0, 0.0, 0.0), 180f32.to_radians()));
          }

          let num = if angle <= 90.0 {
            current
          } else {
            next
          };

          draw_num_offset(canvas, num, &font, &text_paint, width, height, -width / 2.0, -half_height);
        });
      });
    })
  });

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

fn draw_num(canvas: &skia_safe::Canvas, num: u32, font: &Font, text_paint: &Paint, width: f32, height: f32) {
  draw_num_offset(canvas, num, font, text_paint, width, height, 0.0, 0.0);
}

fn draw_num_offset(canvas: &skia_safe::Canvas, num: u32, font: &Font, text_paint: &Paint, width: f32, height: f32, offset_x: f32, offset_y: f32) {
  let num = num.to_string();
  let (_, text_rect) = font.measure_str(&num, Some(text_paint));
  canvas.draw_str(
    &num,
    Point::new((text_rect.left + width - text_rect.right) / 2.0 - text_rect.left + offset_x, (height - text_rect.height()) / 2.0 + text_rect.height() + offset_y),
    font,
    text_paint,
  );
}