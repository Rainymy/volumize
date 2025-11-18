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
    pub tx: Arc<Mutex<UnboundedSender<VolumeCommand>>>,
    pub thread_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
}

impl VolumeCommandSender {
    pub fn send(&self, cmd: VolumeCommand) -> Result<(), String> {
        match self.tx.lock() {
            Ok(tx_guard) => tx_guard
                .send(cmd)
                .map_err(|e| format!("Send failed: {}", e)),
            Err(err) => Err(format!("Failed to lock sender: {}", err)),
        }
    }

    pub fn close_channel(&self) {
        if let Ok(mut tx_guard) = self.tx.lock() {
            // Replace the sender with a new one and drop the original
            let (new_tx, _) = unbounded_channel::<VolumeCommand>();
            let old_tx = std::mem::replace(&mut *tx_guard, new_tx);
            drop(old_tx); // Explicitly drop the sender
        }
    }

    pub fn shutdown(&self) -> Result<(), String> {
        self.close_channel();

        let mut handle = self
            .thread_handle
            .lock()
            .map_err(|e| format!("Failed to lock thread handle: {}", e))?;

        if let Some(join_handle) = handle.take() {
            join_handle
                .join()
                .map_err(|e| format!("Volume thread panicked during shutdown: {:?}", e))?;
        }

        Ok(())
    }
}
