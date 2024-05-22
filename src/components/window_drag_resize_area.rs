use std::sync::atomic::{AtomicI64, Ordering};

use freya::prelude::*;
use mouce::{common::MouseButton, common::MouseEvent, Mouse};
use tokio::sync::broadcast::channel;

const EDGE_SIZE: f32 = 10.0;

const NORTH_DIRECTION: [ResizeDirection; 3] = [
    ResizeDirection::North,
    ResizeDirection::NorthEast,
    ResizeDirection::NorthWest,
];

const SOUTH_DIRECTION: [ResizeDirection; 3] = [
    ResizeDirection::South,
    ResizeDirection::SouthEast,
    ResizeDirection::SouthWest,
];

const WEST_DIRECTION: [ResizeDirection; 3] = [
    ResizeDirection::West,
    ResizeDirection::NorthWest,
    ResizeDirection::SouthWest,
];

const EAST_DIRECTION: [ResizeDirection; 3] = [
    ResizeDirection::East,
    ResizeDirection::NorthEast,
    ResizeDirection::SouthEast,
];

const CORNER_DIRECTION: [ResizeDirection; 4] = [
    ResizeDirection::NorthEast,
    ResizeDirection::NorthWest,
    ResizeDirection::SouthEast,
    ResizeDirection::SouthWest,
];

#[derive(Props, Clone, PartialEq)]
pub struct WindowDragResizeAreaProps {
    pub edge_size: Option<f32>,
    pub aspect_ratio: Option<f32>,
    // work only aspect_ratio is not None
    pub on_size_change: Option<EventHandler<(Size2D, Size2D)>>,
    pub children: Element,
}

#[allow(non_snake_case)]
#[component]
pub fn WindowDragResizeArea(props: WindowDragResizeAreaProps) -> Element {
    let platform = use_platform();
    let mut resize_direction = use_signal(|| None as Option<ResizeDirection>);
    let mut start_resize = use_signal(|| false);

    let mut edge_size = use_signal(|| props.edge_size.unwrap_or(EDGE_SIZE));

    use_effect(use_reactive(&props.edge_size, move |value| {
        edge_size.set(value.unwrap_or(EDGE_SIZE));
    }));

    let mut aspect_ratio = use_signal(|| props.aspect_ratio);
    use_effect(use_reactive(&props.aspect_ratio, move |value| {
        aspect_ratio.set(value);
    }));

    let mut on_size_change = use_signal(|| props.on_size_change);
    use_effect(use_reactive(&props.on_size_change, move |value| {
        on_size_change.set(value);
    }));

    let onmouseover = move |e: freya::prelude::MouseEvent| {
        if start_resize() {
            return;
        }
        let PlatformInformation { window_size, .. } = platform.info();
        let position = e.data().get_screen_coordinates().to_f32();
        resize_direction.set(cursor_resize_direction(window_size, position, edge_size()));
        platform.set_cursor(get_cursor_icon(resize_direction().clone()));
    };

    let onmouseleave = move |_| {
        if start_resize() {
            return;
        }
        resize_direction.set(None);
    };

    let onmousedown = move |e: freya::prelude::MouseEvent| {
        if aspect_ratio().is_none() && resize_direction().is_some() {
            e.stop_propagation();
            platform.drag_resize_window(resize_direction().unwrap());
            return;
        }
        if resize_direction().is_some() {
            e.stop_propagation();
        }
    };

    use_effect(move || {
        let (tx, mut rx) = channel::<mouce::common::MouseEvent>(100);
        let mut mouse_manager = Mouse::new();
        // 这个时间是用来约束报告率过高的鼠标，鼠标如果报告速度小于5ms，那么界面会出现卡顿
        let last_move_time = AtomicI64::default();

        mouse_manager
            .hook(Box::new(move |e| match e {
                MouseEvent::Press(MouseButton::Left) | MouseEvent::Release(MouseButton::Left) => {
                    tx.send(e.clone()).ok();
                }
                MouseEvent::AbsoluteMove(_, _) => {
                    let last_time = last_move_time.load(Ordering::Relaxed);
                    let current_time = chrono::Local::now().timestamp_millis();
                    if last_time == 0 {
                        last_move_time.store(current_time, Ordering::Relaxed);
                        tx.send(e.clone()).ok();
                        return;
                    }

                    let diff = current_time - last_time;

                    if diff > 5 {
                        last_move_time.store(current_time, Ordering::Relaxed);
                        tx.send(e.clone()).ok();
                    }
                }
                _ => {}
            }))
            .ok();

        spawn(async move {
            let mut last_position = mouse_manager.get_position().unwrap();
            let PlatformInformation {
                mut window_size,
                mut window_position,
            } = platform.info();

            loop {
                let e = rx.recv().await.ok();
                if e.is_none() || aspect_ratio().is_none() {
                    continue;
                }
                match e.unwrap() {
                    MouseEvent::AbsoluteMove(x, y) => {
                        if !start_resize() {
                            continue;
                        }

                        let delta_x = (x - last_position.0) as f32;
                        let delta_y = (y - last_position.1) as f32;

                        if delta_x == 0.0 && delta_y == 0.0 {
                            continue;
                        }

                        let resize_direction = resize_direction().unwrap();

                        let mut new_window_size = window_size;
                        let mut new_window_position = window_position;

                        //包含上
                        if NORTH_DIRECTION.contains(&resize_direction) {
                            new_window_size.height = new_window_size.height - delta_y;
                            new_window_position.y = new_window_position.y + delta_y;
                        }

                        //包含下
                        if SOUTH_DIRECTION.contains(&resize_direction) {
                            new_window_size.height = new_window_size.height + delta_y;
                        }

                        //包含左
                        if WEST_DIRECTION.contains(&resize_direction) {
                            new_window_size.width = new_window_size.width - delta_x;
                            new_window_position.x = new_window_position.x + delta_x;
                        }

                        //包含右
                        if EAST_DIRECTION.contains(&resize_direction) {
                            new_window_size.width = new_window_size.width + delta_x;
                        }

                        if aspect_ratio().is_some() {
                            let aspect_ratio = aspect_ratio.unwrap();
                            if CORNER_DIRECTION.contains(&resize_direction) {
                                new_window_size.height = new_window_size.width / aspect_ratio;
                            } else if window_size.width != new_window_size.width {
                                new_window_size.height = new_window_size.width / aspect_ratio;
                            } else {
                                new_window_size.width = new_window_size.height * aspect_ratio;
                            }
                        }

                        platform.set_window_size_and_position(new_window_size, new_window_position);
                        if let Some(on_size_change) = on_size_change() {
                            on_size_change.call((new_window_size, window_size));
                        }

                        //更新鼠标位置
                        last_position.0 = x;
                        last_position.1 = y;
                        window_size = new_window_size;
                        window_position = new_window_position;
                    }
                    MouseEvent::Press(MouseButton::Left) => {
                        if resize_direction().is_some() {
                            last_position = mouse_manager.get_position().unwrap();
                            let info = platform.info();
                            window_size = info.window_size;
                            window_position = info.window_position;
                            start_resize.set(true);
                        }
                    }
                    MouseEvent::Release(MouseButton::Left) => {
                        start_resize.set(false);
                        resize_direction.set(None);
                    }
                    _ => {}
                };
            }
        });
    });

    rsx!(rect {
        onmousedown,
        onmouseover,
        onmouseleave,
        { props.children }
    })
}

