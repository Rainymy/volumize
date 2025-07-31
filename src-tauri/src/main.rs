// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

fn main() {
    run();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let volume_thread = volumize_lib::spawn_volume_thread();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(volume_thread)
        .invoke_handler(tauri::generate_handler![
            // Master
            commands::set_master_volume,
            commands::get_master_volume,
            //     MuteMaster,
            //     UnmuteMaster,
            // Application
            //     GetAllApplications,
            //     GetAppVolume,
            //     SetAppVolume,
            //     MuteApp,
            //     UnmuteApp,
            // DeviceControl
            //     GetCurrentPlaybackDevice,
            //     GetPlaybackDevices,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
