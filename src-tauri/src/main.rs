// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{AppHandle, Manager, Result as TauriResult, RunEvent};

use crate::{
    server::{start_websocket_server, WebSocketServerState},
    volume_control::VolumeCommandSender,
};

mod commands;
mod server;
mod types;
mod volume_control;

fn main() {
    if let Err(e) = start_application() {
        eprintln!("Failed to start application: {}", e);
        std::process::exit(1);
    }
}

pub fn start_application() -> TauriResult<()> {
    let app = create_tauri_app()?;

    setup_signal_handlers(&app).expect("Failed to set Ctrl-C handler");
    run_application(app);

    Ok(())
}

fn create_tauri_app() -> TauriResult<tauri::App> {
    tauri::Builder::default()
        .plugin(tauri_plugin_websocket::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .manage(volume_control::spawn_volume_thread())
        .manage(WebSocketServerState::new())
        .setup(|app| {
            let _ = setup_dev_tools(app);

            let app_handle = app.app_handle().clone();

            tauri::async_runtime::spawn(async move {
                match start_websocket_server(9001, app_handle).await {
                    Ok(addr) => println!("WebSocket server listening on {}", addr),
                    Err(e) => eprintln!("WebSocket server failed to start: {}", e),
                }
            });
            Ok(())
        })
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

fn setup_dev_tools(_app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    {
        for window_config in &_app.config().app.windows {
            if window_config.devtools.unwrap_or(false) {
                if let Some(window) = _app.get_webview_window(&window_config.label) {
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
        app_handle.exit(0);
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
    if let Err(e) = app_handle.state::<VolumeCommandSender>().shutdown() {
        eprintln!("Volume thread shutdown error: {}", e);
    }

    let ws_state = app_handle.state::<WebSocketServerState>();
    tauri::async_runtime::block_on(async {
        if let Err(err) = ws_state.shutdown().await {
            eprintln!("WebSocket server shutdown error: {}", err);
        }
    });
}
