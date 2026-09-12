use windows::Win32::{
    Devices::FunctionDiscovery::PKEY_DeviceClass_IconPath,
    Foundation::GetLastError,
    Graphics::Gdi::{
        DeleteObject, GetDC, GetDIBits, GetObjectW, ReleaseDC, BITMAP, BITMAPINFO,
        BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS, HBITMAP, HDC,
    },
    Media::Audio::{
        IAudioSessionControl2, IAudioSessionManager2, IMMDevice, IMMDeviceEnumerator,
        MMDeviceEnumerator,
    },
    System::Com::{
        CoCreateInstance, CoInitializeEx, CoUninitialize,
        StructuredStorage::PropVariantToStringAlloc, CLSCTX_ALL, COINIT_MULTITHREADED, STGM_READ,
    },
    UI::{
        Shell::{ExtractIconExW, PathParseIconLocationW, PropertiesSystem::IPropertyStore},
        WindowsAndMessaging::{DestroyIcon, GetIconInfoExW, HICON, ICONINFOEXW},
    },
};

use crate::platform::win32::com_scope::ComManager;

use super::util;

struct AutoHDC(HDC);
impl Drop for AutoHDC {
    fn drop(&mut self) {
        unsafe {
            let _ = ReleaseDC(None, self.0);
        }
    }
}

struct AutoBitmap(HBITMAP);
impl Drop for AutoBitmap {
    fn drop(&mut self) {
        unsafe {
            let _ = DeleteObject(self.0.into());
        }
    }
}

struct IconData {
    data: Vec<u8>,
    width: u32,
    height: u32,
}

/**
 * Converts an HICON to a Vec<u8> representing the icon data.
 * Returns an empty vector if the conversion fails.
 */
fn convert_hicon_to_rgba(hicon: HICON) -> Option<IconData> {
    unsafe {
        let mut icon_info = ICONINFOEXW::default();
        icon_info.cbSize = std::mem::size_of::<ICONINFOEXW>() as u32;

        if !GetIconInfoExW(hicon, &mut icon_info).as_bool() {
            return None;
        }

        // RAII - cleanup
        let _color_cleanup = AutoBitmap(icon_info.hbmColor);
        let _mask_cleanup_ = AutoBitmap(icon_info.hbmMask);

        // Check if this is a monochrome icon (no color bitmap)
        let bitmap_handle = if !icon_info.hbmColor.is_invalid() {
            icon_info.hbmColor
        } else if !icon_info.hbmMask.is_invalid() {
            icon_info.hbmMask
        } else {
            eprintln!("No valid bitmap found");
            return None;
        };

        use std::mem::MaybeUninit;

        let mut bitmap: MaybeUninit<BITMAP> = MaybeUninit::uninit();
        let result = GetObjectW(
            bitmap_handle.into(),
            std::mem::size_of::<BITMAP>() as i32,
            Some(bitmap.as_mut_ptr().cast()),
        );
        if result == 0 {
            eprintln!("GetObjectW failed with error: {:?}", GetLastError());
            return None;
        }

        let bitmap = bitmap.assume_init();
        let bm_height = bitmap.bmHeight.unsigned_abs();
        let bm_width = bitmap.bmWidth.unsigned_abs();

        // No idea why multipled by 4. Maybe it's rgba channels?
        let mut buffer: Vec<u8> = vec![0u8; (bm_width * bm_height * 4) as usize];

        let hdc = GetDC(None);
        let _dc_cleanup = AutoHDC(hdc); // RAII - cleanup

        if hdc.is_invalid() {
            eprintln!("Failed to get screen DC");
            return None;
        }

        let mut bmp_info = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: bitmap.bmWidth,
                biHeight: -bitmap.bmHeight,
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB.0,

                ..Default::default()
            },
            bmiColors: Default::default(),
        };

        let hresult = GetDIBits(
            hdc,
            bitmap_handle,
            0,
            bitmap.bmHeight.unsigned_abs(),
            Some(buffer.as_mut_ptr().cast()),
            &mut bmp_info,
            DIB_RGB_COLORS,
        );

        if hresult == 0 {
            return None;
        }

        Some(IconData {
            width: bm_width,
            height: bm_height,
            data: buffer,
        })
    }
}

fn get_device_icon_location(device_id: String) -> windows::core::Result<String> {
    let enumerator: IMMDeviceEnumerator =
        unsafe { CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)? };

    let (_wide, pcwstr) = util::string_to_pcwstr(device_id.as_str());
    unsafe {
        let device: IMMDevice = enumerator.GetDevice(pcwstr)?;

        let props: IPropertyStore = device.OpenPropertyStore(STGM_READ)?;
        let var = props.GetValue(&PKEY_DeviceClass_IconPath)?;
        let pwstr = PropVariantToStringAlloc(&var)?;
        let icon_location = pwstr.to_string()?;
        windows::Win32::System::Com::CoTaskMemFree(Some(pwstr.0 as *const _));

        Ok(icon_location)
    }
}

