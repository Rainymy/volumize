use super::VolumeController;

use crate::types::shared::{
    AppIdentifier, AudioDevice, DeviceControl, DeviceIdentifier, VolumeControllerError,
    VolumeResult,
};

impl DeviceControl for VolumeController {
    fn get_playback_devices(&self) -> VolumeResult<Vec<AudioDevice>> {
        Err(VolumeControllerError::Unknown("Not implemented".into()))
    }
    fn get_device_applications(
        &self,
        _device_id: DeviceIdentifier,
    ) -> VolumeResult<Vec<AppIdentifier>> {
        return Ok(vec![]);
    }
}
