use windows::Win32::Media::Audio::IAudioEndpointVolume;
use windows::{
    // core::*,
    Win32::{
        // Foundation::HWND,
        Media::Audio::*,
        System::Com::*,
    },
};

use crate::types::shared::VolumeControllerError;
use crate::types::shared::VolumeResult;
use crate::types::shared::{AppIdentifier, AudioSession, VolumeController, VolumePercent};

pub fn windows_controller() -> VolumeResult<f32> {
    // Specify the return Result type here
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED)
            .ok()
            .map_err(|e| {
                VolumeControllerError::ComInitializationError(format!(
                    "Failed to initialize COM: {}",
                    e
                ))
            })?;

        let result = (|| -> VolumeResult<f32> {
            let device_enumerator: IMMDeviceEnumerator =
                CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;

            let default_device = device_enumerator.GetDefaultAudioEndpoint(eRender, eConsole)?;
            let endpoint_volume: IAudioEndpointVolume =
                default_device.Activate(CLSCTX_ALL, None)?;

            let mut volume: f32 = 0.0;
            endpoint_volume.GetMasterVolumeLevelScalar(&mut volume)?;

            Ok(volume)
        })();

        CoUninitialize();

        result
    }
}

pub struct WindowsVolumeController;

impl VolumeController for WindowsVolumeController {
    fn get_playback_devices(&self) -> VolumeResult<Vec<AudioSession>> {
        // implement it!
        Ok(vec![])
    }

    fn get_current_playback_device(&self) -> VolumeResult<Option<AudioSession>> {
        Ok(None)
    }

    fn get_master_volume(&self) -> VolumeResult<Option<VolumePercent>> {
        Ok(Some(0.8))
    }

    fn set_master_volume(&self, _percent: VolumePercent) -> VolumeResult<()> {
        Ok(())
    }

    fn mute_master(&self) -> VolumeResult<()> {
        Ok(())
    }

    fn unmute_master(&self) -> VolumeResult<()> {
        Ok(())
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

    fn load_sessions(&self) -> VolumeResult<Vec<AudioSession>> {
        Ok(vec![])
    }

    fn get_all_applications(&self) -> VolumeResult<Vec<AudioSession>> {
        Ok(vec![])
    }
}
