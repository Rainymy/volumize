// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod types;
mod volume_control;

fn main() {
    start_application();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn start_application() {
    let volume_thread = volume_control::spawn_volume_thread();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(volume_thread)
        .invoke_handler(tauri::generate_handler![
            // Master
            commands::set_master_volume,
            commands::get_master_volume,
            commands::unmute_master,
            commands::mute_master,
            // Application
            commands::get_all_applications,
            commands::get_app_volume,
            commands::set_app_volume,
            commands::mute_app_volume,
            commands::unmute_app_volume,
            // DeviceControl
            commands::get_current_playback_device,
            commands::get_playback_devices,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
