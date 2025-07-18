/// Utility functions for clipboard, memory management, and other helpers
use anyhow::Result;
use std::ffi::CString;
use winapi::shared::windef::HWND;
use winapi::um::winbase::{GlobalAlloc, GlobalLock, GlobalSize, GlobalUnlock, GMEM_MOVEABLE};
use winapi::um::winuser::{
    CloseClipboard, EmptyClipboard, GetClipboardData, OpenClipboard, SetClipboardData,
};

const CF_TEXT: u32 = 1;
const CF_UNICODETEXT: u32 = 13;

/// Get UTF-8 string from Windows clipboard
pub fn get_from_clipboard_utf8(hwnd: HWND) -> Result<String> {
    unsafe {
        if OpenClipboard(hwnd) == 0 {
            return Err(anyhow::anyhow!("Failed to open clipboard"));
        }

        let result = (|| -> Result<String> {
            // Try to get Unicode text first
            let handle = GetClipboardData(CF_UNICODETEXT);
            if !handle.is_null() {
                let ptr = GlobalLock(handle) as *const u16;
                if !ptr.is_null() {
                    let len = GlobalSize(handle) / 2; // UTF-16 characters
                    let slice = std::slice::from_raw_parts(ptr, len);

                    // Find the null terminator
                    let mut end = 0;
                    for (i, &ch) in slice.iter().enumerate() {
                        if ch == 0 {
                            end = i;
                            break;
                        }
                    }

                    let utf16_slice = &slice[..end];
                    GlobalUnlock(handle);

                    return String::from_utf16(utf16_slice)
                        .map_err(|e| anyhow::anyhow!("Failed to convert UTF-16 to string: {}", e));
                }
            }

            // Fallback to ANSI text
            let handle = GetClipboardData(CF_TEXT);
            if !handle.is_null() {
                let ptr = GlobalLock(handle) as *const u8;
                if !ptr.is_null() {
                    let len = GlobalSize(handle);
                    let slice = std::slice::from_raw_parts(ptr, len);

                    // Find null terminator
                    let mut end = 0;
                    for (i, &ch) in slice.iter().enumerate() {
                        if ch == 0 {
                            end = i;
                            break;
                        }
                    }

                    let string = String::from_utf8_lossy(&slice[..end]).into_owned();
                    GlobalUnlock(handle);
                    return Ok(string);
                }
            }

            Err(anyhow::anyhow!("No text data in clipboard"))
        })();

        CloseClipboard();
        result
    }
}

/// Copy UTF-8 string to Windows clipboard
pub fn copy_to_clipboard_utf8(text: &str, hwnd: HWND) -> Result<()> {
    unsafe {
        if OpenClipboard(hwnd) == 0 {
            return Err(anyhow::anyhow!("Failed to open clipboard"));
        }

        let result = (|| -> Result<()> {
            EmptyClipboard();

            // Convert to UTF-16 for Unicode clipboard format
            let wide: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();

            let size = wide.len() * 2; // size in bytes
            let handle = GlobalAlloc(GMEM_MOVEABLE, size);
            if handle.is_null() {
                return Err(anyhow::anyhow!("Failed to allocate memory"));
            }

            let ptr = GlobalLock(handle) as *mut u16;
            if ptr.is_null() {
                return Err(anyhow::anyhow!("Failed to lock memory"));
            }

            std::ptr::copy_nonoverlapping(wide.as_ptr(), ptr, wide.len());
            GlobalUnlock(handle);

            if SetClipboardData(CF_UNICODETEXT, handle).is_null() {
                return Err(anyhow::anyhow!("Failed to set clipboard data"));
            }

            Ok(())
        })();

        CloseClipboard();
        result
    }
}

/// Convert a Rust string to a C string safely
#[allow(dead_code)]
pub fn to_c_string(s: &str) -> Result<CString> {
    CString::new(s).map_err(|e| anyhow::anyhow!("Failed to create C string: {}", e))
}

/// Safely read a null-terminated string from a raw pointer
#[allow(dead_code)]
pub unsafe fn c_str_to_string(ptr: *const i8) -> Result<String> {
    if ptr.is_null() {
        return Err(anyhow::anyhow!("Null pointer"));
    }

    let c_str = std::ffi::CStr::from_ptr(ptr);
    c_str
        .to_str()
        .map(|s| s.to_owned())
        .map_err(|e| anyhow::anyhow!("Failed to convert C string: {}", e))
}
