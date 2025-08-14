// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use tauri::{Manager, WindowEvent::CloseRequested};

use crate::volume_control::{VolumeCommand, VolumeCommandSender};

mod commands;
mod types;
mod volume_control;

fn main() {
    start_application();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn start_application() {
    let volume_thread = volume_control::spawn_volume_thread();
    let thread_handle = Arc::clone(&volume_thread.thread_handle);

    tauri::Builder::default()
        .setup(|app| {
            // It uses the windows devtools parameter in tauri.conf.json
            // to indicate if the DevTools should open for that window or not.
            #[cfg(debug_assertions)]
            {
                for window_config in &app.config().app.windows {
                    if window_config.devtools.unwrap_or(false) {
                        if let Some(window) = app.get_webview_window(&window_config.label) {
                            window.open_devtools();
                        }
                    }
                }
            }
            Ok(())
        })
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
        .on_window_event(move |_window, event| {
            if let CloseRequested { .. } = event {
                println!("App closing — sending shutdown signal");

                let state = _window.app_handle().state::<VolumeCommandSender>();

                // Send close thread signal.
                let _ = state.tx.send(VolumeCommand::CloseThread);

                // Join the thread into main.
                if let Ok(mut handle_opt) = thread_handle.lock() {
                    if let Some(handle) = handle_opt.take() {
                        let _ = handle.join();
                    }
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
