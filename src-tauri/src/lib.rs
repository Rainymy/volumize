#![allow(dead_code)]
mod commands;
mod platform;
mod server;
mod types;

/// Entry point for the android/ios application.
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn start_application() {
    tauri::Builder::default()
        .plugin(tauri_plugin_websocket::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![commands::discover_server_address])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
