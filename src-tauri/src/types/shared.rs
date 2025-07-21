#![allow(dead_code)]
use std::path::PathBuf;
use thiserror::Error;

pub type VolumePercent = f32;
pub type AppIdentifier = String;
pub type VolumeResult<T> = Result<T, VolumeControllerError>;

#[derive(Debug, Clone)]
pub enum SessionType {
    Application,
    Device,
    System,
    Unknown,
}

#[derive(Debug, Clone)]
pub enum SessionDirection {
    Render,
    Capture,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub id: u32,
    pub name: String,
    pub path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct AudioSession {
    pub process: ProcessInfo,
    pub session_type: SessionType,
    pub direction: SessionDirection,
    pub device: AudioDevice,
    pub volume: AudioVolume,
    pub sound_playing: bool,
}

#[derive(Debug, Clone)]
pub struct AudioDevice {
    pub id: String,
    pub name: String,
    pub direction: SessionDirection,
}

#[derive(Debug, Clone)]
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
    #[error("IPC communication error: {0}")]
    IpcError(String),
    #[error("Serialization/deserialization error: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("COM initialization error: {0}")]
    ComError(String),
    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub trait MasterVolumeControl {
    fn get_master_volume(&self) -> VolumeResult<Option<VolumePercent>>;
    fn set_master_volume(&self, percent: VolumePercent) -> VolumeResult<()>;
    fn mute_master(&self) -> VolumeResult<()>;
    fn unmute_master(&self) -> VolumeResult<()>;
}

pub trait ApplicationVolumeControl {
    fn set_app_volume(&self, app: AppIdentifier, percent: VolumePercent) -> VolumeResult<()>;
    fn mute_app(&self, app: AppIdentifier) -> VolumeResult<()>;
    fn unmute_app(&self, app: AppIdentifier) -> VolumeResult<()>;
    fn get_all_applications(&self) -> VolumeResult<Vec<AudioSession>>;
}

pub trait DeviceControl {
    fn get_playback_devices(&self) -> VolumeResult<Vec<AudioSession>>;
    fn get_current_playback_device(&self) -> VolumeResult<Option<AudioSession>>;
}

pub trait VolumeControllerTrait:
    MasterVolumeControl + ApplicationVolumeControl + DeviceControl
{
    fn load_sessions(&self) -> VolumeResult<Vec<AudioSession>>;
}

pub trait VolumeValidation {
    const MIN_VOLUME: VolumePercent = 0.0;
    const MAX_VOLUME: VolumePercent = 1.0;
    const DEFAULT_VOLUME: VolumePercent = 1.0;
    fn validate_volume(volume: VolumePercent) -> VolumeResult<()>;
}

impl VolumeValidation for AudioVolume {
    fn validate_volume(volume: VolumePercent) -> VolumeResult<()> {
        if !(Self::MIN_VOLUME..=Self::MAX_VOLUME).contains(&volume) {
            return Err(VolumeControllerError::InvalidVolumePercentage(volume));
        }
        Ok(())
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
