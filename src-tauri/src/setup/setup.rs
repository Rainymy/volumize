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

    #[cfg(debug_assertions)]
    {
        show_window_visibility(app_handle);
        setup_dev_tools(app_handle);
    }

    setup_tray_system(app_handle)?;

    let storage = app_handle.state::<Storage>();
    storage.load(app_handle);
    let settings = storage.get();

    let (tx, rx) = std::sync::mpsc::channel::<UpdateChange>();
    spawn_volume_thread(app_handle, tx); // Thread for volume control
    spawn_update_thread(app_handle, rx); // Thread for propagate updates to the UI

    match start_websocket_server(settings.port_address, app_handle) {
        Ok(addr) => println!("WebSocket server listening on {}", addr),
        Err(e) => eprintln!("Failed to start WebSocket server: {}", e),
    }

    start_service_register(settings.port_address, app_handle, settings.duration);

    Ok(())
}

pub fn setup_tray_system(app: &tauri::AppHandle) -> Result<(), Box<dyn Error>> {
    use std::io::Error;
    use tauri::tray::TrayIconBuilder;

    let app_config = app.config();
    let tray_config = app_config.app.tray_icon.clone().unwrap_or_default();

    let tray_icon_path = tray_config.icon_path.clone();
    let tray_id = tray_config.id.unwrap_or("tray_icon_id".into()).clone();

    let tray_icon_builder = app
        .tray_by_id(&tray_id)
        .or_else(|| TrayIconBuilder::with_id(&tray_id).build(app).ok())
        .ok_or_else(|| Box::new(Error::other("Failed to build TrayIconBuilder")))?;

    let _ = tray_icon_builder.set_menu(super::system_tray::create_tray(app).ok());

    if let Ok(false) = tray_icon_path.try_exists() {
        let icon = app
            .default_window_icon()
            .expect("Application should have a default window icon configured")
            .clone();
        let _ = tray_icon_builder.set_icon(Some(icon));
    }

    tray_icon_builder.on_tray_icon_event(|tray, event| {
        use crate::types::click::ClickState;
        use tauri::tray::{MouseButton, MouseButtonState, TrayIconEvent};

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
    });

    Ok(())
}

fn setup_dev_tools(app: &tauri::AppHandle) {
    for window_config in &app.config().app.windows {
        if let Some(window) = app.get_webview_window(&window_config.label) {
            window.open_devtools();
        }
    }
}

pub fn show_window_visibility(app: &tauri::AppHandle) {
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
