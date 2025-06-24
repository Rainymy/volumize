use thiserror::Error;

pub type VolumePercent = f32;
pub type AppIdentifier = String;

#[derive(Debug, Clone)]
pub enum SessionType {
    Application,
    Device,
}

#[derive(Debug, Clone)]
pub enum SessionDirection {
    Render,
    Capture,
    Noop,
}

#[derive(Debug, Clone)]
pub struct AudioSession {
    pub name: String,
    pub session_type: SessionType,
    pub direction: SessionDirection,
    pub device_output: SessionDirection,
    pub device_name: String,
    pub id: String,
    pub window_title: String,
    pub volume_percent: VolumePercent,
    pub muted: bool,
    pub active: bool,
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
    #[error("Windows API error: {0}")]
    WindowsApiError(#[from] windows::core::Error),
    #[error("IPC communication error: {0}")]
    IpcError(String),
    #[error("Serialization/deserialization error: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("COM initialization error: {0}")]
    ComInitializationError(String),
    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub type VolumeResult<T> = Result<T, VolumeControllerError>;

pub trait VolumeController {
    fn get_playback_devices(&self) -> VolumeResult<Vec<AudioSession>>;
    fn get_current_playback_device(&self) -> VolumeResult<Option<AudioSession>>;
    fn get_master_volume(&self) -> VolumeResult<Option<VolumePercent>>;
    fn set_master_volume(&self, percent: VolumePercent) -> VolumeResult<()>;
    fn mute_master(&self) -> VolumeResult<()>;
    fn unmute_master(&self) -> VolumeResult<()>;
    fn set_app_volume(&self, app: AppIdentifier, percent: VolumePercent) -> VolumeResult<()>;
    fn mute_app(&self, app: AppIdentifier) -> VolumeResult<()>;
    fn unmute_app(&self, app: AppIdentifier) -> VolumeResult<()>;
    fn load_sessions(&self) -> VolumeResult<Vec<AudioSession>>;
    fn get_all_applications(&self) -> VolumeResult<Vec<AudioSession>>;
}
