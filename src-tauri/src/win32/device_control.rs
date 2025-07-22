use super::VolumeController;
use super::{convert, util};

use crate::types::shared::{AudioDevice, DeviceControl, VolumeResult};

impl DeviceControl for VolumeController {
    fn get_playback_devices(&self) -> VolumeResult<Vec<AudioDevice>> {
        let device_ids = self.com.get_all_device_id()?;

        let devices = device_ids
            .into_iter()
            .filter_map(|val| {
                let (_pw_buffer, pwstr) = util::string_to_pcwstr(val);
                self.com.get_device_with_id(pwstr).ok()
            })
            .filter_map(|val| convert::process_device(val).ok());

        Ok(devices.collect())
    }

    fn get_current_playback_device(&self) -> VolumeResult<AudioDevice> {
        convert::process_device(self.com.get_default_device()?)
    }
}
