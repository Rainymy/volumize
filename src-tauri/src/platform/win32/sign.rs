use std::{env, path::PathBuf};

use super::util::string_to_pcwstr;

const HELPER_HASH: &str = env!("EMBEDDED_WIN32_SHA256"); // set in build.rs

pub fn verify_hash(path: &PathBuf) -> bool {
    let hash_match = match sha256_file(path) {
        Ok(hash) => hash == HELPER_HASH,
        Err(e) => {
            eprintln!("Failed to read helper: {e}");
            return false;
        }
    };

    if !hash_match {
        eprintln!("Hash mismatch! Expected {HELPER_HASH}, got {hash_match}");
    }
    hash_match
}

fn sha256_file(path: &PathBuf) -> Result<String, std::io::Error> {
    use sha2::{Digest, Sha256};

    Ok(Sha256::digest(std::fs::read(path)?)
        .into_iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<String>>()
        .join(""))
}

use windows::Win32::Security::WinTrust::{
    WinVerifyTrust, WINTRUST_ACTION_GENERIC_VERIFY_V2, WINTRUST_DATA, WINTRUST_DATA_0,
    WINTRUST_FILE_INFO, WTD_CHOICE_FILE, WTD_REVOKE_WHOLECHAIN, WTD_STATEACTION_CLOSE,
    WTD_STATEACTION_VERIFY, WTD_UI_NONE,
};
use windows_core::HRESULT;

pub fn verify_signature(path: &PathBuf) -> Result<(), String> {
    let (_, pcwstr) = string_to_pcwstr(&path.display().to_string());

    let mut file_info = WINTRUST_FILE_INFO {
        cbStruct: std::mem::size_of::<WINTRUST_FILE_INFO>() as u32,
        pcwszFilePath: pcwstr,
        ..Default::default()
    };
    let mut trust_data = WINTRUST_DATA {
        cbStruct: std::mem::size_of::<WINTRUST_DATA>() as u32,
        dwUIChoice: WTD_UI_NONE,
        fdwRevocationChecks: WTD_REVOKE_WHOLECHAIN,
        dwUnionChoice: WTD_CHOICE_FILE,
        dwStateAction: WTD_STATEACTION_VERIFY,
        Anonymous: WINTRUST_DATA_0 {
            pFile: &mut file_info,
        },
        ..Default::default()
    };

    let exit_code = unsafe {
        use windows::Win32::Foundation::HWND;
        let mut policy_guid = WINTRUST_ACTION_GENERIC_VERIFY_V2;

        let result = WinVerifyTrust(
            HWND::default(),
            &mut policy_guid,
            &mut trust_data as *mut _ as *mut _,
        );

        trust_data.dwStateAction = WTD_STATEACTION_CLOSE;
        WinVerifyTrust(
            HWND::default(),
            &mut policy_guid,
            &mut trust_data as *mut _ as *mut _,
        );
        result
    };

    if exit_code != 0 {
        Err(format!(
            "Signature verification failed: {}",
            HRESULT(exit_code).message()
        ))
    } else {
        Ok(())
    }
}
