#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::alloc::System;
// use std::sync::mpsc::channel;
// use std::thread;
use freya::common::EventMessage;
// use freya::events::PointerType::Mouse;

#[global_allocator]
static A: System = System;

use freya::prelude::*;
use tray_icon::{TrayIconBuilder, menu::{Menu, MenuItem, MenuEvent}, TrayIconEvent, ClickType};
// use tokio::sync::watch::channel;
use tokio::sync::broadcast::channel;

use mouce::Mouse;

fn main() {
  // let path = concat!(env!("CARGO_MANIFEST_DIR"), "./icon.png");
  // let icon = load_icon(std::path::Path::new(path));
  // let menu = Menu::new();
  // let quit_i = MenuItem::new("Quit", true, None);
  // let quit_id = quit_i.id().clone();
  // menu.append(&quit_i).unwrap();
  // let _tray_icon = TrayIconBuilder::new()
  //   // .with_menu(Box::new(menu))
  //   .with_tooltip("system-tray - tray icon library!")
  //   .with_icon(icon)
  //   .build()
  //   .unwrap();
  // std::thread::spawn(move || {
  //   let menu_channel = MenuEvent::receiver();
  //   loop {
  //     if let Ok(event) = menu_channel.try_recv() {
  //       if event.id == quit_id {
  //         // std::process::exit(1);
  //         println!("do exits");
  //       }
  //     }
  //   }
  // });

  launch_cfg(
    app,
    LaunchConfig::<()>::builder()
      .with_width(400.0)
      .with_height(200.0)
      // .with_decorations(false)
      .with_transparency(true)
      .with_title("Floating window")
      .with_background("transparent")
      .build(),
  );
}

fn app() -> Element {
  let mut state = use_signal(|| 0);

  println!("re do app");
  // let platform = use_platform();

  use_effect(move || {
    // let mut ticker = platform.new_ticker();
    // spawn(async move {
    //   loop {
    //     ticker.tick().await;
    //     println!("next tick");
    //     for e in tray_icon::TrayIconEvent::receiver().try_iter() {
    //       println!("rec data");
    //       // state += 1;
    //     }
    //   }
    // });
    let (tx, mut rx) = channel::<mouce::common::MouseEvent>(100);
    // let icon_tx = tx.clone();
    // TrayIconEvent::set_event_handler(Some(move |event: TrayIconEvent| {
    //   let data = match event.click_type {
    //     ClickType::Left => { 1 }
    //     ClickType::Right => { 2 }
    //     ClickType::Double => { 3 }
    //   };
    //   icon_tx.send(data).ok();
    //   // icon_sender.send(1).ok();
    // }));
    // let menu_icon = tx.clone();
    // MenuEvent::set_event_handler(Some(move |eve| {
    //   menu_icon.send(4).ok();
    // }));
    // let platform = use_platform();

    let mut mouse_manager = Mouse::new();

    mouse_manager.hook(Box::new(move |e| {
      println!("mouse event: {e:?}");
      tx.send(e.clone()).ok();
    })).ok();

    spawn(async move {
      loop {
        let data = rx.recv().await.ok().unwrap();
        println!("get data on there {data:?}");
        // platform.send(EventMessage::RequestRerender).ok();
        if let mouce::common::MouseEvent::Press(_) = data {
          state += 1;
        }
      }
    });
  });

  rsx!(
    WindowDragArea {
        rect {
            background: "red",
            onclick: move |_|{ state += 1 },
            padding: "10",
            main_align: "center",
            cross_align: "center",
            width: "80%",
            height: "80%",
            corner_radius: "15",
            label {
                color: "black",
                "A frameless window{state}"
            }
            label {
                color: "black",
                "A frameless window"
            }
        }
    }
  )
}

fn load_icon(path: &std::path::Path) -> tray_icon::Icon {
  let (icon_rgba, icon_width, icon_height) = {
    let image = image::open(path)
      .expect("Failed to open icon path")
      .into_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();
    (rgba, width, height)
  };
  tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}