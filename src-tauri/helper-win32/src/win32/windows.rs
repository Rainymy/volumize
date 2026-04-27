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
pub fn elevate_current_exe() -> Result<bool, String> {
    use super::get_formatted_args;
    use std::env::current_exe;

    use windows::Win32::UI::{Shell::ShellExecuteW, WindowsAndMessaging::SW_SHOWNORMAL};
    use windows::core::{Error, PCWSTR};

    let current_exe = current_exe().map_err(|e| format!("Failed to get current exe: {e}"))?;

    let verb = string_to_pcwstr_vec("runas");
    let path = string_to_pcwstr_vec(&current_exe.display().to_string());
    let args = string_to_pcwstr_vec(&get_formatted_args().join(" "));

    let exit_code = unsafe {
        ShellExecuteW(
            None,
            PCWSTR(verb.as_ptr()),
            PCWSTR(path.as_ptr()),
            PCWSTR(args.as_ptr()),
            None,
            SW_SHOWNORMAL,
        )
    };

    if exit_code.is_invalid() {
        return Err(format!("Failed to elevate: {}", Error::from_thread()));
    }

    // https://learn.microsoft.com/en-us/windows/win32/api/shellapi/nf-shellapi-shellexecutew
    // - If the function succeeds, it returns a value greater than 32.
    //
    // Over 32 = user accepted admin elevation.
    Ok((exit_code.0 as usize) > 32)
}

/// Returns `true` if the current process is elevated (has admin privileges).
#[cfg(windows)]
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

struct ComGuard();

impl ComGuard {
    fn new() -> Result<Self, String> {
        use windows::Win32::System::Com::{COINIT_MULTITHREADED, CoInitializeEx};
        let guard = match unsafe { CoInitializeEx(None, COINIT_MULTITHREADED).ok() } {
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
        match CoCreateInstance(&NetworkListManager, None, CLSCTX_ALL) {
            Ok(v) => v,
            Err(err) => return Err(format!("[CoCreateInstance] {}", err)),
        }
    };

    // Early exit if not connected
    let is_connected = unsafe { nlm.IsConnected().map(|v| v.as_bool()) }
        .map_err(|err| format!("[IsConnected] {}", err))?;

    if !is_connected {
        return Err("No network connection".to_string());
    }

    let network_enumator = unsafe {
        match nlm.GetNetworks(NLM_ENUM_NETWORK_CONNECTED) {
            Ok(connections) => connections,
            Err(err) => return Err(format!("[GetNetworks] {}", err)),
        }
    };

    let network = unsafe {
        let mut networks = [None::<INetwork>; 1];
        let mut pceltfetched = 0u32;

        match network_enumator.Next(&mut networks, Some(&mut pceltfetched)) {
            Ok(_) if pceltfetched > 0 => {}
            Ok(_) => return Err("[Next] No network connection".to_string()),
            Err(err) => return Err(format!("[Next] {}", err)),
        }
        match networks[0].take() {
            Some(net) => net,
            None => {
                return Err(
                    "Something went very wrong. Expected [networks; 1] to be valid".to_string(),
                );
            }
        }
    };

    let is_private = match unsafe { network.GetCategory() } {
        Ok(NLM_NETWORK_CATEGORY_PRIVATE) => true,
        // I'm not sure what the domain authenticated category means,
        // so treat it as private.
        Ok(NLM_NETWORK_CATEGORY_DOMAIN_AUTHENTICATED) => true,
        Ok(NLM_NETWORK_CATEGORY_PUBLIC) => false,
        Ok(_) => false,
        Err(err) => return Err(format!("[GetCategory] {}", err)),
    };

    Ok(is_private)
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
