use windows::Win32::Media::Audio::IAudioSessionManager2;

use super::{convert, VolumeController};
use crate::{
    platform::{com_scope::ComManager, util},
    types::shared::{
        AppIdentifier, ApplicationVolumeControl, AudioSession, DeviceControl, VolumePercent,
        VolumeResult,
    },
};

impl ApplicationVolumeControl for VolumeController {
    fn get_all_applications(&self) -> VolumeResult<Vec<AudioSession>> {
        // loop over all devices and get all application
        let devices = self.get_playback_devices()?;
        let mut applications: Vec<AudioSession> = vec![];

        for device in devices {
            let (_pcw_buffer, pcw_str) = util::string_to_pcwstr(device.id);
            let imm_device = self.com.get_device_with_id(pcw_str)?;

            let session: IAudioSessionManager2 =
                unsafe { imm_device.Activate(ComManager::CLS_CONTEXT, None)? };

            applications.push(AudioSession {
                applications: unsafe {
                    convert::process_sessions(&session.GetSessionEnumerator()?)?
                },
                device: convert::process_device(imm_device)?,
            });
        }

        Ok(applications)
    }

    fn set_app_volume(&self, _app: AppIdentifier, _percent: VolumePercent) -> VolumeResult<()> {
        Ok(())
    }

    fn mute_app(&self, _app: AppIdentifier) -> VolumeResult<()> {
        Ok(())
    }

    fn unmute_app(&self, _app: AppIdentifier) -> VolumeResult<()> {
        Ok(())
    }
}
