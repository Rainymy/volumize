use std::time::Duration;

use tauri::{
    menu::{CheckMenuItemBuilder, Menu, MenuItem, PredefinedMenuItem, Submenu, SubmenuBuilder},
    {Manager, Result as TauriResult, Wry},
};
use tauri_plugin_autostart::ManagerExt;

use crate::types::storage::Storage;
use crate::types::tray::Discovery;

pub fn create_tray(handle: &tauri::AppHandle) -> TauriResult<Menu<Wry>> {
    let show = MenuItem::with_id(handle, "show", "Show", true, None::<&str>)?;
    let refresh_token = MenuItem::with_id(handle, "refresh", "Refresh menu", true, None::<&str>)?;

    let separator = PredefinedMenuItem::separator(handle)?;
    let quit = PredefinedMenuItem::quit(handle, Some("Exit"))?;

    let tray_menu = Menu::new(handle)?;
    let _ = tray_menu.append(&show);
    let _ = tray_menu.append(&refresh_token);
    let _ = tray_menu.append(&separator);
    let _ = tray_menu.append(&auto_start_sub_menu(handle)?); // Sub-menu
    let _ = tray_menu.append(&separator)?;
    let _ = tray_menu.append(&discovery_sub_menu(handle)?); // Sub-menu
    let _ = tray_menu.append(&separator);
    let _ = tray_menu.append(&quit);

    Ok(tray_menu)
}

fn auto_start_sub_menu(handle: &tauri::AppHandle) -> tauri::Result<Submenu<Wry>> {
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
    let status_info = MenuItem::with_id(
        handle,
        "auto_start_info",
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

    SubmenuBuilder::new(handle, "Auto Start")
        .item(&status_info)
        .item(&PredefinedMenuItem::separator(handle)?)
        .item(&auto_start_toggle)
        .build()
}

fn discovery_sub_menu(handle: &tauri::AppHandle) -> tauri::Result<Submenu<Wry>> {
    let settings = handle.state::<Storage>().get();

    let info_text = format!("Status: {}", settings.duration.display());
    let status_info = MenuItem::with_id(handle, "show", info_text, false, None::<&str>)?;

    let always_off = checked_menu_item(Discovery::TurnOff, settings.duration).build(handle)?;
    let always_on = checked_menu_item(Discovery::AlwaysOn, settings.duration).build(handle)?;

    SubmenuBuilder::new(handle, "Server Discovery")
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
    let duration = Duration::from_secs(u64::from(timer_secs) * 60);
    let id = Discovery::OnDuration(duration).to_string();
    let text = format!("On for {} minute", timer_secs);

    CheckMenuItemBuilder::with_id(id, text).checked(discovery == Discovery::OnDuration(duration))
}
