use std::num::ParseFloatError;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use tauri::menu::{
    CheckMenuItemBuilder, Menu, MenuItem, PredefinedMenuItem, Submenu, SubmenuBuilder,
};
use tauri::Wry;

pub struct ClickState {
    pub last_click_time: Arc<Mutex<Instant>>,
    double_click_threshold_ms: u64,
}

impl ClickState {
    /// If parameter is None, this will default to 500ms.
    ///
    /// windows double click definition:
    /// - https://learn.microsoft.com/en-us/windows/win32/controls/ttm-setdelaytime
    pub fn new(double_click_threshold_ms: Option<u64>) -> Self {
        Self {
            last_click_time: Arc::new(Mutex::new(Instant::now())),
            double_click_threshold_ms: double_click_threshold_ms.unwrap_or(500),
        }
    }
    pub fn is_double_click(&self) -> bool {
        let mut last_click_time = match self.last_click_time.lock() {
            Ok(value) => value,
            Err(e) => e.into_inner(),
        };
        let threshold = Duration::from_millis(self.double_click_threshold_ms);
        let now = Instant::now();

        let is_double = now.saturating_duration_since(*last_click_time) < threshold;

        *last_click_time = now;
        is_double
    }
}

pub fn create_tray(handle: &tauri::AppHandle) -> tauri::Result<Menu<Wry>> {
    let show = MenuItem::with_id(handle, "show", "Show", true, Some("None"))?;
    let separator = PredefinedMenuItem::separator(handle)?;
    let quit = PredefinedMenuItem::quit(handle, Some("Exit"))?;
    let about = PredefinedMenuItem::about(handle, Some("about"), None)?;

    let tray_menu = Menu::new(handle)?;
    let _ = tray_menu.append(&show)?;
    let _ = tray_menu.append(&separator)?;
    let _ = tray_menu.append(&create_sub_menu(handle)?)?; // Sub-menu
    let _ = tray_menu.append(&separator)?;
    let _ = tray_menu.append(&about)?;
    let _ = tray_menu.append(&quit)?;

    Ok(tray_menu)
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Discovery {
    TurnOff,
    OnDuration(Duration),
    AlwaysOn,
}

impl std::fmt::Display for Discovery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Discovery::TurnOff => write!(f, "turn_off"),
            Discovery::OnDuration(mins) => write!(f, "{}", mins.as_secs()),
            Discovery::AlwaysOn => write!(f, "always_on"),
        }
    }
}

impl FromStr for Discovery {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == Discovery::AlwaysOn.to_string() {
            return Ok(Discovery::AlwaysOn);
        }
        if s == Discovery::TurnOff.to_string() {
            return Ok(Discovery::TurnOff);
        }

        match s.parse::<f32>() {
            Ok(mins) => Ok(Discovery::OnDuration(Duration::from_secs_f32(mins))),
            Err(e) => Err(e),
        }
    }
}

fn create_sub_menu(handle: &tauri::AppHandle) -> tauri::Result<Submenu<Wry>> {
    let separator = PredefinedMenuItem::separator(handle)?;

    let status_info = MenuItem::with_id(handle, "show", "Status: always on", false, None::<&str>)?;
    let always_off = CheckMenuItemBuilder::with_id(Discovery::TurnOff.to_string(), "Turn off")
        .checked(true)
        .build(handle)?;
    let always_on = CheckMenuItemBuilder::with_id(Discovery::AlwaysOn.to_string(), "Always on")
        .checked(false)
        .build(handle)?;

    SubmenuBuilder::new(handle, "Server Discovery: active")
        .item(&status_info)
        .item(&separator)
        .item(&always_off)
        .item(&timer_submenu(2).build(handle)?)
        .item(&timer_submenu(5).build(handle)?)
        .item(&timer_submenu(15).build(handle)?)
        .item(&always_on)
        .build()
}

fn timer_submenu(timer: u32) -> CheckMenuItemBuilder {
    let duration = Duration::from_secs(u64::from(timer) * 60);
    let id = Discovery::OnDuration(duration).to_string();
    let text = format!("On for {} minute", timer);

    CheckMenuItemBuilder::with_id(id, text).checked(false)
}
