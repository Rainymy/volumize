use std::{ffi::OsStr, os::windows::ffi::OsStrExt, path::PathBuf};

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

pub unsafe fn get_process_info(process_id: u32) -> (String, Option<PathBuf>) {
    let access_bits = PROCESS_QUERY_INFORMATION | PROCESS_VM_READ;
    let process_handle = match OpenProcess(access_bits, false, process_id) {
        Ok(handle) => handle,
        Err(_) => return (String::new(), None),
    };

    // Keep process handle alive until end of the function.
    let _guard = HandleGuard(process_handle);

    // Get process name (just the executable name)
    // https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-getmodulebasenamew
    //
    // If the function fails, the return value is zero.
    let mut process_name_buf = [0u16; MAX_PATH as usize];
    let name = if GetModuleBaseNameW(process_handle, None, &mut process_name_buf) > 0 {
        process_lossy_name(&process_name_buf)
    } else {
        String::new()
    };

    // Get full process path
    // https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-getmodulefilenameexw
    //
    // If the function fails, the return value is zero.
    let mut process_path_buf = [0u16; MAX_PATH as usize];
    let path = if GetModuleFileNameExW(Some(process_handle), None, &mut process_path_buf) > 0 {
        let path_str = process_lossy_name(&process_path_buf);
        if !path_str.is_empty() {
            Some(PathBuf::from(path_str))
        } else {
            None
        }
    } else {
        None
    };

    (name, path)
}

pub struct HandleGuard(HANDLE);

impl Drop for HandleGuard {
    fn drop(&mut self) {
        unsafe {
            let _ = CloseHandle(self.0);
        }
    }
}

pub fn pwstr_to_string(pwstr: PWSTR) -> String {
    if pwstr.is_null() {
        return "Unknown".to_string();
    }

    unsafe {
        pwstr
            .to_string()
            .unwrap_or_else(|_| String::from("Invalid UTF-16"))
    }
}

pub fn string_to_pcwstr(pw_string: &str) -> (Vec<u16>, PCWSTR) {
    let wide: Vec<u16> = OsStr::new(&pw_string)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    let pcwstr = PCWSTR(wide.as_ptr());
    (wide, pcwstr)
}

pub fn process_lossy_name(value: &[u16]) -> String {
    String::from_utf16_lossy(&value)
        .trim_end_matches('\0')
        .to_string()
}
