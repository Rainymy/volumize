// #![allow(dead_code)]
use thiserror::Error;

pub const UPDATE_EVENT_NAME: &str = "update_event";
pub const VOLUME_LABEL_EVENT: &str = "volume-control-panel";
pub const WEBSOCKET_PORT: u16 = 9002;

pub type VolumeResult<T> = Result<T, VolumeControllerError>;

/// Represents an error that can occur during volume controller operations.
///
/// TODO: This needs cleaning up. Too many unnecessary variants.
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
}

use shared_types::{
    AppIdentifier, AudioApplication, AudioDevice, AudioVolume, DeviceIdentifier, VolumePercent,
};

pub trait DeviceVolumeControl {
    fn get_device_volume(&self, device_id: DeviceIdentifier) -> VolumeResult<AudioVolume>;
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
    fn check_and_reinit(&self);
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
