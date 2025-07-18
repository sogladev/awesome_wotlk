/// Game client constants, types and function definitions
/// This module contains the WoW client memory addresses and data structures
use std::ffi::c_void;

use crate::enums::*;

// Basic types used by WoW client
pub type Guid = u64;
pub type LuaNumber = f64;

// Forward declarations for opaque structures
#[repr(C)]
pub struct LuaState {
    _private: [u8; 0],
}

#[repr(C)]
pub struct WorldFrame {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Camera {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Status {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Frame {
    _private: [u8; 0],
}

#[repr(C)]
pub struct XMLObject {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Object {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ObjectVtbl {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ObjectEntry {
    _private: [u8; 0x18], // sizeof(ObjectEntry) == 0x18
}

#[repr(C)]
pub struct Unit {
    _private: [u8; 0],
}

#[repr(C)]
pub struct UnitVtbl {
    _private: [u8; 0],
}

#[repr(C)]
pub struct UnitEntry {
    _private: [u8; 0x250], // sizeof(UnitEntry) == 0x250
}

#[repr(C)]
pub struct Player {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PlayerVtbl {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PlayerEntry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PlayerQuest {
    _private: [u8; 0x14], // sizeof(PlayerQuest) == 0x14
}

#[repr(C)]
pub struct PlayerVisibleItem {
    _private: [u8; 0x8], // sizeof(PlayerVisibleItem) == 0x8
}

// Vector types used in game
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vec2D<T> {
    pub x: T,
    pub y: T,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vec3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Vec4D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub o: T,
}

pub type VecXYZ = Vec3D<f32>;

use std::ops::Sub;

impl Sub for VecXYZ {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl VecXYZ {
    pub fn distance(&self, other: &VecXYZ) -> f32 {
        let diff = *self - *other;
        (diff.x * diff.x + diff.y * diff.y + diff.z * diff.z).sqrt()
    }
}
// Console CVar structure
#[repr(C)]
pub struct CVar {
    pub hash: u32,
    pub gap4: [u32; 4],
    pub name: *const i8,
    pub field18: u32,
    pub flags: CVarFlags,
    pub field20: u32,
    pub field24: u32,
    pub v_str: *const i8,
    pub field2c: [u32; 5],
    pub v_bool: u32,
    pub gap44: [u32; 9],
    pub handler: CVarHandler,
    pub user_data: *mut c_void,
}

// CVar handler function type
pub type CVarHandler = unsafe extern "C" fn(
    cvar: *mut CVar,
    prev_val: *const i8,
    new_val: *const i8,
    user_data: *mut c_void,
) -> i32;

// Game function types for object management
pub type EnumObjectsFunc = unsafe extern "C" fn(guid: Guid, user_data: *mut c_void) -> i32;

// Character data structure for login screen
#[repr(C, packed)]
pub struct CharData {
    pub guid: Guid,
    pub name: [i8; 48],
    pub map: i32,
    pub zone: i32,
    pub guild_id: i32,
    pub pos: VecXYZ,
    pub display_info_id: [i32; 23],
    pub inventory_type: [i32; 23],
    pub enchant_visual: [i32; 23],
    pub pet_display_id: i32,
    pub pet_level: i32,
    pub pet_family: i32,
    pub flags: i32,
    pub char_customize_flags: i32,
    pub race: i8,
    pub class: i8,
    pub gender: i8,
    pub skin: i8,
    pub face: i8,
    pub hair_style: i8,
    pub hair_color: i8,
    pub facial_color: i8,
    pub level: i8,
    pub first_login: i8,
    pub gap: [i8; 6],
}

#[repr(C)]
pub struct CharVector {
    pub reserved: i32,
    pub size: i32,
    pub buf: *mut CharData,
    pub field_c: i32,
}

// Camera structure
#[repr(C)]
pub struct CameraVtbl {
    _private: [u8; 0],
}

#[repr(C)]
pub struct CameraStruct {
    pub vmt: *mut CameraVtbl,
    pub field4: u32,
    pub pos: VecXYZ,
    pub gap14: [u32; 11],
    pub fov_in_radians: f32,
}

// Game function addresses and types
pub mod functions {
    use super::*;

