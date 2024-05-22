use freya::prelude::*;
#[allow(deprecated)]
use skia_safe::utils::View3D;
use skia_safe::{Color, Font, FontStyle, Paint, Point, RRect, Rect, Size, M44, V3};

use crate::{canvas_utils::CanvasUtils, colors::Parse, AppState};

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
            Box::new(move |canvas, font_collection, region| {
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
            })
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
