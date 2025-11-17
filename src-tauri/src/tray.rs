use std::time::Duration;

use tauri::menu::{
    CheckMenuItemBuilder, Menu, MenuItem, PredefinedMenuItem, Submenu, SubmenuBuilder,
};
use tauri::{Manager, Wry};

use crate::types::storage::Storage;
use crate::types::tray::Discovery;

pub fn create_tray(handle: &tauri::AppHandle) -> tauri::Result<Menu<Wry>> {
    let show = MenuItem::with_id(handle, "show", "Show", true, None::<&str>)?;
    let refresh_token = MenuItem::with_id(handle, "refresh", "Refresh menu", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(handle)?;
    let quit = PredefinedMenuItem::quit(handle, Some("Exit"))?;

    let tray_menu = Menu::new(handle)?;
    let _ = tray_menu.append(&show)?;
    let _ = tray_menu.append(&refresh_token)?;
    let _ = tray_menu.append(&separator)?;
    let _ = tray_menu.append(&create_sub_menu(handle)?)?; // Sub-menu
    let _ = tray_menu.append(&separator)?;
    let _ = tray_menu.append(&quit)?;

    Ok(tray_menu)
}

fn create_sub_menu(handle: &tauri::AppHandle) -> tauri::Result<Submenu<Wry>> {
    let settings = handle.state::<Storage>().get();

    let info_text = format!("Status: {}", settings.duration.display());
    let status_info = MenuItem::with_id(handle, "show", info_text, false, None::<&str>)?;

    // dbg!(&settings.dutaion);

    let always_off = checked_menu_item(Discovery::TurnOff, settings.duration).build(handle)?;
    let always_on = checked_menu_item(Discovery::AlwaysOn, settings.duration).build(handle)?;

    SubmenuBuilder::new(handle, "Server Discovery")
        .item(&status_info)
        .item(&PredefinedMenuItem::separator(handle)?)
        .item(&always_off)
        .item(&timer_submenu(2, settings.duration).build(handle)?)
        .item(&timer_submenu(5, settings.duration).build(handle)?)
        .item(&timer_submenu(15, settings.duration).build(handle)?)
        .item(&always_on)
        .build()
}

fn checked_menu_item(item: Discovery, settings: Discovery) -> CheckMenuItemBuilder {
    CheckMenuItemBuilder::with_id(Discovery::to_string(&item), Discovery::display(&item))
        .checked(settings == item)
}

fn timer_submenu(timer: u32, discovery: Discovery) -> CheckMenuItemBuilder {
    let duration = Duration::from_secs(u64::from(timer) * 60);
    let id = Discovery::OnDuration(duration).to_string();
    let text = format!("On for {} minute", timer);

    CheckMenuItemBuilder::with_id(id, text).checked(discovery == Discovery::OnDuration(duration))
}
