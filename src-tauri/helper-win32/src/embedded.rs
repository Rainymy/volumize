use windows::Win32::UI::{Shell::ShellExecuteW, WindowsAndMessaging::SW_SHOWNORMAL};
use windows::core::PCWSTR;

pub fn string_to_pcwstr(pw_string: &str) -> (Vec<u16>, PCWSTR) {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    let wide: Vec<u16> = OsStr::new(&pw_string)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    let pcwstr = PCWSTR(wide.as_ptr());
    (wide, pcwstr)
}

/// Elevates the current executable by running it with admin privileges.
pub fn elevate_current_exe() -> Result<bool, String> {
    use std::env::current_exe;
    use windows::core::Error;

    let args = super::formatter::get_formatted_args().join(" ");
    let current_exe = current_exe()
        .inspect_err(|err| eprintln!("{}", err))
        .map_err(|e| format!("Failed to get current exe: {e}"))?;

    let (_1, verb) = string_to_pcwstr("runas");
    let (_2, path) = string_to_pcwstr(&current_exe.display().to_string());
    let (_3, args) = string_to_pcwstr(&args);

    let exit_code = unsafe { ShellExecuteW(None, verb, path, args, None, SW_SHOWNORMAL) };

    // https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shellexecutew
    // - If the function succeeds, it returns a value greater than 32.

    if exit_code.is_invalid() {
        return Err(format!("Failed to elevate: {}", Error::from_thread()));
    }

    // Over 32 = user accepted admin elevation.
    Ok((exit_code.0 as usize) > 32)
}

/// Returns `true` if the current process is elevated (has admin privileges).
pub fn is_elevated() -> bool {
    use windows::Win32::Foundation::{CloseHandle, HANDLE};
    use windows::Win32::Security::{
        GetTokenInformation, TOKEN_ELEVATION_TYPE, TOKEN_QUERY, TokenElevationType,
        TokenElevationTypeFull,
    };
    use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

    let mut token = HANDLE::default();
    unsafe {
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token).is_err() {
            return false;
        }

        let mut elevation = TOKEN_ELEVATION_TYPE::default();
        let mut size = 0u32;

        if GetTokenInformation(
            token,
            TokenElevationType,
            Some(&mut elevation as *mut _ as *mut _),
            std::mem::size_of::<TOKEN_ELEVATION_TYPE>() as u32,
            &mut size,
        )
        .inspect_err(|err| eprintln!("{}", err))
        .is_err()
        {
            return false;
        }

        let _ = CloseHandle(token);

        !token.is_invalid() && elevation == TokenElevationTypeFull
    }
}