fn cursor_resize_direction(
    win_size: Size2D,
    position: Point2D,
    border_size: f32,
) -> Option<ResizeDirection> {
    enum XDirection {
        West,
        East,
        Default,
    }

    enum YDirection {
        North,
        South,
        Default,
    }

    let xdir = if position.x < border_size {
        XDirection::West
    } else if position.x > (win_size.width - border_size) {
        XDirection::East
    } else {
        XDirection::Default
    };

    let ydir = if position.y < border_size {
        YDirection::North
    } else if position.y > (win_size.height - border_size) {
        YDirection::South
    } else {
        YDirection::Default
    };

    Some(match xdir {
        XDirection::West => match ydir {
            YDirection::North => ResizeDirection::NorthWest,
            YDirection::South => ResizeDirection::SouthWest,
            YDirection::Default => ResizeDirection::West,
        },

        XDirection::East => match ydir {
            YDirection::North => ResizeDirection::NorthEast,
            YDirection::South => ResizeDirection::SouthEast,
            YDirection::Default => ResizeDirection::East,
        },

        XDirection::Default => match ydir {
            YDirection::North => ResizeDirection::North,
            YDirection::South => ResizeDirection::South,
            YDirection::Default => return None,
        },
    })
}

fn get_cursor_icon(resize_direction: Option<ResizeDirection>) -> CursorIcon {
    if resize_direction.is_none() {
        CursorIcon::Default
    } else {
        match resize_direction.unwrap() {
            ResizeDirection::East => CursorIcon::EResize,
            ResizeDirection::North => CursorIcon::NResize,
            ResizeDirection::NorthEast => CursorIcon::NeResize,
            ResizeDirection::NorthWest => CursorIcon::NwResize,
            ResizeDirection::South => CursorIcon::SResize,
            ResizeDirection::SouthEast => CursorIcon::SeResize,
            ResizeDirection::SouthWest => CursorIcon::SwResize,
            ResizeDirection::West => CursorIcon::WResize,
        }
    }
}
