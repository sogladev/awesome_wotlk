/// Name plates functionality and improvements
use crate::game_client::{addresses, CVar, CVarFlags, Guid, LuaState, NamePlateVars};
use crate::hooks::{frame_script, frame_xml};
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref NAMEPLATE_VARS: Mutex<NamePlateVars> = Mutex::new(NamePlateVars {
        nameplates: Vec::new(),
    });
}

// CVar for nameplate distance
#[allow(dead_code)]
static mut NAMEPLATE_DISTANCE_CVAR: *mut CVar = std::ptr::null_mut();

// Token functions for nameplate access
unsafe extern "C" fn get_nameplate_guid(id: i32) -> Guid {
    let vars = NAMEPLATE_VARS.lock().unwrap();
    if id >= 0 && (id as usize) < vars.nameplates.len() {
        vars.nameplates[id as usize].guid
    } else {
        0
    }
}

unsafe extern "C" fn get_nameplate_id(guid: Guid) -> i32 {
    if guid == 0 {
        return -1;
    }

    let vars = NAMEPLATE_VARS.lock().unwrap();
    for (i, entry) in vars.nameplates.iter().enumerate() {
        if entry.guid == guid {
            return i as i32;
        }
    }
    -1
}

// CVar handler for nameplate distance
unsafe extern "C" fn cvar_handler_nameplate_distance(
    _cvar: *mut CVar,
    _name: *const i8,
    value: *const i8,
    _user_data: *mut std::ffi::c_void,
) -> i32 {
    if value.is_null() {
        return 0;
    }

    let c_str = std::ffi::CStr::from_ptr(value);
    if let Ok(s) = c_str.to_str() {
        if let Ok(f) = s.parse::<f32>() {
            let distance = if f > 0.0 { f } else { 41.0 };

            *(addresses::NAMEPLATE_DISTANCE as *mut f32) = distance * distance;
            return 1;
        }
    }
    0
}

// Helper: push a nameplate frame onto the Lua stack (stub, needs real impl)
unsafe fn lua_pushframe(_l: *mut LuaState, _frame: usize) {
    // TODO: Implement pushing a WoW UI frame pointer onto the Lua stack
    // This is a stub for now
}

// C_NamePlate.GetNamePlates() - returns a Lua table of visible nameplates
unsafe extern "C" fn c_nameplate_get_nameplates(l: *mut LuaState) -> i32 {
    // Create a new Lua table
    // TODO: Replace with actual Lua C API call
    // e.g. mlua::ffi::lua_createtable(l, 0, 0);
    // For now, just a stub
    // let lua = mlua::Lua::init_from_ptr(l); // if using mlua
    // let table = lua.create_table().unwrap();
    // ...
    // table.set(...)
    // table.to_lua(l)
    //
    // Instead, we just return 0 (empty stack)
    0
}

// C_NamePlate.GetNamePlateForUnit(token) - returns the frame for a given unit token
unsafe extern "C" fn c_nameplate_get_nameplate_for_unit(_l: *mut LuaState) -> i32 {
    // TODO: Implement lookup by unit token and push frame
    // For now, just return 0 (nil)
    0
}

// Lua library open function for C_NamePlate
unsafe extern "C" fn lua_openlib_nameplates(l: *mut LuaState) -> i32 {
    // TODO: Register methods as a Lua table: C_NamePlate = { GetNamePlates = ..., GetNamePlateForUnit = ... }
    // This would use the Lua C API or mlua FFI
    // For now, stub
    0
}

/// Initialize nameplate functionality
pub fn initialize() {
    // Register nameplate token
    frame_script::register_token_multi("nameplate", get_nameplate_guid, get_nameplate_id);

    // Register nameplate distance CVar
    frame_xml::register_cvar(
        "nameplateDistance",
        "Maximum distance for nameplate visibility",
        CVarFlags::Archive,
        "41",
        Some(cvar_handler_nameplate_distance),
    );

    // Register Lua library for C_NamePlate
    crate::hooks::register_lua_lib(lua_openlib_nameplates);
    #[cfg(debug_assertions)]
    println!("Nameplate module initialized");
}
