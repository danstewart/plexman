use crate::error::{to_err, ErrorInfo};
use anyhow::{anyhow, Result};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use serde_json;

/// Return the path to the application configuration file
fn get_config_file_path() -> Result<PathBuf> {
    use tauri::api::path::config_dir;

    if let Some(mut config_file) = config_dir() {
        config_file.push("plexman");
        std::fs::create_dir_all(&config_file)?;

        config_file.push("plexman.json");
        return Ok(config_file);
    }

    Err(anyhow!("Could not get path to config file"))
}

/// An object detailing where media files can be found
#[derive(Serialize, Deserialize, Debug)]
pub struct MediaSource {
    path: String,
}

/// An object detailing where a plex library is
#[derive(Serialize, Deserialize, Debug)]
pub struct LibraryTarget {
    path: String,
    is_local: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    source: Option<MediaSource>,
    targets: Vec<LibraryTarget>,
}

impl AppConfig {
    fn default() -> AppConfig {
        AppConfig {
            source: None,
            targets: vec![],
        }
    }

    fn from_disk() -> Option<AppConfig> {
        use std::fs::read_to_string;

        if let Ok(config_file) = get_config_file_path() {
            if let Ok(contents) = read_to_string(config_file) {
                let parsed: Result<AppConfig, _> = serde_json::from_str(&contents);

                if let Ok(parsed) = parsed {
                    return Some(parsed);
                }
            }
        }

        None
    }

    fn to_disk(self) -> Result<bool> {
        use std::fs;

        let config_file = get_config_file_path()?;

        let json_string = serde_json::to_string(&self)?;
        fs::File::create(&config_file)?;
        fs::write(&config_file, json_string)?;

        Ok(true)
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
pub fn write_config(config: AppConfig) -> Result<bool, ErrorInfo> {
    let res = config.to_disk();

    if let Err(error) = res {
        return Err(to_err(error));
    }

    Ok(true)
}

#[tauri::command]
pub fn delete_config() -> Result<bool, ErrorInfo> {
    use std::fs;

    match get_config_file_path() {
        Ok(path) => match fs::remove_file(path) {
            Ok(_) => return Ok(true),
            Err(error) => return Err(to_err(anyhow!(error))),
        },
        Err(error) => return Err(to_err(error)),
    }
}
