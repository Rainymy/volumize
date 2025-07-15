use windows::Win32::{
    Media::Audio::{IMMDeviceEnumerator, MMDeviceEnumerator},
    System::Com::{
        CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX_ALL, COINIT_MULTITHREADED,
    },
};

use crate::types::shared::{VolumeControllerError, VolumeResult};

pub struct ComScope;

pub fn com_initialized_scope<F, R>(callback: F) -> VolumeResult<R>
where
    F: FnOnce(IMMDeviceEnumerator) -> VolumeResult<R>,
{
    // bind COM to this variable.
    // Auto cleanup on scope exit.
    let _com_guard = ComScope::new()?;
    let device_enumerator = unsafe { CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)? };

    return callback(device_enumerator);
}

impl ComScope {
    pub fn new() -> VolumeResult<Self> {
        unsafe {
            CoInitializeEx(None, COINIT_MULTITHREADED)
                .ok()
                .map_err(|e| {
                    VolumeControllerError::ComInitializationError(format!(
                        "Failed to initialize COM: {}",
                        e
                    ))
                })?;
        }
        Ok(Self)
    }
}

impl Drop for ComScope {
    fn drop(&mut self) {
        unsafe {
            CoUninitialize();
        }
    }
}
