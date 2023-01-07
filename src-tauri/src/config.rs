use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    name: String,
    age: i32,
    initialised: bool,
}

impl AppConfig {
    fn default() -> AppConfig {
        AppConfig {
            name: "".to_string(),
            age: 0,
            initialised: false,
        }
    }

    fn from_disk() -> Option<AppConfig> {
        use std::fs::read_to_string;
        use tauri::api::path::config_dir;

        if let Some(mut config_file) = config_dir() {
            config_file.push("plexman.json");

            if let Ok(contents) = read_to_string(config_file) {
                let parsed: Result<AppConfig, _> = serde_json::from_str(&contents);

                if let Ok(parsed) = parsed {
                    return Some(parsed);
                }
            }
        }

        None
    }

    fn to_disk(self) -> Result<bool, String> {
        use std::fs;
        use tauri::api::path::config_dir;

        if let Some(mut config_file) = config_dir() {
            config_file.push("plexman.json");
            if let Ok(json_string) = serde_json::to_string(&self) {
                if let Ok(_) = fs::File::create(&config_file) {
                    if let Ok(_) = fs::write(&config_file, json_string) {
                        return Ok(true);
                    }
                }
            }
        }

        Err("Unknown error while writing config file".to_string())
    }
}

#[tauri::command]
pub fn get_config() -> AppConfig {
    if let Some(config) = AppConfig::from_disk() {
        return config;
    }

    return AppConfig::default();
}

#[tauri::command]
pub fn write_config(config: AppConfig) -> Result<bool, String> {
    println!("{:?}", config);
    return config.to_disk();
}
