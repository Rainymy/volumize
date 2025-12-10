use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tauri::{AppHandle, Manager};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Settings {
    pub duration: super::tray::Discovery,
    pub port_address: u16,
    pub exit_to_tray: bool,
    pub autostart: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            duration: Default::default(),
            port_address: 9002,
            exit_to_tray: true,
            autostart: true,
        }
    }
}

#[derive(Default)]
pub struct Storage {
    settings: Arc<Mutex<Settings>>,
    pub tray_icon_id: Arc<Mutex<Option<String>>>,
}

impl Storage {
    fn settings_path(&self, app: &AppHandle) -> PathBuf {
        #[cfg(not(target_os = "ios"))]
        let os_save_path = app.path().home_dir();
        #[cfg(target_os = "ios")]
        let os_save_path = app.path().app_config_dir();

        let mut dir = os_save_path.expect("Expected app save directory");
        dir.push(".volumize");

        fs::create_dir_all(&dir).ok();
        dir.push("settings.json");
        dir
    }

    pub fn get(&self) -> Settings {
        match self.settings.lock() {
            Ok(setting) => setting.clone(),
            Err(_) => Settings::default(),
        }
    }

    pub fn load(&self, app: &AppHandle) {
        let settings = match fs::read(&self.settings_path(app)) {
            Ok(bytes) => serde_json::from_slice(&bytes).unwrap_or_default(),
            Err(_) => Settings::default(),
        };

        if let Ok(mut val) = self.settings.lock() {
            *val = settings;
        }
    }

    pub fn update(&self, value: Settings) {
        if let Ok(mut item) = self.settings.lock() {
            *item = value;
        }
    }

    pub fn save(&self, app: &AppHandle, value: &Settings) -> std::io::Result<()> {
        let data = serde_json::to_vec_pretty(value)?;
        fs::write(&self.settings_path(app), data)
    }
}
