//! Miscellaneous improvements and fixes

use crate::lib::game_client::*;
use std::ffi::{CStr, CString};

// Static storage for requested interaction
static mut REQUESTED_INTERACTION: Guid = 0;

// Static storage for camera FOV CVar
static mut CVAR_CAMERA_FOV: *mut CVar = std::ptr::null_mut();

/// Lua function: Flash the WoW window
/// # Safety
/// This function is unsafe as it interfaces with WoW's window system
unsafe extern "C" fn lua_flash_window(lua_state: *mut LuaState) -> i32 {
    let hwnd = game_functions::get_game_window();
    if !hwnd.is_null() {
        use windows_sys::Win32::UI::WindowsAndMessaging::FlashWindow;
        FlashWindow(hwnd as isize, 0); // FALSE parameter
    }
    0
}

/// Lua function: Check if WoW window is focused
/// # Safety
/// This function is unsafe as it interfaces with WoW's window system
unsafe extern "C" fn lua_is_window_focused(lua_state: *mut LuaState) -> i32 {
    let hwnd = game_functions::get_game_window();
    if hwnd.is_null() {
        return 0;
    }

    use windows_sys::Win32::UI::WindowsAndMessaging::GetForegroundWindow;
    if GetForegroundWindow() != hwnd as isize {
        return 0;
    }

    lua_pushnumber(lua_state, 1.0);
    1
}

/// Lua function: Focus the WoW window
/// # Safety
/// This function is unsafe as it interfaces with WoW's window system
unsafe extern "C" fn lua_focus_window(lua_state: *mut LuaState) -> i32 {
    let hwnd = game_functions::get_game_window();
    if !hwnd.is_null() {
        use windows_sys::Win32::UI::WindowsAndMessaging::SetForegroundWindow;
        SetForegroundWindow(hwnd as isize);
    }
    0
}

/// Lua function: Copy text to clipboard
/// # Safety
/// This function is unsafe as it interfaces with WoW's clipboard system
unsafe extern "C" fn lua_copy_to_clipboard(lua_state: *mut LuaState) -> i32 {
    let str_ptr = luaL_checkstring(lua_state, 1);
    if !str_ptr.is_null() {
        let c_str = CStr::from_ptr(str_ptr);
        if !c_str.to_bytes().is_empty() {
            game_functions::copy_to_clipboard(str_ptr);
        }
    }
    0
}

/// Process queued interaction requests
/// # Safety
/// This function is unsafe as it interfaces with WoW's object system
unsafe fn process_queued_interaction() {
    if REQUESTED_INTERACTION == 0 {
        return;
    }

    let object = object_manager::get_object_ptr(
        REQUESTED_INTERACTION,
        TypeMask::GameObject as u32 | TypeMask::Unit as u32,
    );
    if !object.is_null() {
        // Simulate right click on object
        object_manager::unit_right_click_by_guid(REQUESTED_INTERACTION);
    }

    REQUESTED_INTERACTION = 0;
}

/// Check if a game object type is good for interaction
fn is_good_object(game_object_type: u8) -> bool {
    matches!(
        game_object_type,
        GameObjectType::Door as u8
            | GameObjectType::Button as u8
            | GameObjectType::QuestGiver as u8
            | GameObjectType::Chest as u8
            | GameObjectType::Binder as u8
            | GameObjectType::Trap as u8
            | GameObjectType::Chair as u8
            | GameObjectType::SpellFocus as u8
            | GameObjectType::Goober as u8
            | GameObjectType::FishingNode as u8
            | GameObjectType::SummoningRitual as u8
            | GameObjectType::Mailbox as u8
            | GameObjectType::MeetingStone as u8
            | GameObjectType::FlagStand as u8
            | GameObjectType::FlagDrop as u8
            | GameObjectType::BarberChair as u8
            | GameObjectType::GuildBank as u8
            | GameObjectType::TrapDoor as u8
    )
}

