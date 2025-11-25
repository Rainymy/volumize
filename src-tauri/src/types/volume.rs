use std::{
    sync::{Arc, Mutex},
    thread::JoinHandle,
};

use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};

use super::shared::{
    AppIdentifier, AudioApplication, AudioDevice, DeviceIdentifier, VolumePercent, VolumeResult,
};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum VolumeCommand {
    // ===================== DEVICE ======================
    DeviceGetVolume {
        request_id: String,
        id: DeviceIdentifier,
        #[serde(skip, default = "default_sender")]
        sender: UnboundedSender<VolumeResult<VolumePercent>>,
    },
    DeviceSetVolume {
        request_id: String,
        id: DeviceIdentifier,
        volume: VolumePercent,
    },
    DeviceMute {
        request_id: String,
        id: DeviceIdentifier,
    },
    DeviceUnmute {
        request_id: String,
        id: DeviceIdentifier,
    },

    // =================== Application ===================
    GetApplication {
        request_id: String,
        id: AppIdentifier,
        #[serde(skip, default = "default_sender")]
        sender: UnboundedSender<VolumeResult<AudioApplication>>,
    },
    ApplicationGetIcon {
        request_id: String,
        id: AppIdentifier,
        #[serde(skip, default = "default_sender")]
        sender: UnboundedSender<VolumeResult<Vec<u8>>>,
    },
    ApplicationGetVolume {
        request_id: String,
        id: AppIdentifier,
        #[serde(skip, default = "default_sender")]
        sender: UnboundedSender<VolumeResult<VolumePercent>>,
    },
    ApplicationSetVolume {
        request_id: String,
        id: AppIdentifier,
        volume: VolumePercent,
    },
    ApplicationMute {
        request_id: String,
        id: AppIdentifier,
    },
    ApplicationUnmute {
        request_id: String,
        id: AppIdentifier,
    },

    // ===================== MANAGER =====================
    GetDeviceApplications {
        request_id: String,
        id: DeviceIdentifier,
        #[serde(skip, default = "default_sender")]
        sender: UnboundedSender<VolumeResult<Vec<AppIdentifier>>>,
    },
    GetPlaybackDevices {
        request_id: String,
        #[serde(skip, default = "default_sender")]
        sender: UnboundedSender<VolumeResult<Vec<AudioDevice>>>,
    },
}
fn default_sender<T>() -> UnboundedSender<T> {
    unbounded_channel().0
}

impl VolumeCommand {
    pub fn get_name(&self) -> String {
        let serde_value = match serde_json::to_value(&self) {
            Ok(value) => value,
            Err(_) => serde_json::Value::Null,
        };

        let obj = match serde_value.as_object() {
            Some(value) => value,
            None => &serde_json::Map::new(),
        };

        for key in obj.keys() {
            return key.to_string();
        }

        if let Some(name) = serde_value.as_str() {
            return name.to_string();
        }

        match serde_json::to_string(&self) {
            Ok(name) => name,
            Err(_) => "unknown_name".into(),
        }
    }

    pub fn get_request_id(&self) -> String {
        let serde_value = match serde_json::to_value(&self) {
            Ok(value) => value,
            Err(_) => serde_json::Value::Null,
        };

        let obj = match serde_value.as_object() {
            Some(value) => value,
            None => &serde_json::Map::new(),
        };

        for value in obj.values() {
            let v_request_id = &value["request_id"];
            let id = match v_request_id.as_str() {
                Some(id) => id,
                None => continue,
            };

            return id.to_string();
        }

        String::new()
    }
}

pub struct VolumeCommandSender {
    pub server: Arc<Mutex<Option<VolumeServer>>>,
}

impl VolumeCommandSender {
    pub fn new() -> Self {
        Self {
            server: Default::default(),
        }
    }

    pub fn send(&self, cmd: VolumeCommand) -> Result<(), String> {
        let server = match self.server.lock() {
            Ok(server) => server,
            Err(err) => return Err(format!("Failed to lock server: {}", err)),
        };

        match &*server {
            Some(server) => server.send(cmd),
            None => Err("No server".to_string()),
        }
    }

    pub fn shutdown(&self) -> Result<(), String> {
        let server_guard = match self.server.lock() {
            Ok(mut server) => server.take(),
            Err(err) => return Err(format!("Failed to lock server: {}", err)),
        };

        match server_guard {
            Some(mut server) => server.shutdown(),
            None => Ok(()),
        }
    }
}

pub struct VolumeServer {
    pub tx: UnboundedSender<VolumeCommand>,
    pub thread_handle: Option<JoinHandle<()>>,
}

impl VolumeServer {
    fn send(&self, cmd: VolumeCommand) -> Result<(), String> {
        self.tx.send(cmd).map_err(|e| format!("Send failed: {}", e))
    }

    fn close_channel(&mut self) {
        let (new_tx, _) = unbounded_channel::<VolumeCommand>();
        // Replace the sender with a new one and drop the original
        let old_tx = std::mem::replace(&mut self.tx, new_tx);
        drop(old_tx); // Explicitly drop the sender
    }

    pub fn shutdown(&mut self) -> Result<(), String> {
        self.close_channel();

        let thread = match self.thread_handle.take() {
            Some(handle) => handle.join(),
            None => return Ok(()),
        };

        thread.map_err(|e| format!("Volume thread panicked during shutdown: {:?}", e))
    }
}
