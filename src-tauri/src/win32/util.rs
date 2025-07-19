#![allow(dead_code)]

use std::path::PathBuf;

use windows::{
    core::PWSTR,
    Win32::{
        Foundation::{CloseHandle, HANDLE, MAX_PATH},
        System::{
            ProcessStatus::{GetModuleBaseNameW, GetModuleFileNameExW},
            Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ},
        },
    },
};

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

pub fn pwstr_to_string(pwstr: PWSTR) -> String {
    if pwstr.is_null() {
        return "Unknown".to_string();
    }
    unsafe { pwstr.to_string().unwrap_or("failed to parse".into()) }
}
