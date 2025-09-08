use std::path::PathBuf;

use windows::Win32::{
    Foundation::{CloseHandle, HANDLE, MAX_PATH},
    System::{
        ProcessStatus::{GetModuleBaseNameW, GetModuleFileNameExW},
        Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ},
    },
};

use super::process_lossy_name;

#[must_use = "Handle must be kept alive to maintain access to process"]
pub struct HandleGuard(HANDLE);

impl HandleGuard {
    fn get_process_name(&self) -> String {
        let mut buffer = [0u16; MAX_PATH as usize];

        unsafe {
            // Get process name (just the executable name)
            // https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-getmodulebasenamew
            //
            // If the function fails, the return value is zero.
            if GetModuleBaseNameW(self.0, None, &mut buffer) > 0 {
                process_lossy_name(&buffer)
            } else {
                String::new()
            }
        }
    }

    fn get_process_path(&self) -> Option<PathBuf> {
        let mut process_path_buf = [0u16; MAX_PATH as usize];
        unsafe {
            // Get full process path
            // https://learn.microsoft.com/en-us/windows/win32/api/psapi/nf-psapi-getmodulefilenameexw
            //
            // If the function fails, the return value is zero.
            if GetModuleFileNameExW(Some(self.0), None, &mut process_path_buf) > 0 {
                let path_str = process_lossy_name(&process_path_buf);
                if path_str.is_empty() {
                    None
                } else {
                    Some(PathBuf::from(path_str))
                }
            } else {
                None
            }
        }
    }
}

impl Drop for HandleGuard {
    fn drop(&mut self) {
        unsafe {
            let _ = CloseHandle(self.0);
        }
    }
}

pub fn get_process_info(process_id: u32) -> (String, Option<PathBuf>) {
    let access_bits = PROCESS_QUERY_INFORMATION | PROCESS_VM_READ;
    let process_handle = unsafe {
        match OpenProcess(access_bits, false, process_id) {
            Ok(handle) => handle,
            Err(_) => return (String::new(), None),
        }
    };

    // Keep process handle alive until end of the function.
    let guard = HandleGuard(process_handle);
    let name = guard.get_process_name();
    let path = guard.get_process_path();

    (name, path)
}