    // Object manager functions
    pub type GetPlayerFunc = unsafe extern "C" fn() -> *mut Player;
    pub type GetObjectFunc = unsafe extern "C" fn(guid: Guid, flags: u32) -> *mut Object;
    pub type GetObjectPtrFunc =
        unsafe extern "C" fn(guid: Guid, type_mask: u32, file: *const i8, line: i32) -> *mut c_void;

    // Console functions
    pub type RegisterCVarFunc = unsafe extern "C" fn(
        name: *const i8,
        desc: *const i8,
        flags: u32,
        default_val: *const i8,
        callback: CVarHandler,
        a6: i32,
        a7: i32,
        a8: i32,
        a9: i32,
    ) -> *mut CVar;
    pub type GetCVarFunc = unsafe extern "C" fn(name: *const i8) -> *mut CVar;

    // Utility functions
    pub type IsInWorldFunc = unsafe extern "C" fn() -> bool;
    pub type GetGameWindowFunc = unsafe extern "C" fn() -> *mut c_void; // HWND on Windows
}

// Additional game client constants
pub mod game_constants {
    // Game state addresses (these are for WoW 3.3.5a - version specific)
    pub const IS_IN_WORLD_ADDR: usize = 0x00BD0792;
    pub const GAME_WINDOW_ADDR: usize = 0x00D41620;
    pub const TARGET_GUID_ADDR: usize = 0x00BD07B0;
    pub const LUA_ERROR_HANDLER_ADDR: usize = 0x00AF576C;

    // Object manager function addresses
    pub const GET_PLAYER_ADDR: usize = 0x004038F0;
    pub const GET_OBJECT_ADDR: usize = 0x004D4DB0;
    pub const ENUM_OBJECTS_ADDR: usize = 0x004D4B30;

    // Console function addresses
    pub const REGISTER_CVAR_ADDR: usize = 0x00767FC0;
    pub const GET_CVAR_ADDR: usize = 0x00767460;

