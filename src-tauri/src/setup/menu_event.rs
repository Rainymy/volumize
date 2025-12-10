use std::str::FromStr;

use tauri::{menu::MenuEvent, AppHandle, Manager};
use tauri_plugin_autostart::ManagerExt;

use crate::{
    server::service_register::start_service_register,
    types::{storage::Storage, tray::Discovery},
};

pub fn menu_event(app: &AppHandle, event: MenuEvent) {
    match event.id().as_ref() {
        "show" => super::setup::show_window_visibility(app),
        "refresh" => {
            let storage = app.app_handle().state::<Storage>();
            let settings = storage.get();

            start_service_register(settings.port_address, app, settings.duration);
        }
        "auto_start" => {
            let manager = app.autolaunch();

            let storage = app.app_handle().state::<Storage>();
            let mut settings = storage.get();

            let currently_enabled = manager.is_enabled().unwrap_or_default();
            settings.autostart = !currently_enabled;

            if let Err(err) = storage.save(app, &settings) {
                eprintln!("{}", err);
            }

            if currently_enabled {
                let _ = manager.disable();
            } else {
                let _ = manager.enable();
            }

            if let Err(e) = super::setup::setup_tray_system(&app) {
                eprintln!("{}", e);
            }
        }
        rest => {
            let discover = match Discovery::from_str(rest) {
                Ok(value) => value,
                Err(_) => return,
            };

            let sould_save = match discover {
                Discovery::OnDuration(_) => false,
                _ => true,
            };

            let storage = app.app_handle().state::<Storage>();
            let mut settings = storage.get();

            settings.duration = discover;

            if sould_save {
                if let Err(err) = storage.save(app, &settings) {
                    eprintln!("{}", err);
                }
            }

            storage.update(settings);
            start_service_register(settings.port_address, app, discover);

            if let Err(e) = super::setup::setup_tray_system(&app) {
                eprintln!("{}", e);
            }
        }
    }
}
