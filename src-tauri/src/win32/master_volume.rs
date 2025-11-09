use windows::Win32::Media::Audio::{Endpoints::IAudioEndpointVolume, IAudioSessionManager2};

use crate::{
    server::volume_control::platform::com_scope::ComManager,
    types::shared::{
        AppIdentifier, AudioVolume, DeviceControl, DeviceIdentifier, DeviceVolumeControl,
        VolumeControllerError, VolumePercent, VolumeResult, VolumeValidation,
    },
};

use super::{convert, util, VolumeController};

impl DeviceVolumeControl for VolumeController {
    fn get_device_applications(
        &self,
        device_id: DeviceIdentifier,
    ) -> VolumeResult<Vec<AppIdentifier>> {
        let device = self
            .get_playback_devices()?
            .into_iter()
            .find(|id| id.id == device_id)
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

        let device_applications = convert::process_sessions(
            &session_enums,
            Some(direction.direction),
            is_default_device,
            &util::pwstr_to_string(unsafe { imm_device.GetId()? }),
        )?;

        return Ok(device_applications.iter().map(|f| f.process.id).collect());
    }

    fn get_all_devices(&self) -> VolumeResult<Vec<DeviceIdentifier>> {
        self.com.get_all_device_id()
    }

    fn get_device_volume(&self, device_id: DeviceIdentifier) -> VolumeResult<VolumePercent> {
        let endpoint: IAudioEndpointVolume = self.com.with_device_id_activate(&device_id)?;
        unsafe {
            endpoint
                .GetMasterVolumeLevelScalar()
                .map_err(|err| VolumeControllerError::WindowsApiError(err))
        }
    }

    fn set_device_volume(
        &self,
        device_id: DeviceIdentifier,
        percent: VolumePercent,
    ) -> VolumeResult<()> {
        AudioVolume::validate_volume(percent)?;

        let endpoint: IAudioEndpointVolume = self.com.with_device_id_activate(&device_id)?;
        unsafe {
            endpoint
                .SetMasterVolumeLevelScalar(percent, self.com.get_event_context())
                .map_err(|err| VolumeControllerError::WindowsApiError(err))
        }
    }

    fn mute_device(&self, device_id: DeviceIdentifier) -> VolumeResult<()> {
        let endpoint: IAudioEndpointVolume = self.com.with_device_id_activate(&device_id)?;
        unsafe {
            endpoint
                .SetMute(true, self.com.get_event_context())
                .map_err(|err| VolumeControllerError::WindowsApiError(err))
        }
    }

    fn unmute_device(&self, device_id: DeviceIdentifier) -> VolumeResult<()> {
        let endpoint: IAudioEndpointVolume = self.com.with_device_id_activate(&device_id)?;
        unsafe {
            endpoint
                .SetMute(false, self.com.get_event_context())
                .map_err(|err| VolumeControllerError::WindowsApiError(err))
        }
    }
}
