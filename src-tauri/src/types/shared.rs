// #![allow(dead_code)]
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;

pub type VolumePercent = f32;
pub type AppIdentifier = u32;
pub type DeviceIdentifier = String;

pub type VolumeResult<T> = Result<T, VolumeControllerError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionType {
    Application,
    Device,
    System,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionDirection {
    Render,
    Capture,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub id: AppIdentifier,
    pub name: String,
    pub path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioApplication {
    pub process: ProcessInfo,
    pub session_type: SessionType,
    pub direction: SessionDirection,
    pub volume: AudioVolume,
    pub device_id: DeviceIdentifier,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioDevice {
    pub id: DeviceIdentifier,
    pub name: String,
    pub friendly_name: String,
    pub direction: SessionDirection,
    pub is_default: bool,
    pub volume: AudioVolume,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AudioVolume {
    pub current: VolumePercent,
    pub muted: bool,
}

#[derive(Debug, Error)]
pub enum VolumeControllerError {
    #[error("Device not found: {0}")]
    DeviceNotFound(String),
    #[error("Application not found: {0}")]
    ApplicationNotFound(String),
    #[error("Invalid volume percentage: {0}")]
    InvalidVolumePercentage(f32),
    #[error("Operating system audio API error: {0}")]
    OsApiError(String),
    #[cfg(target_os = "windows")]
    #[error("Windows API error: {0}")]
    WindowsApiError(#[from] windows::core::Error),
    #[error("Serialization/deserialization error: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("COM initialization error: {0}")]
    ComError(String),
    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub trait DeviceVolumeControl {
    fn get_device_volume(&self, device_id: DeviceIdentifier) -> VolumeResult<VolumePercent>;
    fn set_device_volume(&self, id: DeviceIdentifier, volume: VolumePercent) -> VolumeResult<()>;
    fn mute_device(&self, id: DeviceIdentifier) -> VolumeResult<()>;
    fn unmute_device(&self, id: DeviceIdentifier) -> VolumeResult<()>;
}

pub trait ApplicationVolumeControl {
    fn get_application(&self, id: AppIdentifier) -> VolumeResult<AudioApplication>;
    fn get_app_volume(&self, id: AppIdentifier) -> VolumeResult<AudioVolume>;
    fn set_app_volume(&self, id: AppIdentifier, volume: VolumePercent) -> VolumeResult<()>;
    fn mute_app(&self, id: AppIdentifier) -> VolumeResult<()>;
    fn unmute_app(&self, id: AppIdentifier) -> VolumeResult<()>;
}

pub trait DeviceControl {
    fn get_playback_devices(&self) -> VolumeResult<Vec<AudioDevice>>;
    fn get_device_applications(&self, id: DeviceIdentifier) -> VolumeResult<Vec<AppIdentifier>>;
}

pub trait VolumeControllerTrait:
    DeviceVolumeControl + ApplicationVolumeControl + DeviceControl
{
    fn cleanup(&self);
}

pub trait VolumeValidation {
    const MIN_VOLUME: VolumePercent = 0.0;
    const MAX_VOLUME: VolumePercent = 1.0;
    #[allow(dead_code)]
    const DEFAULT_VOLUME: VolumePercent = 1.0;
    fn validate_volume(volume: VolumePercent) -> VolumeResult<AudioVolume>;
}

impl VolumeValidation for AudioVolume {
    fn validate_volume(volume: VolumePercent) -> VolumeResult<Self> {
        if !(Self::MIN_VOLUME..=Self::MAX_VOLUME).contains(&volume) {
            return Err(VolumeControllerError::InvalidVolumePercentage(volume));
        }
        Ok(Self::new(volume))
    }
}

impl AudioVolume {
    fn new(volume: VolumePercent) -> Self {
        Self {
            current: volume,
            muted: false,
        }
    }
}
