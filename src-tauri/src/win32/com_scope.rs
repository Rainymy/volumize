use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_MULTITHREADED};

use crate::types::shared::{VolumeControllerError, VolumeResult};

pub struct ComScope;

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
