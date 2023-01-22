#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod config;
mod error;
mod io;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            config::get_config,
            config::write_config,
            config::delete_config,
            io::list_files,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
