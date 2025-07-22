use super::VolumeController;
use crate::types::shared::{
    AudioVolume, MasterVolumeControl, VolumeControllerError, VolumePercent, VolumeResult,
    VolumeValidation,
};

impl MasterVolumeControl for VolumeController {
    fn get_master_volume(&self) -> VolumeResult<Option<VolumePercent>> {
        let endpoint = self.com.with_default_audio_endpoint()?;
        unsafe { Ok(Some(endpoint.GetMasterVolumeLevelScalar()?)) }
    }

    fn set_master_volume(&self, percent: VolumePercent) -> VolumeResult<()> {
        AudioVolume::validate_volume(percent)?;

        let endpoint = self.com.with_default_audio_endpoint()?;
        unsafe {
            endpoint
                .SetMasterVolumeLevelScalar(percent, self.com.get_event_context())
                .map_err(|err| VolumeControllerError::WindowsApiError(err))
        }
    }

    fn mute_master(&self) -> VolumeResult<()> {
        let endpoint = self.com.with_default_audio_endpoint()?;
        unsafe {
            endpoint.SetMute(true, self.com.get_event_context())?;
        }
        Ok(())
    }

    fn unmute_master(&self) -> VolumeResult<()> {
        let endpoint = self.com.with_default_audio_endpoint()?;
        unsafe {
            endpoint.SetMute(false, self.com.get_event_context())?;
        }

        Ok(())
    }
}
