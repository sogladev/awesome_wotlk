/// Bug fixes module - handles clipboard and other client fixes
use crate::game_client::addresses;
use retour::RawDetour;
use std::ffi::CStr;
use winapi::shared::windef::HWND;

static mut CLIPBOARD_GET_HOOK: Option<RawDetour> = None;
static mut CLIPBOARD_SET_HOOK: Option<RawDetour> = None;

// Hook implementation for getting clipboard string
unsafe extern "C" fn clipboard_get_string_hook(hwnd: HWND) -> *const i8 {
    // Get UTF-8 string from clipboard using our utility function
    match crate::utils::get_from_clipboard_utf8(hwnd) {
        Ok(string) => {
            // Allocate memory using WoW's allocator
            let len = string.len() + 1; // +1 for null terminator
            let buf = crate::game_client::wow_alloc(len) as *mut i8;
            if !buf.is_null() {
                std::ptr::copy_nonoverlapping(string.as_ptr() as *const i8, buf, string.len());
                *buf.add(string.len()) = 0; // null terminator
                buf as *const i8
            } else {
                std::ptr::null()
            }
        }
        Err(_) => std::ptr::null(),
    }
}

// Hook implementation for setting clipboard string
unsafe extern "C" fn clipboard_set_string_hook(buf: *const i8, hwnd: HWND) -> i32 {
    if buf.is_null() {
        return 0;
    }

    // Convert C string to Rust string
    let c_str = CStr::from_ptr(buf);
    match c_str.to_str() {
        Ok(string) => {
            if crate::utils::copy_to_clipboard_utf8(string, hwnd).is_ok() {
                1
            } else {
                0
            }
        }
        Err(_) => 0,
    }
}

/// Initialize bug fixes by setting up function hooks
pub fn initialize() {
    unsafe {
        // Hook clipboard functions to fix UTF-8 handling
        let get_string_addr = addresses::CLIPBOARD_GET_STRING as *const ();
        let set_string_addr = addresses::CLIPBOARD_SET_STRING as *const ();

        // Create the detours
        let get_hook = RawDetour::new(get_string_addr, clipboard_get_string_hook as *const ());

        let set_hook = RawDetour::new(set_string_addr, clipboard_set_string_hook as *const ());

        // Store and enable the hooks
        if let Ok(hook) = get_hook {
            hook.enable().unwrap_or_else(|_e| {
                #[cfg(debug_assertions)]
                eprintln!("Failed to enable clipboard get string hook: {:?}", _e);
            });
            CLIPBOARD_GET_HOOK = Some(hook);
        }

        if let Ok(hook) = set_hook {
            hook.enable().unwrap_or_else(|_e| {
                #[cfg(debug_assertions)]
                eprintln!("Failed to enable clipboard set string hook: {:?}", _e);
            });
            CLIPBOARD_SET_HOOK = Some(hook);
        }
    }

    #[cfg(debug_assertions)]
    println!("Bug fixes initialized");
}
