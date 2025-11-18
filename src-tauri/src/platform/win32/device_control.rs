use windows::Win32::Media::Audio::IAudioSessionManager2;

use super::com_scope::ComManager;
use super::{convert, util, VolumeController};

use crate::types::shared::{
    AppIdentifier, AudioDevice, DeviceControl, DeviceIdentifier, VolumeControllerError,
    VolumeResult,
};

impl DeviceControl for VolumeController {
    fn get_device_applications(
        &self,
        device_id: DeviceIdentifier,
    ) -> VolumeResult<Vec<AppIdentifier>> {
        let device = self
            .get_playback_devices()?
            .into_iter()
            .find(|device| device.id == device_id)
            .ok_or(VolumeControllerError::DeviceNotFound(device_id.clone()))?;

        let session_enums = {
            let session: IAudioSessionManager2 =
                self.com.with_generic_device_activate(&device.id)?;

            unsafe { session.GetSessionEnumerator()? }
        };

        let imm_device = self.com.get_device_with_id(&device.id)?;
        let direction = convert::get_direction(&imm_device)?;

        let is_default_device =
            util::is_default_device(&imm_device, direction.edataflow, ComManager::E_ROLE);

        // This handles the case where sound system "application" exists
        // even tho playback device is not selected.
        // - Unsure if this is necessary
        let device_applications = convert::process_sessions(
            &session_enums,
            Some(direction.direction),
            is_default_device,
            &util::pwstr_to_string(unsafe { imm_device.GetId()? }),
        )?;

        return Ok(device_applications.iter().map(|f| f.process.id).collect());
    }

    fn get_playback_devices(&self) -> VolumeResult<Vec<AudioDevice>> {
        let device_ids = self.com.get_all_device_id()?;

        let devices = device_ids
            .into_iter()
            .filter_map(|val| self.com.get_device_with_id(&val).ok())
            .filter_map(|val| convert::process_device(val).ok())
            .collect();

        Ok(devices)
    }
}
