/// Function hooking and registration system
/// This module provides the infrastructure for hooking into WoW's functions
use crate::game_client::{CVar, CVarFlags, DummyCallback, Guid, LuaCFunction};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

// Global storage for registered callbacks and hooks
lazy_static! {
    static ref FRAME_SCRIPT_TOKENS: Mutex<HashMap<String, TokenCallbacks>> =
        Mutex::new(HashMap::new());
    static ref FRAME_XML_EVENTS: Mutex<Vec<String>> = Mutex::new(Vec::new());
    static ref FRAME_XML_CVARS: Mutex<Vec<CvarRegistration>> = Mutex::new(Vec::new());
    static ref LUA_LIBS: Mutex<Vec<LuaCFunction>> = Mutex::new(Vec::new());
    static ref ON_UPDATE_CALLBACKS: Mutex<Vec<DummyCallback>> = Mutex::new(Vec::new());
}

// Callback types for tokens
pub struct TokenCallbacks {
    pub single_guid_getter: Option<unsafe extern "C" fn() -> Guid>,
    pub single_id_getter: Option<unsafe extern "C" fn(Guid) -> bool>,
    pub multi_guid_getter: Option<unsafe extern "C" fn(i32) -> Guid>,
    pub multi_id_getter: Option<unsafe extern "C" fn(Guid) -> i32>,
}

pub struct CvarRegistration {
    pub name: String,
    pub description: String,
    pub flags: CVarFlags,
    pub initial_value: String,
    pub handler:
        Option<unsafe extern "C" fn(*mut CVar, *const i8, *const i8, *mut std::ffi::c_void) -> i32>,
}

/// FrameScript namespace - handles UI tokens and updates
pub mod frame_script {
    use super::*;

    pub fn register_token_single(
        token: &str,
        get_guid: unsafe extern "C" fn() -> Guid,
        get_id: unsafe extern "C" fn(Guid) -> bool,
    ) {
        let mut tokens = FRAME_SCRIPT_TOKENS.lock().unwrap();
        tokens.insert(
            token.to_string(),
            TokenCallbacks {
                single_guid_getter: Some(get_guid),
                single_id_getter: Some(get_id),
                multi_guid_getter: None,
                multi_id_getter: None,
            },
        );
    }

    pub fn register_token_multi(
        token: &str,
        get_guid: unsafe extern "C" fn(i32) -> Guid,
        get_id: unsafe extern "C" fn(Guid) -> i32,
    ) {
        let mut tokens = FRAME_SCRIPT_TOKENS.lock().unwrap();
        tokens.insert(
            token.to_string(),
            TokenCallbacks {
                single_guid_getter: None,
                single_id_getter: None,
                multi_guid_getter: Some(get_guid),
                multi_id_getter: Some(get_id),
            },
        );
    }

    pub fn register_on_update(func: DummyCallback) {
        let mut callbacks = ON_UPDATE_CALLBACKS.lock().unwrap();
        callbacks.push(func);
    }
}

/// FrameXML namespace - handles events, CVars, and Lua libraries
pub mod frame_xml {
    use super::*;

    pub fn register_event(event: &str) {
        let mut events = FRAME_XML_EVENTS.lock().unwrap();
        events.push(event.to_string());
    }

    pub fn register_cvar(
        name: &str,
        description: &str,
        flags: CVarFlags,
        initial_value: &str,
        handler: Option<
            unsafe extern "C" fn(*mut CVar, *const i8, *const i8, *mut std::ffi::c_void) -> i32,
        >,
    ) {
        let mut cvars = FRAME_XML_CVARS.lock().unwrap();
        cvars.push(CvarRegistration {
            name: name.to_string(),
            description: description.to_string(),
            flags,
            initial_value: initial_value.to_string(),
            handler,
        });
    }

    pub fn register_lua_lib(func: LuaCFunction) {
        let mut libs = LUA_LIBS.lock().unwrap();
        libs.push(func);
    }
}

/// GlueXML namespace - handles login screen functionality
pub mod glue_xml {
    use super::*;
    use std::sync::Mutex;

    static POST_LOAD_CALLBACKS: Mutex<Vec<DummyCallback>> = Mutex::new(Vec::new());
    static CHAR_ENUM_CALLBACKS: Mutex<Vec<DummyCallback>> = Mutex::new(Vec::new());

    pub fn register_post_load(func: DummyCallback) {
        if let Ok(mut callbacks) = POST_LOAD_CALLBACKS.lock() {
            callbacks.push(func);
        }
    }

    pub fn register_char_enum(func: DummyCallback) {
        if let Ok(mut callbacks) = CHAR_ENUM_CALLBACKS.lock() {
            callbacks.push(func);
        }
    }
}

/// Initialize the hooks system
pub fn initialize() {
    // This would set up the basic hooking infrastructure
    // In the C++ version, this was handled by Detours
    // In Rust, we'll use the retour crate for function hooking

    // TODO: Set up hooks for:
    // - Lua function registration
    // - Event system integration
    // - CVar registration
    // - Token system integration

    #[cfg(debug_assertions)]
    println!("Hooks system initialized");
}
