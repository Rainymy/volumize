use std::{mem::MaybeUninit, path::PathBuf};

use windows::Win32::{
    Foundation::GetLastError,
    Graphics::Gdi::{
        DeleteObject, GetDC, GetDIBits, GetObjectW, ReleaseDC, BITMAP, BITMAPINFO,
        BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS, HBITMAP, HDC,
    },
    Storage::FileSystem::FILE_FLAGS_AND_ATTRIBUTES,
    System::Com::{CoInitializeEx, CoUninitialize, COINIT_MULTITHREADED},
    UI::{
        Shell::{SHGetFileInfoW, SHFILEINFOW, SHGFI_ICON, SHGFI_LARGEICON},
        WindowsAndMessaging::{DestroyIcon, GetIconInfoExW, HICON, ICONINFOEXW},
    },
};

use super::util;

struct AutoHDC(HDC);
impl Drop for AutoHDC {
    fn drop(&mut self) {
        unsafe {
            let _ = ReleaseDC(None, self.0);
            // let _ = DeleteDC(self.0);
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

fn extract_hicon(path: &str) -> windows::core::Result<HICON> {
    let (_buffer, pzpath) = util::string_to_pcwstr(path);

    let mut psfi = MaybeUninit::<SHFILEINFOW>::uninit();

    let info_exit_code = unsafe {
        SHGetFileInfoW(
            pzpath,
            // without SHGFI_USEFILEATTRIBUTES this flag is ignored.
            FILE_FLAGS_AND_ATTRIBUTES(0),
            Some(psfi.as_mut_ptr()),
            std::mem::size_of::<SHFILEINFOW>() as u32,
            SHGFI_ICON | SHGFI_LARGEICON, // SHGFI_USEFILEATTRIBUTES
        )
    };

    let psfi = unsafe { psfi.assume_init() };

    if info_exit_code == 0 || psfi.hIcon.is_invalid() {
        return Err(windows::core::Error::from_win32());
    }

    Ok(psfi.hIcon)
}

fn bitmap_to_image(data: IconData) -> Option<Vec<u8>> {
    let IconData {
        width,
        height,
        mut data,
    } = data;

    // convert bgra -> rgba
    for chunk in data.chunks_exact_mut(4) {
        let [b, _, r, _] = chunk else { unreachable!() };
        std::mem::swap(b, r);
    }

    // Raw RBGA data.
    let img = image::RgbaImage::from_vec(width, height, data)?;

    // Encode the image as PNG. (buffer is smaller than the original)
    let mut buffer = std::io::Cursor::new(vec![]);
    let _ = img.write_to(&mut buffer, image::ImageFormat::Png).ok()?;
    Some(buffer.into_inner())
}

pub fn extract_icon(path: PathBuf) -> Option<Vec<u8>> {
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

    let hicon: HICON = match extract_hicon(path.to_str()?) {
        Ok(hicon) => hicon,
        Err(err) => {
            eprintln!("HICON Error: {}", err);
            return None;
        }
    };

    let bitmap_bgra = convert_hicon_to_rgba(hicon);

    unsafe {
        let _ = DestroyIcon(hicon);
    }

    bitmap_to_image(bitmap_bgra?)
}
