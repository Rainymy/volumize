pub type VolumePercent = f32;
pub type AppIdentifier = String;

pub enum SessionType {
    Application = "Application",
    Device = "Device",
}

enum SessionDirection {
    Render = "Render",
    Capture = "Capture",
    NOOP = "Noop",
}

// #[derive(Debug, Clone)]
pub struct AudioSession {
    pub name: string,
    pub session_type: SessionType,
    pub direction: SessionDirection,
    pub device_output: SessionDirection,
    pub device_name: string,
    pub id: string,
    pub window_title: string,
    pub volume_percent: number,
    pub muted: boolean,
    pub active: boolean,
}

pub trait VolumeController {
    fn get_playback_devices(&self) -> Result<Vec<AudioSession>>;
    fn get_current_playback_device(&self) -> Result<Option<AudioSession>>;
    fn get_master_volume(&self) -> Result<Option<VolumePercent>>;
    fn set_master_volume(&self, percent: VolumePercent) -> Result<()>;
    fn mute_master(&self) -> Result<()>;
    fn unmute_master(&self) -> Result<()>;
    fn set_app_volume(&self, app: AppIdentifier, percent: VolumePercent) -> Result<()>;
    fn mute_app(&self, app: AppIdentifier) -> Result<()>;
    fn unmute_app(&self, app: AppIdentifier) -> Result<()>;
    fn load_sessions(&self) -> Result<Vec<AudioSession>>;
    fn get_all_applications(&self) -> Result<Vec<AudioSession>>;
}
