#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::collections::HashMap;

#[tauri::command]
fn list_files(dir: &str) -> Vec<String> {
    use std::fs;

    let mut items: Vec<String> = vec![];
    let paths = fs::read_dir(dir).unwrap();

    for path in paths {
        items.push(path.unwrap().path().display().to_string());
    }

    items
}

#[tauri::command]
fn get_config() -> HashMap<String, String> {
    // TODO: Use a proper struct
    // TODO: Read from disk
    let mut config: HashMap<String, String> = HashMap::new();
    config.insert("name".to_string(), "Dan".to_string());
    config.insert("age".to_string(), "25".to_string());

    config
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![list_files, get_config,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
