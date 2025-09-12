use windows::Win32::Media::Audio::Endpoints::IAudioEndpointVolume;

use crate::types::shared::{
    AudioVolume, DeviceIdentifier, DeviceVolumeControl, VolumeControllerError, VolumePercent,
    VolumeResult, VolumeValidation,
};

use super::VolumeController;

impl DeviceVolumeControl for VolumeController {
    fn get_device_volume(
        &self,
        device_id: DeviceIdentifier,
    ) -> VolumeResult<Option<VolumePercent>> {
        let endpoint: IAudioEndpointVolume = self.com.with_device_id_activate(&device_id)?;

        unsafe {
            let get_volume = endpoint
                .GetMasterVolumeLevelScalar()
                .map_err(|err| VolumeControllerError::WindowsApiError(err))?;

            Ok(Some(get_volume))
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
