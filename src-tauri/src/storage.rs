use serde::{Deserialize, Serialize};
use std::{
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tauri::{AppHandle, Manager};

use crate::Discovery;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Settings {
    pub dutaion: Discovery,
    pub port_address: u16,
    pub exit_to_tray: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            dutaion: Default::default(),
            port_address: 9002,
            exit_to_tray: Default::default(),
        }
    }
}

#[derive(Default)]
pub struct Storage {
    settings: Arc<Mutex<Settings>>,
}

impl Storage {
    fn settings_path(&self, app: &AppHandle) -> PathBuf {
        // Find a better place to save user configs. it's saving in ROAMING folder.
        let mut dir = app.path().app_data_dir().expect("app data dir");
        fs::create_dir_all(&dir).ok();
        dir.push("settings.json");
        dir
    }

    pub fn get_settings(&self) -> Settings {
        match self.settings.lock() {
            Ok(setting) => setting.clone(),
            Err(_) => Settings::default(),
        }
    }

    pub fn load_settings(&self, app: &AppHandle) {
        let settings = match fs::read(&self.settings_path(app)) {
            Ok(bytes) => serde_json::from_slice(&bytes).unwrap_or_default(),
            Err(_) => Settings::default(),
        };

        if let Ok(mut val) = self.settings.lock() {
            *val = settings;
        }
    }

    pub fn save_settings(&self, _app: &AppHandle, value: &Settings) -> std::io::Result<()> {
        let _data = serde_json::to_vec_pretty(value)?;
        // fs::write(&self.settings_path(app), data)
        Ok(())
    }
}
