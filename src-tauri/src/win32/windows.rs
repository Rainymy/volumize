use windows::{
    core::*,
    Win32::{
        // Foundation::HWND,
        Media::Audio::*,
        System::Com::*,
    },
};

pub fn windows_controller() -> Result<f32> {
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED);

        let device_enumerator: IMMDeviceEnumerator =
            CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;

        let default_device = device_enumerator.GetDefaultAudioEndpoint(eRender, eConsole)?;
        let endpoint_volume: IAudioEndpointVolume = default_device.Activate(CLSCTX_ALL, None)?;

        let mut volume: f32 = 0.0;
        endpoint_volume.GetMasterVolumeLevelScalar(&mut volume)?;

        CoUninitialize();
        Ok(volume);
    }
    return Err(Error::empty());
}
