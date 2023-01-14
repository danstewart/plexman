#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod config;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            config::get_config,
            config::write_config,
            config::delete_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
