use super::VolumeController;

use crate::types::shared::{AudioDevice, DeviceControl, VolumeControllerError, VolumeResult};

impl DeviceControl for VolumeController {
    fn get_playback_devices(&self) -> VolumeResult<Vec<AudioDevice>> {
        Err(VolumeControllerError::NotImplemented())
    }

    fn get_current_playback_device(&self) -> VolumeResult<AudioDevice> {
        Err(VolumeControllerError::NotImplemented())
    }
}
