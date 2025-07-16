use windows::Win32::{
    Media::Audio::{IMMDeviceEnumerator, MMDeviceEnumerator},
    System::Com::{
        CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX_ALL, COINIT_MULTITHREADED,
    },
};

use crate::types::shared::{VolumeControllerError, VolumeResult};

#[must_use = "ComScope must be kept alive to maintain COM initialization"]
pub struct ComScope;

pub fn with_com_initialized<F, R>(callback: F) -> VolumeResult<R>
where
    F: FnOnce(IMMDeviceEnumerator) -> VolumeResult<R>,
{
    // bind COM to this variable. Else value is immediately dropped!
    let _com_guard = ComScope::try_new()?;
    let device_enumerator = unsafe { CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)? };
    callback(device_enumerator)
}

impl ComScope {
    pub fn try_new() -> VolumeResult<Self> {
        #[cfg(debug_assertions)]
        println!("ComScope initialized!");

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
        };

        #[cfg(debug_assertions)]
        println!("ComScope Dropped!");
    }
}
