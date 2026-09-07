use tauri::{
    menu::{CheckMenuItemBuilder, Menu, MenuItem, PredefinedMenuItem, Submenu, SubmenuBuilder},
    Manager, Result as TauriResult, Wry,
};

use crate::{
    server::{serial::SerialState, serialport::find_devices},
    setup::get_main_window,
    types::storage::Storage,
    types::tray::Discovery,
};

pub fn create_tray(handle: &tauri::AppHandle) -> TauriResult<Menu<Wry>> {
    let is_minimized =
        get_main_window(handle).map_or(false, |w| w.is_minimized().unwrap_or_default());

    let show = MenuItem::with_id(handle, "show", "Show", true, None::<&str>)?;
    let refresh_token = MenuItem::with_id(handle, "refresh", "Quick refresh", true, None::<&str>)?;
    let system_info = MenuItem::new(handle, "System info", false, None::<&str>)?;
    let communication = MenuItem::new(handle, "Communication", false, None::<&str>)?;

    let separator = PredefinedMenuItem::separator(handle)?;
    let quit = PredefinedMenuItem::quit(handle, Some("Quit"))?;

    let tray_menu = Menu::new(handle)?;
    let _ = tray_menu.append(&app_version(handle)?);
    {
        // Simple action
        let _ = tray_menu.append(&separator);
        if is_minimized {
            let _ = tray_menu.append(&show);
        }
        let _ = tray_menu.append(&refresh_token);
        let _ = tray_menu.append(&separator);
    }
    {
        // Serial and discovery
        let _ = tray_menu.append(&communication);
        let _ = tray_menu.append(&separator);
        let _ = tray_menu.append(&serial_sub_menu(handle)?);
        let _ = tray_menu.append(&discovery_sub_menu(handle)?);
        let _ = tray_menu.append(&separator);
    }
    {
        // System configurations
        let _ = tray_menu.append(&system_info);
        let _ = tray_menu.append(&separator);
        let _ = tray_menu.append(&exit_to_tray_menu(handle)?);
        let _ = tray_menu.append(&auto_start_sub_menu(handle)?);
        let _ = tray_menu.append(&separator);
    }
    let _ = tray_menu.append(&quit);

    Ok(tray_menu)
}

fn auto_start_sub_menu(handle: &tauri::AppHandle) -> tauri::Result<Submenu<Wry>> {
    use tauri_plugin_autostart::ManagerExt;
    let is_auto_start_enabled = handle.autolaunch().is_enabled().unwrap_or(false);
    let is_enabled_text = if is_auto_start_enabled {
        "Enabled"
    } else {
        "Disabled"
    };
    let is_enabled_inverted_text = if is_auto_start_enabled {
        "Disable"
    } else {
        "Enable"
    };
    let status_info = MenuItem::new(
        handle,
        format!("Status: {}", is_enabled_text),
        false,
        None::<&str>,
    )?;

    let auto_start_toggle = MenuItem::with_id(
        handle,
        "auto_start",
        format!("{}", is_enabled_inverted_text),
        true,
        None::<&str>,
    )?;

    SubmenuBuilder::new(handle, "Autostart")
        .item(&status_info)
        .item(&PredefinedMenuItem::separator(handle)?)
        .item(&auto_start_toggle)
        .build()
}

fn exit_to_tray_menu(handle: &tauri::AppHandle) -> tauri::Result<Submenu<Wry>> {
    let settings = handle.state::<Storage>().get();
    let exit_to_tray = settings.exit_to_tray;

    let status_info = MenuItem::new(
        handle,
        format!(
            "Status: {}",
            if exit_to_tray { "Enabled" } else { "Disabled" }
        ),
        false,
        None::<&str>,
    )?;

    let tray_main = MenuItem::with_id(
        handle,
        "exit_to_tray",
        if exit_to_tray { "Disable" } else { "Enable" },
        true,
        None::<&str>,
    )?;

    SubmenuBuilder::new(handle, "Minimize to tray")
        .item(&status_info)
        .item(&PredefinedMenuItem::separator(handle)?)
        .item(&tray_main)
        .build()
}

fn app_version(handle: &tauri::AppHandle) -> tauri::Result<MenuItem<Wry>> {
    let package_info = handle.package_info();

    let version = package_info.version.to_string();
    let name = package_info.name.clone();

    MenuItem::with_id(
        handle,
        "version",
        &format!("{name} v{version}"),
        false,
        None::<&str>,
    )
}

fn serial_sub_menu(handle: &tauri::AppHandle) -> tauri::Result<Submenu<Wry>> {
    let devices = find_devices();

    let serial_state = handle.state::<SerialState>();
    let serial_port = match serial_state.serial_port.clone() {
        Some(port) => port,
        None => "<None>".to_string(),
    };

    let current_info = format!("Serial: {}", serial_port);
    let status_info = MenuItem::new(handle, current_info, false, None::<&str>)?;

    let mut submenu = SubmenuBuilder::new(handle, "Serial ports")
        .item(&status_info)
        .item(&PredefinedMenuItem::separator(handle)?);

    for device in &devices {
        submenu = submenu.item(&MenuItem::with_id(
            handle,
            &format!("s_{}", device.name),
            device.name.clone(),
            true,
            None::<&str>,
        )?);
    }

    submenu.build()
}

fn discovery_sub_menu(handle: &tauri::AppHandle) -> tauri::Result<Submenu<Wry>> {
    let settings = handle.state::<Storage>().get();

    let info_text = format!("Status: {}", settings.duration.display());
    let status_info = MenuItem::with_id(handle, "show", info_text, false, None::<&str>)?;

    let always_off = checked_menu_item(Discovery::TurnOff, settings.duration).build(handle)?;
    let always_on = checked_menu_item(Discovery::AlwaysOn, settings.duration).build(handle)?;

    SubmenuBuilder::new(handle, "Server discovery")
        .item(&status_info)
        .item(&PredefinedMenuItem::separator(handle)?)
        .item(&always_on)
        .item(&timer_submenu(15, settings.duration).build(handle)?)
        .item(&timer_submenu(5, settings.duration).build(handle)?)
        .item(&timer_submenu(2, settings.duration).build(handle)?)
        .item(&always_off)
        .build()
}

fn checked_menu_item(item: Discovery, settings: Discovery) -> CheckMenuItemBuilder {
    CheckMenuItemBuilder::with_id(Discovery::to_string(&item), Discovery::display(&item))
        .checked(settings == item)
}

fn timer_submenu(timer_secs: u32, discovery: Discovery) -> CheckMenuItemBuilder {
    use std::time::Duration;

    let duration = Duration::from_secs(u64::from(timer_secs) * 60);
    let id = Discovery::OnDuration(duration).to_string();
    let text = format!("On for {} minute", timer_secs);

    CheckMenuItemBuilder::with_id(id, text).checked(discovery == Discovery::OnDuration(duration))
}