fn extract_hicon(path: &str) -> windows::core::Result<HICON> {
    use windows::core::{PCWSTR, PWSTR};

    let expanded_path = expand_env_strings(path)?;
    let expanded_path = expanded_path.strip_prefix('@').unwrap_or(&expanded_path);

    let (mut buffer, _pzpath) = util::string_to_pcwstr(&expanded_path);
    let index = unsafe { PathParseIconLocationW(PWSTR(buffer.as_mut_ptr())) };

    let mut large_icon = HICON::default();
    let extracted = unsafe {
        ExtractIconExW(
            PCWSTR(buffer.as_ptr()),
            index,
            Some(&mut large_icon),
            None,
            1,
        )
    };

    match extracted == 0 || large_icon.is_invalid() {
        true => Err(windows::core::Error::from_thread()),
        false => Ok(large_icon),
    }
}

fn bitmap_to_image(data: IconData) -> Option<Vec<u8>> {
    let IconData {
        width,
        height,
        mut data,
    } = data;

    // Convert bgra -> rgba
    for chunk in data.chunks_exact_mut(4) {
        let [b, _, r, _] = chunk else { unreachable!() };
        std::mem::swap(b, r);
    }

    // Raw RGBA data.
    let rgba_img = image::RgbaImage::from_vec(width, height, data)?;
    // Container
    let mut encoded_image = std::io::Cursor::new(vec![]);

    // Start encoding.
    let _ = rgba_img
        .write_to(&mut encoded_image, image::ImageFormat::WebP)
        .ok()?;

    Some(encoded_image.into_inner())
}

fn expand_env_strings(input: &str) -> windows::core::Result<String> {
    use windows::Win32::System::Environment::ExpandEnvironmentStringsW;

    let (_input_wide, input_pcwstr) = util::string_to_pcwstr(input);

    let needed = unsafe { ExpandEnvironmentStringsW(input_pcwstr, None) };
    if needed == 0 {
        return Err(windows::core::Error::from_thread());
    }

    let mut buf = vec![0u16; needed as usize];
    unsafe { ExpandEnvironmentStringsW(input_pcwstr, Some(&mut buf)) };

    let s = String::from_utf16_lossy(&buf);
    Ok(s.trim_end_matches('\0').to_string())
}

fn extract_icon_with(f: impl FnOnce() -> Option<HICON>) -> Option<Vec<u8>> {
    struct ComGuard;
    impl ComGuard {
        fn new() -> Option<Self> {
            unsafe { CoInitializeEx(None, COINIT_MULTITHREADED).ok().ok()? }
            Some(Self)
        }
    }
    impl Drop for ComGuard {
        fn drop(&mut self) {
            unsafe { CoUninitialize() }
        }
    }

    // RAII
    let _com = ComGuard::new()?;
    let hicon = f()?;

    let rgba = convert_hicon_to_rgba(hicon);

    unsafe {
        let _ = DestroyIcon(hicon);
    }

    bitmap_to_image(rgba?)
}

pub fn extract_device_icon(device_id: String) -> Option<Vec<u8>> {
    extract_icon_with(|| {
        let icon_location = get_device_icon_location(device_id.clone()).ok()?;
        match extract_hicon(&icon_location) {
            Ok(hicon) => Some(hicon),
            Err(err) => {
                eprintln!("HICON Error: {}", err);
                None
            }
        }
    })
}

pub fn extract_system_icon() -> Option<Vec<u8>> {
    extract_icon_with(|| {
        use windows_core::Interface;
        let enumerator: IMMDeviceEnumerator =
            unsafe { CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL).ok()? };

        let icon_path = unsafe {
            let device = enumerator
                .GetDefaultAudioEndpoint(ComManager::E_DATAFLOW, ComManager::E_ROLE)
                .ok()?;

            let session_manager: IAudioSessionManager2 = device.Activate(CLSCTX_ALL, None).ok()?;
            let session_enumerator = session_manager.GetSessionEnumerator().ok()?;
            let count = session_enumerator.GetCount().ok()?;

            let mut found_icon_path: Option<String> = None;

            for i in 0..count {
                let session_control = session_enumerator.GetSession(i).ok()?;
                let session_control2: IAudioSessionControl2 =
                    match session_control.cast::<IAudioSessionControl2>() {
                        Ok(sc2) => sc2,
                        Err(_) => continue,
                    };

                let pid = session_control2.GetProcessId().unwrap_or(u32::MAX);

                if pid == 0 {
                    if let Ok(pwstr) = session_control2.GetIconPath() {
                        found_icon_path = Some(util::pwstr_to_string(pwstr));
                        break;
                    }
                }
            }
            found_icon_path?
        };

        let hicon: HICON = match extract_hicon(&icon_path) {
            Ok(hicon) => hicon,
            Err(err) => {
                eprintln!("HICON Error: {}", err);
                return None;
            }
        };

        Some(hicon)
    })
}

pub fn extract_icon(path: String) -> Option<Vec<u8>> {
    extract_icon_with(|| {
        let hicon: HICON = match extract_hicon(&path) {
            Ok(hicon) => hicon,
            Err(err) => {
                eprintln!("HICON Error: {}", err);
                return None;
            }
        };

        Some(hicon)
    })
}
