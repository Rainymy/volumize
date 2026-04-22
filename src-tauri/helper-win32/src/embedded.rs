use std::path::PathBuf;

use windows::Win32::UI::Shell::ShellExecuteW;
use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;

pub static EMBEDDED_EXE: &[u8] =
    include_bytes!("../../../helper-win32/target/release/helper-win32.exe");

fn get_helper_path() -> PathBuf {
    use tauri::process::current_binary;
    use tauri::Env;

    let path = current_binary(&Env::default()).unwrap();

    let mut parent = match path.parent() {
        Some(parent) => parent.to_path_buf(),
        None => path,
    };

    parent.push("helper-win32.exe");
    parent
}

pub fn extract_helper() -> Result<PathBuf, String> {
    use super::sign::verify_hash;
    let path = get_helper_path();

    if !path.exists() || !verify_hash(&path) {
        std::fs::write(&path, EMBEDDED_EXE)
            .map_err(|e| format!("Failed to extract helper: {e}"))?;
    }

    Ok(path)
}

pub fn elevate_helper(path: &PathBuf, args: &str) -> Result<(), String> {
    use super::util::string_to_pcwstr;

    let (_, verb) = string_to_pcwstr("runas");
    let (_, path) = string_to_pcwstr(&path.display().to_string());
    let (_, args) = string_to_pcwstr(args);

    let exit_code = unsafe { ShellExecuteW(None, verb, path, args, None, SW_SHOWNORMAL) };

    if exit_code.is_invalid() {
        use windows_core::Error;
        return Err(format!(
            "Failed to elevate helper: {}",
            Error::from_thread().message()
        ));
    }

    Ok(())
}
