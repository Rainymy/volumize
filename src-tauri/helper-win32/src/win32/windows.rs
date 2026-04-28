#[cfg(windows)]
pub fn string_to_pcwstr_vec(pw_string: &str) -> Vec<u16> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    OsStr::new(&pw_string)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect::<Vec<u16>>()
}

/// Elevates the current executable by running it with admin privileges.
#[cfg(target_family = "windows")]
pub fn elevate_current_exe() -> Result<u32, String> {
    use super::get_formatted_args;
    use std::env::current_exe;

    use windows::Win32::Foundation::CloseHandle;
    use windows::Win32::System::Threading::{GetExitCodeProcess, INFINITE, WaitForSingleObject};
    use windows::Win32::UI::Shell::{SEE_MASK_NOCLOSEPROCESS, SHELLEXECUTEINFOW, ShellExecuteExW};
    use windows::Win32::UI::WindowsAndMessaging::SW_HIDE;
    use windows::core::PCWSTR;

    let current_exe = current_exe().map_err(|e| format!("Failed to get current exe: {e}"))?;

    let verb = string_to_pcwstr_vec("runas");
    let path = string_to_pcwstr_vec(&current_exe.display().to_string());
    let args = string_to_pcwstr_vec(&get_formatted_args().join(" "));

    let mut info = SHELLEXECUTEINFOW {
        cbSize: std::mem::size_of::<SHELLEXECUTEINFOW>() as u32,
        fMask: SEE_MASK_NOCLOSEPROCESS,
        lpVerb: PCWSTR(verb.as_ptr()),
        lpFile: PCWSTR(path.as_ptr()),
        lpParameters: PCWSTR(args.as_ptr()),
        nShow: SW_HIDE.0,
        ..Default::default()
    };

    unsafe {
        ShellExecuteExW(&mut info).map_err(|e| format!("Failed to elevate: {e}"))?;

        // No need to handle the return value of WaitForSingleObject.
        WaitForSingleObject(info.hProcess, INFINITE);

        let mut exit_code = 0u32;
        GetExitCodeProcess(info.hProcess, &mut exit_code)
            .map_err(|e| format!("Failed to get exit code: {e}"))?;

        CloseHandle(info.hProcess).map_err(|e| format!("Failed to close handle: {e}"))?;

        Ok(exit_code)
    }
}

/// Returns `true` if the current process is elevated (has admin privileges).
#[cfg(windows)]
pub fn is_elevated() -> bool {
    use windows::Win32::Foundation::{CloseHandle, HANDLE};
    use windows::Win32::Security::{
        GetTokenInformation, TOKEN_ELEVATION, TOKEN_QUERY, TokenElevation,
    };
    use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

    unsafe {
        let mut token = HANDLE::default();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token).is_err() {
            return false;
        }

        let mut elevation = TOKEN_ELEVATION::default();
        let mut size = 0u32;

        let result = GetTokenInformation(
            token,
            TokenElevation,
            Some(&mut elevation as *mut _ as *mut _),
            std::mem::size_of::<TOKEN_ELEVATION>() as u32,
            &mut size,
        )
        .is_ok();

        let _ = CloseHandle(token);

        result && elevation.TokenIsElevated != 0
    }
}

struct ComGuard();

impl ComGuard {
    fn new() -> Result<Self, String> {
        use windows::Win32::System::Com::{COINIT_APARTMENTTHREADED, CoInitializeEx};
        let guard = match unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok() } {
            Ok(_) => Ok(Self()),
            Err(err) => Err(format!("[CoInitializeEx] {}", err)),
        };

        return guard;
    }
}

impl Drop for ComGuard {
    fn drop(&mut self) {
        use windows::Win32::System::Com::CoUninitialize;
        unsafe { CoUninitialize() };
    }
}

#[cfg(windows)]
pub fn is_private_network() -> Result<bool, String> {
    use windows::Win32::Networking::NetworkListManager::{
        INetwork, INetworkListManager, NLM_ENUM_NETWORK_CONNECTED,
        NLM_NETWORK_CATEGORY_DOMAIN_AUTHENTICATED, NLM_NETWORK_CATEGORY_PRIVATE,
        NLM_NETWORK_CATEGORY_PUBLIC, NetworkListManager,
    };
    use windows::Win32::System::Com::{CLSCTX_ALL, CoCreateInstance};

    let _guard = ComGuard::new()?;

    let nlm: INetworkListManager = unsafe {
        CoCreateInstance(&NetworkListManager, None, CLSCTX_ALL)
            .map_err(|e| format!("[CoCreateInstance] {}", e))?
    };

    // Early exit if not connected
    let is_connected = unsafe { nlm.IsConnected() }
        .map(|v| v.as_bool())
        .map_err(|err| format!("[IsConnected] {}", err))?;

    if !is_connected {
        return Err("No network connection".to_string());
    }

    let network_enumator = unsafe {
        nlm.GetNetworks(NLM_ENUM_NETWORK_CONNECTED)
            .map_err(|e| format!("[GetNetworks] {}", e))?
    };

    let mut found_any = false;

    loop {
        let mut networks: [Option<INetwork>; 16] = std::array::from_fn(|_| None);
        let mut fetched = 0u32;

        let done = unsafe {
            match network_enumator.Next(&mut networks, Some(&mut fetched)) {
                Ok(_) if fetched == 0 => true,
                Ok(_) => false,
                Err(e) => return Err(format!("[Next] {}", e)),
            }
        };

        for network in networks[..fetched as usize].iter_mut() {
            let network = match network.take() {
                Some(n) => n,
                None => continue,
            };

            found_any = true;

            let category = unsafe {
                network
                    .GetCategory()
                    .map_err(|e| format!("[GetCategory] {}", e))?
            };

            match category {
                NLM_NETWORK_CATEGORY_PRIVATE => return Ok(true),
                NLM_NETWORK_CATEGORY_DOMAIN_AUTHENTICATED => return Ok(true),
                NLM_NETWORK_CATEGORY_PUBLIC => {}
                _ => {}
            }
        }

        if done {
            break;
        }
    }

    if !found_any {
        return Err("No networks returned by enumerator".to_string());
    }

    Ok(false)
}

#[cfg(windows)]
pub fn local_utc_offset_secs() -> i64 {
    use windows::Win32::System::Time::{
        DYNAMIC_TIME_ZONE_INFORMATION, GetDynamicTimeZoneInformation, GetTimeZoneInformation,
        TIME_ZONE_INFORMATION,
    };

    unsafe {
        let mut tz = DYNAMIC_TIME_ZONE_INFORMATION::default();
        let _ = GetDynamicTimeZoneInformation(&mut tz);

        if tz.DynamicDaylightTimeDisabled {
            return 0 as i64;
        }

        let mut _tz_info = TIME_ZONE_INFORMATION::default();
        let time_zone = GetTimeZoneInformation(&mut _tz_info);

        let is_daylight = time_zone == 2;

        let bias = tz.Bias;
        let offset_bias = if is_daylight {
            tz.DaylightBias
        } else {
            tz.StandardBias
        };

        -((offset_bias + bias) as i64 * 60)
    }
}
