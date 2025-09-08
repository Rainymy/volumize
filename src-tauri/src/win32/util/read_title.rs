use std::{ffi::OsString, os::windows::ffi::OsStringExt, path::Path, ptr};

use windows::{
    core::PCWSTR,
    Win32::Storage::FileSystem::{GetFileVersionInfoSizeW, GetFileVersionInfoW, VerQueryValueW},
};

use super::string_to_pcwstr;

pub fn get_main_window_title(file_path: &Path) -> Option<String> {
    if !file_path.exists() {
        return None;
    }

    let path_as_str = file_path.as_os_str().to_str()?;
    let (_pwc_path, pwc_buffer) = string_to_pcwstr(path_as_str);

    unsafe {
        let version_info_size = get_version_size(&pwc_buffer)?;
        let mut version_info = vec![0u8; version_info_size as usize];

        GetFileVersionInfoW(
            pwc_buffer,
            None,
            version_info_size,
            version_info.as_mut_ptr() as *mut _,
        )
        .ok()?;

        for translation in get_translations(&version_info) {
            // The query string for FileDescription in the default language
            let pcw_query = string_to_pcwstr(
                format!("\\StringFileInfo\\{}\\FileDescription", translation).as_str(),
            );

            if let Some(description) = query_version_string(&version_info, &pcw_query.1) {
                if !description.is_empty() {
                    return Some(description);
                }
            }
        }
    }

    None
}

unsafe fn query_version_string(version_info: &[u8], query: &PCWSTR) -> Option<String> {
    let mut query_ptr: *mut u16 = ptr::null_mut();
    let mut query_len: u32 = 0;

    if VerQueryValueW(
        version_info.as_ptr() as *const _,
        *query,
        &mut query_ptr as *mut *mut u16 as *mut *mut _,
        &mut query_len,
    )
    .as_bool()
        && !query_ptr.is_null()
        && query_len > 1
    {
        let query_slice =
            std::slice::from_raw_parts(query_ptr, (query_len as usize).saturating_sub(1));

        let description = OsString::from_wide(query_slice)
            .to_string_lossy()
            .trim()
            .to_owned();

        if !description.is_empty() {
            return Some(description);
        }
    }

    None
}

unsafe fn get_translations(version_info: &[u8]) -> Vec<String> {
    let mut translation_ptr: *mut u16 = ptr::null_mut();
    let mut translation_len: u32 = 0;

    let query = string_to_pcwstr("\\VarFileInfo\\Translation");
    let mut translations = Vec::new();

    if VerQueryValueW(
        version_info.as_ptr() as *const _,
        query.1,
        &mut translation_ptr as *mut *mut u16 as *mut *mut _,
        &mut translation_len,
    )
    .as_bool()
        && !translation_ptr.is_null()
        && translation_len >= 4
    {
        // Each entry is 4 bytes (2 u16s)
        //     [lang_id₁][codepage₁]         [lang_id₂][codepage₂]...
        //      2 bytes  +  2 bytes    |      2 bytes +  2 bytes
        // |---- Entry 1 (4 bytes) ----|---- Entry 2 (4 bytes) ----|
        let translation_count = (translation_len as usize) / 4;
        let translation_slice = std::slice::from_raw_parts(translation_ptr, translation_count * 2);

        for i in (0..translation_count).step_by(2) {
            let lang_id = translation_slice[i];
            let codepage = translation_slice[i + 1];
            translations.push((lang_id, codepage));
        }
    }

    if translations.is_empty() {
        // Fallback to common translations if the translation table is not available
        translations.extend([
            (0x0409, 0x04B0), // English (US), Unicode
            (0x0409, 0x04E4), // English (US), Windows Latin-1
            (0x0000, 0x04B0), // Neutral language, Unicode
            (0x0000, 0x04E4), // Neutral language, Windows Latin-1
        ]);
    }

    return translations
        .into_iter()
        .map(|(lang_id, codepage)| format!("{:04X}{:04X}", lang_id, codepage))
        .collect();
}

fn get_version_size(pwc_buffer: &PCWSTR) -> Option<u32> {
    let version_info_size = unsafe { GetFileVersionInfoSizeW(*pwc_buffer, None) };

    if version_info_size == 0 {
        return None;
    }

    Some(version_info_size)
}
