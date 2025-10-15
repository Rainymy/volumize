// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::str::FromStr;

use tauri::{
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, Result as TauriResult, RunEvent,
};

mod commands;
mod server;
mod storage;
mod tray;
mod types;

use server::{
    start_service_register, start_websocket_server, ServiceDiscovery, VolumeCommandSender,
    WebSocketServerState,
};

pub use tray::Discovery;

fn main() {
    if let Err(e) = start_application() {
        eprintln!("Failed to start application: {}", e);
        std::process::exit(1);
    }
}

pub fn start_application() -> TauriResult<()> {
    let app = create_tauri_app()?;

    #[cfg(debug_assertions)]
    {
        let app_handle = app.handle().clone();

        let _ = ctrlc::set_handler(move || {
            println!("CTRL-C received, initiating clean shutdown...");
            shutdown_background_thread(&app_handle);
            app_handle.exit(0);
        })
        .inspect_err(|err| eprintln!("Failed to set Ctrl-C handler: {}", err));
    }

    app.run(move |app_handle, event| {
        if let RunEvent::ExitRequested { .. } = event {
            shutdown_background_thread(app_handle);
        }
    });

    Ok(())
}

#[tauri::command]
async fn discover_server_address() -> Option<String> {
    server::discover_server().await.ok()
}

fn create_tauri_app() -> TauriResult<tauri::App> {
    tauri::Builder::default()
        .plugin(tauri_plugin_websocket::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .manage(server::spawn_volume_thread())
        .manage(WebSocketServerState::new())
        .manage(ServiceDiscovery::new())
        .manage(tray::ClickState::new(None))
        .manage(storage::Storage::default())
        .setup(|app| {
            setup_dev_tools(app);

            let app_handle = app.handle();
            let storage = app_handle.state::<storage::Storage>();
            storage.load_settings(app_handle);
            let settings = storage.get_settings();

            dbg!(&settings);

            match start_websocket_server(settings.port_address, app_handle) {
                Ok(addr) => println!("WebSocket server listening on {}", addr),
                Err(e) => eprintln!("\nWebSocket server failed to start: \n\t{}\n", e),
            }

            start_service_register(settings.port_address, app_handle, settings.dutaion);
            setup_tray_system(app)?;

            Ok(())
        })
        .on_menu_event(|app, event| match event.id().as_ref() {
            "show" => show_window_visibility(app),
            rest => {
                let discover = match Discovery::from_str(rest) {
                    Ok(value) => value,
                    Err(_) => return,
                };

                let storage = app.app_handle().state::<storage::Storage>();
                let mut settings = storage.get_settings();
                settings.dutaion = discover;

                if let Err(err) = storage.save_settings(app, &settings) {
                    eprintln!("{}", err);
                }

                start_service_register(settings.port_address, app.app_handle(), discover);
            }
        })
        .on_window_event(|_window, _event| {
            let storage = _window.app_handle().state::<storage::Storage>();
            let should_exit_to_tray = storage.get_settings().exit_to_tray;

            if should_exit_to_tray {
                // // Turn off exit to tray functionality to test other features.
                // if let tauri::WindowEvent::CloseRequested { api, .. } = _event {
                //     let _ = _window.hide();
                //     api.prevent_close();
                // }
            }
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
            discover_server_address
        ])
        .build(tauri::generate_context!())
}

fn setup_tray_system(app: &mut tauri::App) -> TauriResult<()> {
    let icon = app
        .default_window_icon()
        .expect("Application should have a default window icon configured")
        .clone();

    let tray_menu = tray::create_tray(app.handle())?;
    let _tray = TrayIconBuilder::new()
        .menu(&tray_menu)
        .tooltip("Volumize")
        .show_menu_on_left_click(false)
        .icon(icon)
        .on_tray_icon_event(|tray, event| {
            let click_state = tray.app_handle().state::<tray::ClickState>();
            match event {
                TrayIconEvent::Click {
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
                    ..
                } => {
                    if click_state.is_double_click() {
                        show_window_visibility(tray.app_handle());
                    }
                }
                _ => {}
            }
        })
        .build(app)?;

    Ok(())
}

fn setup_dev_tools(_app: &mut tauri::App) {
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
}

fn show_window_visibility(app: &tauri::AppHandle) {
    let window = match app.get_webview_window("main") {
        Some(window) => window,
        None => return,
    };
    let is_visible = window.is_visible().unwrap_or(false);
    let is_minimized = window.is_minimized().unwrap_or(false);

    match (is_visible, is_minimized) {
        (true, true) => {
            // Window is minimized, restore it
            let _ = window.unminimize();
            let _ = window.set_focus();
        }
        (true, false) => {}
        (false, _) => {
            // Window is hidden, show it
            let _ = window.show();
            let _ = window.set_focus();
        }
    }
}

fn _hide_window_visibility(app: &tauri::AppHandle) {
    let window = match app.get_webview_window("main") {
        Some(window) => window,
        None => return,
    };
    let is_visible = window.is_visible().unwrap_or(false);
    let is_minimized = window.is_minimized().unwrap_or(false);

    match (is_visible, is_minimized) {
        (true, true) => {}
        (true, false) => {
            // Window is visible and not minimized, hide it
            let _ = window.hide();
        }
        (false, _) => {}
    }
}

fn shutdown_background_thread(app_handle: &AppHandle) {
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
