#![cfg(target_os = "windows")]

use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE};
use winapi::um::winnt::DLL_PROCESS_ATTACH;

// Module declarations
mod bug_fixes;
mod command_line;
mod entry;
mod game_client;
mod hooks;
mod inventory;
mod misc;
mod name_plates;
mod unit_api;
mod utils;
mod enums;

// Re-export main modules
pub use entry::*;
pub use game_client::*;
pub use hooks::*;

// DLL entry point
#[no_mangle]
#[allow(non_snake_case)]
pub extern "system" fn DllMain(_hinstance: HINSTANCE, reason: DWORD, _reserved: LPVOID) -> BOOL {
    match reason {
        DLL_PROCESS_ATTACH => {
            // Initialize the library when DLL is loaded
            entry::on_attach();
            TRUE
        }
        _ => TRUE,
    }
}
