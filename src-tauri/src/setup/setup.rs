use std::error::Error;

use crate::{
    server::{
        service_register::start_service_register,
        start_websocket_server,
        volume_control::{spawn_update_thread, spawn_volume_thread},
    },
    types::{shared::UpdateChange, storage::Storage},
};

use tauri::{App, Manager};

pub fn setup(app: &mut App) -> Result<(), Box<dyn Error>> {
    let app_handle = app.handle();

    setup_dev_tools(app_handle);
    setup_tray_system(app_handle)?;

    let storage = app_handle.state::<Storage>();
    storage.load(app_handle);
    let settings = storage.get();

    // let (tx, mut rx) = unbounded_channel();
    let (tx, rx) = std::sync::mpsc::channel::<UpdateChange>();
    spawn_volume_thread(app_handle, tx); // Thread for volume control
    spawn_update_thread(app_handle, rx); // Thread for propagate updates to the UI

    let addr = start_websocket_server(settings.port_address, app_handle);
    println!("WebSocket server listening on {}", addr);

    start_service_register(settings.port_address, app_handle, settings.duration);

    Ok(())
}

pub fn setup_tray_system(app: &tauri::AppHandle) -> Result<(), Box<dyn Error>> {
    use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};

    let storage = app.state::<Storage>();

    let icon = app
        .default_window_icon()
        .expect("Application should have a default window icon configured")
        .clone();

    let tray_menu = super::system_tray::create_tray(app)?;
    let tray = TrayIconBuilder::new()
        .menu(&tray_menu)
        .tooltip("Volumize")
        .show_menu_on_left_click(false)
        .icon(icon)
        .on_tray_icon_event(|tray, event| {
            use crate::types::click::ClickState;

            let click_state = tray.app_handle().state::<ClickState>();
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

    let mut icon_id = match storage.tray_icon_id.lock() {
        Ok(icon_id) => icon_id,
        Err(e) => Err(format!("Failed to lock tray icon ID: {}", e))?,
    };

    match icon_id.replace(tray.id().as_ref().to_string()) {
        Some(old_id) => app.remove_tray_by_id(&old_id),
        None => None,
    };

    Ok(())
}

fn setup_dev_tools(_app: &tauri::AppHandle) {
    #[cfg(debug_assertions)]
    {
        for window_config in &_app.config().app.windows {
            if window_config.devtools.unwrap_or(false) {
                use tauri::Manager;

                if let Some(window) = _app.get_webview_window(&window_config.label) {
                    window.open_devtools();
                }
            }
        }
    }
}

pub fn show_window_visibility(_app: &tauri::AppHandle) {
    #[cfg(desktop)]
    {
        let window = match _app.get_webview_window("main") {
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
}

fn _hide_window_visibility(_app: &tauri::AppHandle) {
    #[cfg(desktop)]
    {
        let window = match _app.get_webview_window("main") {
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
}
