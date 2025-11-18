// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod platform;
mod server;
mod setup;
mod types;

use tauri::{AppHandle, Manager};

use crate::{
    server::{ServiceDiscovery, WebSocketServerState},
    types::volume::VolumeCommandSender,
};

/// Entry point for the desktop application.
/// - **NEVER** let the `lib.rs` code to touch any desktop functionality.
/// - It is breaking the android build process.
///
/// This took me 8 hours to figure out.
fn main() {
    let app = match setup::create_tauri_app() {
        Ok(app) => app,
        Err(e) => {
            eprintln!("Failed to create Tauri app: {}", e);
            return;
        }
    };

    #[cfg(debug_assertions)]
    {
        let app_handle = app.handle().clone();

        let _ = ctrlc::set_handler(move || {
            println!("CTRL-C received, initiating clean shutdown...");
            shutdown_background_threads(&app_handle);
            app_handle.cleanup_before_exit();
            app_handle.exit(0);
        })
        .inspect_err(|err| eprintln!("Failed to set Ctrl-C handler: {}", err));
    }

    app.run(move |app_handle, event| {
        if let tauri::RunEvent::ExitRequested { .. } = event {
            shutdown_background_threads(app_handle);
        }
    });
}

fn shutdown_background_threads(app_handle: &AppHandle) {
    if let Err(e) = app_handle.state::<VolumeCommandSender>().shutdown() {
        eprintln!("Volume thread shutdown error: {}", e);
    }

    let service_state = app_handle.state::<ServiceDiscovery>();
    tauri::async_runtime::block_on(async {
        if let Err(e) = service_state.shutdown().await {
            eprintln!("Service thread shutdown error: {}", e);
        }
    });

    let ws_state = app_handle.state::<WebSocketServerState>();
    tauri::async_runtime::block_on(async {
        if let Err(err) = ws_state.shutdown().await {
            eprintln!("WebSocket server shutdown error: {}", err);
        }
    });
}
