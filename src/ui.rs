use anyhow::Result;

#[cfg(windows)]
use windows::Win32::UI::WindowsAndMessaging::{
    MessageBoxA, MB_ICONINFORMATION, MB_ICONWARNING, MB_OK,
};

#[cfg(windows)]
use windows::Win32::Foundation::HWND;

#[cfg(windows)]
use windows::core::PCSTR;

/// Show a success message to the user
pub fn show_success_message(title: &str, message: &str) -> Result<()> {
    #[cfg(windows)]
    {
        show_message_box(title, message, MB_ICONINFORMATION)?;
    }
    #[cfg(not(windows))]
    {
        println!("✅ {title}: {message}");
    }
    Ok(())
}

/// Show a warning message to the user
pub fn show_warning_message(title: &str, message: &str) -> Result<()> {
    #[cfg(windows)]
    {
        show_message_box(title, message, MB_ICONWARNING)?;
    }
    #[cfg(not(windows))]
    {
        println!("⚠️  {title}: {message}");
    }
    Ok(())
}

#[cfg(windows)]
fn show_message_box(
    title: &str,
    message: &str,
    icon: windows::Win32::UI::WindowsAndMessaging::MESSAGEBOX_STYLE,
) -> Result<()> {
    // Convert Rust strings to null-terminated C strings
    let title_cstr =
        std::ffi::CString::new(title).map_err(|_| anyhow::anyhow!("Invalid title string"))?;
    let message_cstr =
        std::ffi::CString::new(message).map_err(|_| anyhow::anyhow!("Invalid message string"))?;

    unsafe {
        MessageBoxA(
            HWND(std::ptr::null_mut()), // No parent window
            PCSTR(message_cstr.as_ptr() as *const u8),
            PCSTR(title_cstr.as_ptr() as *const u8),
            MB_OK | icon,
        );
    }

    Ok(())
}
