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

fn main() {
  launch_cfg(app, LaunchConfig::<()>::builder()
    .with_width(685.0)
    .with_height(200.0)
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
        rect {
          width: "20"
        }
        NumGroup{
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
        rect {
          width: "20"
        }
      }
    }
  )
}

#[allow(non_snake_case)]
#[component]
pub fn Splitter() -> Element {
  rsx!(
    rect {
      main_align: "center",
      cross_align: "center",
      padding: "10 5",
      rect{
        width: "10",
        height: "10",
        corner_radius: "10",
        corner_smoothing: "75%",
        background: DOT_COLOR
      }
      rect{
        width: "10",
        height: "10",
        margin: "30 0 0 0",
        corner_radius: "10",
        corner_smoothing: "75%",
        background: DOT_COLOR
      }
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
      rect {
          width: "100",
          height: "200",
          position: "relative",
          font_size: "400",
          color: "white",
          background: "transparent",
          // margin: "0 0 0 10",
          overflow: "none",
          Num {
            num: props.num / 10,
            max: props.max_num / 10,
          }
        }
        rect {width: "10"}
        rect {
          width: "100",
          height: "200",
          position: "relative",
          font_size: "400",
          color: "white",
          background: "transparent",
          // margin: "0 0 0 10",
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
        // let size = Size::new(width, height);
        let half_size = Size::new(width, height / 2.0);
        let origin_point = Point::new(0.0, 0.0);
        let half_point = Point::new(0.0, height / 2.0);
        let up_rect = Rect::from_point_and_size(origin_point, half_size);
        let down_rect = Rect::from_point_and_size(half_point, half_size);
        let current = num;
        let next = (num + 1) % (num_max + 1);
        let radius = 10.0;
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

        //上半部分的背后数字
        canvas.with_restore(|canvas| {
          canvas.clip_rect(up_rect, None, true);
          let rounded_rect = RRect::new_rect_radii(up_rect, &radii);
          canvas.draw_rrect(rounded_rect, &background_paint);
          draw_num(canvas, next, &font, &text_paint, width, height);
        });

        //下半部分的背后数字
        canvas.with_restore(|canvas| {
          canvas.clip_rect(down_rect, None, true);
          let rounded_rect = RRect::new_rect_radii(down_rect, &radii);
          canvas.draw_rrect(rounded_rect, &background_paint);
          draw_num(canvas, current, &font, &text_paint, width, height);
        });

        canvas.with_restore(|canvas| {
          if angle <= 90.0 {
            canvas.clip_rect(Rect::from_ltrb(f32::MIN, f32::MIN, f32::MAX, half_height), None, true);
          } else {
            canvas.clip_rect(Rect::from_ltrb(f32::MIN, half_height, f32::MAX, f32::MAX), None, true);
          }
          canvas.translate((half_point.x + width / 2.0, half_point.y));


          if angle == 90.0 {
            let mut paint = Paint::default();
            paint.set_anti_alias(true);
            paint.set_color(CARD_COLOR);
            paint.set_stroke_width(5.0);
            canvas.draw_line(Point::new(-width / 2.0, 0.0), Point::new(width / 2.0, 0.0), &paint);
          }
          #[allow(deprecated)]
            let mut view3d = View3D::new();
          view3d.rotate_x(-angle);
          canvas.concat(&view3d.matrix());
          let rounded_rect = RRect::new_rect_radii(Rect::from_point_and_size(Point::new(-width / 2.0, -half_height), half_size), &radii);
          // canvas.draw_rect(r, &background_paint);
          canvas.draw_rrect(rounded_rect, &background_paint);

          if angle > 90.0 {
            canvas.concat_44(&M44::rotate(V3::new(1.0, 0.0, 0.0), 180f32.to_radians()));
          }

          let current = if angle <= 90.0 {
            current
          } else {
            next
          };

          draw_num_offset(canvas, current, &font, &text_paint, width, height, -width / 2.0, -half_height);
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