use chrono::{Local, Timelike};
use freya::prelude::*;
use std::time::Duration;
use tokio::time::sleep;

pub fn use_current_time() -> (Signal<u32>, Signal<u32>, Signal<u32>) {
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

    (hour, minute, second)
}
