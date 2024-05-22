use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use tokio::io::AsyncWriteExt;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct AppConfig {
    pub dot_color: String,
    pub card_color: String,
    pub font_color: String,
    pub size: f64,
    pub x: i32,
    pub y: i32,
    pub lock: bool,
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
