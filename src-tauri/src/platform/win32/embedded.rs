use std::fs;
use std::path::PathBuf;

use windows::Win32::UI::Shell::ShellExecuteW;
use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;

pub static EMBEDDED_EXE: &[u8] =
    include_bytes!("../../../helper-win32/target/release/helper-win32.exe");

fn get_helper_path(root: &PathBuf) -> PathBuf {
    let folder = root.parent().unwrap();
    let mut folder = folder.to_path_buf().clone();
    folder.push("helper.exe");
    folder
}

pub fn extract_helper(root: &PathBuf) -> Result<std::path::PathBuf, String> {
    use super::sign::verify_hash;
    let path = get_helper_path(root);

    if !path.exists() || !verify_hash(&path) {
        fs::write(&path, EMBEDDED_EXE).map_err(|e| format!("Failed to extract helper: {e}"))?;
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
        return Err("Failed to elevate helper".to_string());
    }

    Ok(())
}