    // Lua state function addresses
    pub const GET_LUA_STATE_ADDR: usize = 0x00817DB0;
}

// Convenience functions for accessing game state
/// # Safety
/// This function reads from a hardcoded memory address that may be invalid
/// for different WoW client versions.
pub unsafe fn is_in_world() -> bool {
    *(game_constants::IS_IN_WORLD_ADDR as *const bool)
}

/// # Safety
/// This function reads from a hardcoded memory address that may be invalid
/// for different WoW client versions.
pub unsafe fn get_target_guid() -> Guid {
    *(game_constants::TARGET_GUID_ADDR as *const Guid)
}

/// # Safety
/// This function calls the WoW client's GetLuaState function at a hardcoded address.
/// The address may be invalid for different WoW client versions.
pub unsafe fn get_lua_state_from_game() -> *mut LuaState {
    let get_lua_state: unsafe extern "C" fn() -> *mut LuaState =
        std::mem::transmute(game_constants::GET_LUA_STATE_ADDR);
    get_lua_state()
}

// NamePlate related structures
#[repr(C)]
pub struct NamePlateEntry {
    pub guid: Guid,
    pub flags: u32,
    pub nameplate: *mut Frame, // Reference to the UI frame
    pub update_id: u32,
}

pub struct NamePlateVars {
    pub nameplates: Vec<NamePlateEntry>,
    pub update_id: u32,
}

// Function type definitions
pub type LuaCFunction = unsafe extern "C" fn(*mut LuaState) -> i32;
pub type DummyCallback = unsafe extern "C" fn();

// Additional function type for Lua registration
#[repr(C)]
pub struct LuaLReg {
    pub name: *const i8,
    pub func: LuaCFunction,
}

// Helper function to get current Lua state (would need to be hooked from WoW)
pub fn get_lua_state() -> *mut LuaState {
    // This would need to be populated by hooking WoW's Lua state
    // In practice, use get_lua_state_from_game() once hooked
    std::ptr::null_mut()
}

// Object manager helper functions
/// # Safety
/// This function calls WoW's object enumeration function at a hardcoded address.
/// The callback function is called for each object in the world.
pub unsafe fn enum_objects(callback: EnumObjectsFunc, user_data: *mut c_void) -> i32 {
    let enum_func: unsafe extern "C" fn(EnumObjectsFunc, *mut c_void) -> i32 =
        std::mem::transmute(game_constants::ENUM_OBJECTS_ADDR);
    enum_func(callback, user_data)
}

/// # Safety
/// This function calls WoW's GetPlayer function at a hardcoded address.
pub unsafe fn get_player() -> *mut Player {
    let get_player_func: functions::GetPlayerFunc =
        std::mem::transmute(game_constants::GET_PLAYER_ADDR);
    get_player_func()
}

/// # Safety
/// This function calls WoW's GetObject function at a hardcoded address.
pub unsafe fn get_object(guid: Guid, flags: u32) -> *mut Object {
    let get_object_func: functions::GetObjectFunc =
        std::mem::transmute(game_constants::GET_OBJECT_ADDR);
    get_object_func(guid, flags)
}

// String and GUID conversion utilities
/// Convert a GUID to a hex string representation
/// # Safety
/// The buffer must be at least 24 bytes long to hold the hex string.
pub unsafe fn guid_to_hex_string(guid: Guid, buffer: *mut i8) {
    // This would call WoW's Guid2HexString function at address 0x0074D0D0
    let guid_to_hex: unsafe extern "C" fn(Guid, *mut i8) = std::mem::transmute(0x0074D0D0usize);
    guid_to_hex(guid, buffer);
}

/// Convert a hex string to a GUID
/// # Safety
/// The string pointer must be valid and null-terminated.
pub unsafe fn hex_string_to_guid(hex_str: *const i8) -> Guid {
    // This would call WoW's HexString2Guid function at address 0x0074D120
    let hex_to_guid: unsafe extern "C" fn(*const i8) -> Guid = std::mem::transmute(0x0074D120usize);
    hex_to_guid(hex_str)
}

/// Get GUID by unit ID string (like "player", "target", "focus", etc.)
/// # Safety
/// The string pointer must be valid and null-terminated.
pub unsafe fn get_guid_by_unit_id(unit_id: *const i8) -> Guid {
    // This would call WoW's GetGuidByUnitID function at address 0x0060C1C0
    let get_guid_func: unsafe extern "C" fn(*const i8) -> Guid =
        std::mem::transmute(0x0060C1C0usize);
    get_guid_func(unit_id)
}

// Console/CVar helper functions
/// # Safety
/// This function calls WoW's RegisterCVar function at a hardcoded address.
/// All string pointers must be valid and null-terminated.
pub unsafe fn register_cvar(
    name: *const i8,
    desc: *const i8,
    flags: u32,
    default_val: *const i8,
    callback: CVarHandler,
) -> *mut CVar {
    let register_func: functions::RegisterCVarFunc =
        std::mem::transmute(game_constants::REGISTER_CVAR_ADDR);
    register_func(name, desc, flags, default_val, callback, 0, 0, 0, 0)
}

/// # Safety
/// This function calls WoW's GetCVar function at a hardcoded address.
/// The name pointer must be valid and null-terminated.
pub unsafe fn get_cvar(name: *const i8) -> *mut CVar {
    let get_cvar_func: functions::GetCVarFunc = std::mem::transmute(game_constants::GET_CVAR_ADDR);
    get_cvar_func(name)
}

// Memory allocation using WoW's allocator
/// # Safety
/// This function directly calls the WoW client's memory allocator at a hardcoded address.
/// It's unsafe because:
/// - The memory address may be invalid for different WoW client versions
/// - The transmute to function pointer assumes the address contains valid executable code
/// - The allocated memory must be freed using WoW's corresponding free function
pub unsafe fn wow_alloc(size: usize) -> *mut c_void {
    let alloc_fn: unsafe extern "C" fn(usize) -> *mut c_void =
        std::mem::transmute(addresses::ALLOC_FUNCTION);
    alloc_fn(size)
}

#[cfg(test)]
mod tests {
    use std::mem;

    #[test]
    fn test_struct_sizes() {
        assert_eq!(
            mem::size_of::<ObjectEntry>(),
            0x18,
            "ObjectEntry size mismatch"
        );
        assert_eq!(
            mem::size_of::<UnitEntry>(),
            0x250,
            "UnitEntry size mismatch"
        );
        assert_eq!(
            mem::size_of::<PlayerQuest>(),
            0x14,
            "PlayerQuest size mismatch"
        );
        assert_eq!(
            mem::size_of::<PlayerVisibleItem>(),
            0x8,
            "PlayerVisibleItem size mismatch"
        );
    }
}