/// Lua function: Queue an interaction with the nearest suitable object
/// # Safety
/// This function is unsafe as it interfaces with WoW's object system
unsafe extern "C" fn lua_queue_interact(lua_state: *mut LuaState) -> i32 {
    if !game_functions::is_in_world() {
        return 0;
    }

    let mut candidate = 0u64;
    let mut best_distance = 3000.0f32;

    let player = get_player();
    if player.is_null() {
        return 0;
    }

    // Get player GUID and position for comparison
    let player_guid = (*(*player).entry).guid;

    // Enumerate objects to find the best interaction target
    object_manager::enum_objects_extended(|guid| {
        if guid == player_guid {
            return true;
        }

        let object = object_manager::get_object_ptr(
            guid,
            TypeMask::GameObject as u32 | TypeMask::Unit as u32,
        );
        if object.is_null() {
            return true;
        }

        // Calculate distance using WoW's distance calculation
        // For now, use a simplified distance check
        let distance = 10.0f32; // Would need actual 3D distance calculation

        if distance > 20.0 || distance == 0.0 || distance > best_distance {
            return true;
        }

        // Get object entry for type checking
        let object_entry = (*object).entry;
        if object_entry.is_null() {
            return true;
        }

        let type_id = (*object_entry).type_id;
        let mut is_suitable = false;

        match type_id {
            3 => {
                // TYPEID_UNIT
                // For units, check if they're lootable, skinnable, or interactive
                let unit_data_ptr = object_entry as *const UnitData;
                let unit_data = &*unit_data_ptr;

                // Check various interaction possibilities
                if unit_data.is_lootable()
                    || unit_data.is_quest_giver()
                    || unit_data.is_vendor()
                    || unit_data.is_trainer()
                    || (unit_data.flags & UnitFlags::Skinnable != UnitFlags::ServerControlled)
                {
                    is_suitable = true;
                }
            }
            5 => {
                // TYPEID_GAMEOBJECT
                // For game objects, check if it's a good interaction type
                // This would require reading the game object's bytes to get the type
                // For now, assume it's suitable if we got this far
                is_suitable = true;
            }
            _ => {
                // Other object types not suitable for interaction
                is_suitable = false;
            }
        }

        if is_suitable {
            candidate = guid;
            best_distance = distance;
        }

        true
    });

    if candidate != 0 {
        REQUESTED_INTERACTION = candidate;
    }

    0
}

/// Parse FOV value from string
fn parse_fov(value: &str) -> f64 {
    const M_PI: f64 = 3.14159265358979323846;
    let parsed_value = value.parse::<i32>().unwrap_or(100);
    let clamped_value = parsed_value.max(1).min(200);
    M_PI / 200.0 * (clamped_value as f64)
}

/// CVar handler for camera FOV changes
/// # Safety
/// This function is unsafe as it interfaces with WoW's camera system
unsafe extern "C" fn cvar_handler_camera_fov(
    _cvar: *mut CVar,
    _prev_val: *const i8,
    new_val: *const i8,
    _user_data: *mut std::ffi::c_void,
) -> i32 {
    if !new_val.is_null() {
        let c_str = CStr::from_ptr(new_val);
        if let Ok(val_str) = c_str.to_str() {
            let fov = parse_fov(val_str);
            let camera = game_functions::get_active_camera();
            if !camera.is_null() {
                (*camera).fov_in_radians = fov as f32;
            }
        }
    }
    1
}

/// Register Lua functions with the game
/// # Safety
/// This function is unsafe as it interfaces with WoW's Lua system
unsafe extern "C" fn lua_open_misc_lib(lua_state: *mut LuaState) -> i32 {
    // Register individual functions
    lua_pushcfunction(lua_state, lua_flash_window);
    lua_setglobal(lua_state, b"FlashWindow\0".as_ptr() as *const i8);

    lua_pushcfunction(lua_state, lua_is_window_focused);
    lua_setglobal(lua_state, b"IsWindowFocused\0".as_ptr() as *const i8);

    lua_pushcfunction(lua_state, lua_focus_window);
    lua_setglobal(lua_state, b"FocusWindow\0".as_ptr() as *const i8);

    lua_pushcfunction(lua_state, lua_copy_to_clipboard);
    lua_setglobal(lua_state, b"CopyToClipboard\0".as_ptr() as *const i8);

    lua_pushcfunction(lua_state, lua_queue_interact);
    lua_setglobal(lua_state, b"QueueInteract\0".as_ptr() as *const i8);

    // Register the update callback for processing queued interactions
    // This would need to be registered with the hooks system
    // Hooks::FrameScript::registerOnUpdate(process_queued_interaction);

    0
}

/// Initialize miscellaneous functionality
pub fn initialize() {
    unsafe {
        // Register camera FOV CVar
        let cvar_name = CString::new("cameraFov").unwrap();
        let cvar_desc = CString::new("Field of view for camera").unwrap();
        let default_val = CString::new("100").unwrap();

        CVAR_CAMERA_FOV = console::register_cvar(
            cvar_name.as_ptr(),
            cvar_desc.as_ptr(),
            console::CVarFlags::None as u32,
            default_val.as_ptr(),
            cvar_handler_camera_fov,
        );

        // Register Lua library
        // This would need to be registered with the hooks system
        // Hooks::FrameXML::registerLuaLib(lua_open_misc_lib);

        // Hook camera initialization function (would need detours equivalent)
        // DetourAttach(&(LPVOID&)Camera_Initialize_orig, Camera_Initialize_hk);
    }

    #[cfg(debug_assertions)]
    println!("Misc module initialized with advanced functionality");
}
