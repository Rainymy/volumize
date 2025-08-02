use windows::Win32::Media::Audio::Endpoints::IAudioEndpointVolume;

use crate::types::shared::{
    AudioVolume, MasterVolumeControl, VolumeControllerError, VolumePercent, VolumeResult,
    VolumeValidation,
};

use super::VolumeController;

impl MasterVolumeControl for VolumeController {
    fn get_master_volume(&self) -> VolumeResult<Option<VolumePercent>> {
        let endpoint: IAudioEndpointVolume = self.com.with_default_generic_activate()?;
        unsafe { Ok(Some(endpoint.GetMasterVolumeLevelScalar()?)) }
    }

    fn set_master_volume(&self, percent: VolumePercent) -> VolumeResult<()> {
        AudioVolume::validate_volume(percent)?;

        let endpoint: IAudioEndpointVolume = self.com.with_default_generic_activate()?;
        unsafe {
            endpoint
                .SetMasterVolumeLevelScalar(percent, self.com.get_event_context())
                .map_err(|err| VolumeControllerError::WindowsApiError(err))
        }
    }

    fn mute_master(&self) -> VolumeResult<()> {
        let endpoint: IAudioEndpointVolume = self.com.with_default_generic_activate()?;
        unsafe {
            endpoint.SetMute(true, self.com.get_event_context())?;
        }
        Ok(())
    }

    fn unmute_master(&self) -> VolumeResult<()> {
        let endpoint: IAudioEndpointVolume = self.com.with_default_generic_activate()?;
        unsafe {
            endpoint.SetMute(false, self.com.get_event_context())?;
        }

        Ok(())
    }
}
