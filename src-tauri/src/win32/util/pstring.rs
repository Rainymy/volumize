use windows::core::{PCWSTR, PWSTR};

use std::{ffi::OsStr, os::windows::ffi::OsStrExt};

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
