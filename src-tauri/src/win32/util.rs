use std::{
    ffi::OsString,
    iter::once,
    os::windows::ffi::{OsStrExt, OsStringExt},
    path::PathBuf,
};

use windows::{
    core::{PCWSTR, PWSTR},
    Win32::{
        Foundation::{CloseHandle, HANDLE, MAX_PATH},
        System::{
            ProcessStatus::{GetModuleBaseNameW, GetModuleFileNameExW},
            Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ},
        },
    },
};

use crate::types::shared::{VolumeControllerError, VolumeResult};

pub unsafe fn get_process_info(
    process_id: u32,
) -> windows::core::Result<(String, Option<PathBuf>)> {
    let process_handle = OpenProcess(
        PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
        false,
        process_id,
    )?;

    let _guard = HandleGuard(process_handle);

    // Get process name (just the executable name)
    let mut process_name_buf = [0u16; MAX_PATH as usize];
    let name = if GetModuleBaseNameW(process_handle, None, &mut process_name_buf) > 0 {
        String::from_utf16_lossy(&process_name_buf)
            .trim_end_matches('\0')
            .to_string()
    } else {
        String::from("Unknown")
    };

    // Get full process path
    let mut process_path_buf = [0u16; MAX_PATH as usize];
    let path = if GetModuleFileNameExW(Some(process_handle), None, &mut process_path_buf) > 0 {
        let path_str = String::from_utf16_lossy(&process_path_buf)
            .trim_end_matches('\0')
            .to_string();
        if !path_str.is_empty() {
            Some(PathBuf::from(path_str))
        } else {
            None
        }
    } else {
        None
    };

    Ok((name, path))
}

pub struct HandleGuard(HANDLE);

impl Drop for HandleGuard {
    fn drop(&mut self) {
        unsafe {
            let _ = CloseHandle(self.0);
        }
    }
}

pub fn pwstr_to_string(pwstr: &PWSTR) -> VolumeResult<String> {
    if pwstr.is_null() {
        return Err(VolumeControllerError::OsApiError("Null PWSTR".into()));
    }

    unsafe {
        pwstr
            .to_string()
            .map_err(|e| VolumeControllerError::OsApiError(e.to_string()))
    }
}

pub fn string_to_pcwstr(str: String) -> (Vec<u16>, PCWSTR) {
    let wide: Vec<u16> = std::ffi::OsStr::new(&str)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    let pcwstr = PCWSTR(wide.as_ptr());
    (wide, pcwstr)
}

pub fn pwstr_to_os_string(pwstr: &PWSTR) -> OsString {
    if pwstr.is_null() {
        return OsString::from("Unknown");
    }

    unsafe { OsString::from_wide(pwstr.as_wide()) }
}

#[allow(dead_code)]
pub fn os_string_to_pwstr(rstr: &OsString) -> (Vec<u16>, PWSTR) {
    let mut wide: Vec<u16> = rstr.encode_wide().chain(once(0)).collect();
    let ptr = PWSTR(wide.as_mut_ptr());
    (wide, ptr)
}
