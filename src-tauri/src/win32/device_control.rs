use super::VolumeController;
use crate::types::shared::{AudioSession, DeviceControl, VolumeResult};

impl DeviceControl for VolumeController {
    fn get_playback_devices(&self) -> VolumeResult<Vec<AudioSession>> {
        Ok(vec![])
    }

    fn get_current_playback_device(&self) -> VolumeResult<Option<AudioSession>> {
        Ok(None)
    }
}
