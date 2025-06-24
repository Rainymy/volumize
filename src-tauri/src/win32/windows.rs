use windows::{
    // core::*,
    Win32::{
        // Foundation::HWND,
        Media::Audio::*,
        System::Com::*,
    },
};

use crate::types::shared::VolumeResult;
use crate::types::shared::{AppIdentifier, AudioSession, VolumeController, VolumePercent};
use crate::win32::com_scope::ComScope;

pub fn com_initialized_scope<F, R>(callback: F) -> VolumeResult<R>
where
    F: FnOnce(IMMDeviceEnumerator) -> VolumeResult<R>,
{
    unsafe {
        // bind COM to this variable.
        // Auto cleanup on scope exit.
        let _com_guard = ComScope::new()?;

        let device_enumerator: IMMDeviceEnumerator =
            CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;

        callback(device_enumerator)
    }
}

pub fn windows_controller() -> VolumeResult<f32> {
    return com_initialized_scope(|device_enumerator| unsafe {
        let _default_device = device_enumerator.GetDefaultAudioEndpoint(eRender, eConsole)?;

        let volume: f32 = 0.0;

        Ok(volume)
    });
}

pub struct WindowsVolumeController;

impl VolumeController for WindowsVolumeController {
    fn get_playback_devices(&self) -> VolumeResult<Vec<AudioSession>> {
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
