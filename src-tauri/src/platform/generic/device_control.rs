use crate::types::shared::{DeviceControl, VolumeResult};
use shared_types::{AppIdentifier, AudioDevice, DeviceIdentifier};

use super::VolumeController;

impl DeviceControl for VolumeController {
    fn get_playback_devices(&self) -> VolumeResult<Vec<AudioDevice>> {
        Ok(Vec::new())
    }
    fn get_device_applications(
        &self,
        _device_id: DeviceIdentifier,
    ) -> VolumeResult<Vec<AppIdentifier>> {
        Ok(Vec::new())
    }
}
