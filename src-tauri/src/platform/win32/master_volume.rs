use windows::Win32::Media::Audio::Endpoints::IAudioEndpointVolume;

use crate::types::shared::{
    DeviceVolumeControl, VolumeControllerError, VolumeResult, VolumeValidation,
};

use shared_types::{AudioVolume, DeviceIdentifier, VolumePercent};

use super::VolumeController;

impl DeviceVolumeControl for VolumeController {
    fn get_device_volume(&self, device_id: DeviceIdentifier) -> VolumeResult<AudioVolume> {
        let endpoint: IAudioEndpointVolume = self.com.with_generic_device_activate(&device_id)?;
        let volume = unsafe {
            endpoint
                .GetMasterVolumeLevelScalar()
                .map_err(|err| VolumeControllerError::WindowsApiError(err))?
        };
        let is_muted = unsafe {
            endpoint
                .GetMute()
                .map_err(|err| VolumeControllerError::WindowsApiError(err))?
                .as_bool()
        };
        Ok(AudioVolume {
            current: volume,
            muted: is_muted,
        })
    }

    fn set_device_volume(
        &self,
        device_id: DeviceIdentifier,
        percent: VolumePercent,
    ) -> VolumeResult<()> {
        AudioVolume::validate_volume(percent)?;

        let endpoint: IAudioEndpointVolume = self.com.with_generic_device_activate(&device_id)?;
        unsafe {
            endpoint
                .SetMasterVolumeLevelScalar(percent, self.com.get_event_context())
                .map_err(|err| VolumeControllerError::WindowsApiError(err))
        }
    }

    fn mute_device(&self, device_id: DeviceIdentifier) -> VolumeResult<()> {
        let endpoint: IAudioEndpointVolume = self.com.with_generic_device_activate(&device_id)?;
        unsafe {
            endpoint
                .SetMute(true, self.com.get_event_context())
                .map_err(|err| VolumeControllerError::WindowsApiError(err))
        }
    }

    fn unmute_device(&self, device_id: DeviceIdentifier) -> VolumeResult<()> {
        let endpoint: IAudioEndpointVolume = self.com.with_generic_device_activate(&device_id)?;
        unsafe {
            endpoint
                .SetMute(false, self.com.get_event_context())
                .map_err(|err| VolumeControllerError::WindowsApiError(err))
        }
    }
}
