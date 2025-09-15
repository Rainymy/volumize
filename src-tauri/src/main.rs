// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use tauri::{AppHandle, Manager, Result as TauriResult, RunEvent};

use crate::volume_control::VolumeCommandSender;

mod commands;
mod types;
mod volume_control;

fn main() {
    if let Err(e) = start_application() {
        eprintln!("Failed to start application: {}", e);
        std::process::exit(1);
    }
}

pub fn start_application() -> TauriResult<()> {
    let volume_thread = volume_control::spawn_volume_thread();

    let app = create_tauri_app(volume_thread)?;

    setup_signal_handlers(&app).expect("Failed to set Ctrl-C handler");
    run_application(app);

    Ok(())
}

fn create_tauri_app(volume_thread: VolumeCommandSender) -> TauriResult<tauri::App> {
    tauri::Builder::default()
        .setup(setup_dev_tools)
        .plugin(tauri_plugin_opener::init())
        .manage(volume_thread)
        .invoke_handler(tauri::generate_handler![
            // Master volume controls
            commands::set_device_volume,
            commands::get_device_volume,
            commands::unmute_device,
            commands::mute_device,
            // Application volume controls
            commands::get_all_applications,
            commands::get_app_volume,
            commands::set_app_volume,
            commands::mute_app_volume,
            commands::unmute_app_volume,
            // Device controls
            commands::get_current_playback_device,
            commands::get_playback_devices,
        ])
        .build(tauri::generate_context!())
}

fn setup_dev_tools(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
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
}

fn setup_signal_handlers(app: &tauri::App) -> Result<(), ctrlc::Error> {
    let app_handle = app.app_handle().clone();

    ctrlc::set_handler(move || {
        println!("CTRL-C received, initiating clean shutdown...");
        shutdown_background_thread(&app_handle);
        std::process::exit(0); // important to close, appilcation will stay open
    })
}

fn run_application(app: tauri::App) {
    app.run_return(move |app_handle, event| {
        if let RunEvent::ExitRequested { .. } = event {
            shutdown_background_thread(app_handle);
        }
    });
}

fn shutdown_background_thread(app_handle: &AppHandle) {
    println!("Shutting down background thread...");

    let state = app_handle.state::<VolumeCommandSender>();
    let handle_clone = Arc::clone(&state.thread_handle);

    state.close_channel(); // close the channel before joining.

    match handle_clone.lock() {
        Ok(mut handle_guard) => {
            if let Some(join_handle) = handle_guard.take() {
                if let Err(e) = join_handle.join() {
                    eprintln!("Background thread panicked during shutdown: {:?}", e)
                }
            }
        }
        Err(e) => eprintln!("Failed to acquire thread handle lock: {:?}", e),
    };
}
