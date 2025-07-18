/// Main entry point and initialization logic for the DLL
use crate::game_client::addresses;

// Lua function to expose debugbreak functionality
#[allow(dead_code)]
unsafe extern "C" fn lua_debugbreak(_l: *mut crate::game_client::LuaState) -> i32 {
    #[cfg(debug_assertions)]
    {
        use winapi::um::debugapi::IsDebuggerPresent;
        if IsDebuggerPresent() != 0 {
            // Use inline assembly for debugger breakpoint
            #[cfg(target_arch = "x86_64")]
            std::arch::asm!("int3");
            #[cfg(target_arch = "x86")]
            std::arch::asm!("int3");
        }
    }
    0
}

// Lua function to register AwesomeWotlk
unsafe extern "C" fn lua_open_awesome_wotlk(_l: *mut crate::game_client::LuaState) -> i32 {
    // This would use proper Lua C API bindings
    // For now, just a placeholder - in real implementation we'd need to:
    // 1. Push number 1.0 onto stack
    // 2. Set global variable "AwesomeWotlk" = 1.0
    // 3. In debug builds, register debugbreak function

    #[cfg(debug_assertions)]
    {
        // Register debugbreak function in debug builds
        // lua_pushcfunction(l, lua_debugbreak);
        // lua_setglobal(l, "debugbreak");
    }

    0
}

/// Main initialization function called when DLL is attached
pub fn on_attach() {
    #[cfg(debug_assertions)]
    {
        // In debug builds, could pause or allocate console
        // std::process::Command::new("cmd").arg("/c").arg("pause").status().ok();
    }

    unsafe {
        // Apply direct memory patches
        apply_memory_patches();

        // Initialize all modules
        initialize_modules();

        // Register Lua library
        crate::hooks::frame_xml::register_lua_lib(lua_open_awesome_wotlk);
    }
}

unsafe fn apply_memory_patches() {
    // Apply memory patches directly to WoW client memory
    // Invalid function pointer hack
    *(addresses::INVALID_FUNCTION_PTR_1 as *mut u32) = 1;
    *(addresses::INVALID_FUNCTION_PTR_2 as *mut u32) = 0x7FFFFFFF;

    // Set TOS and EULA accepted
    *(addresses::TOS_ACCEPTED as *mut u32) = 1;
    *(addresses::EULA_ACCEPTED as *mut u32) = 1;
}

unsafe fn initialize_modules() {
    // Initialize all modules in order
    // Note: In the C++ version, these were wrapped in Detour transactions
    // In Rust, we'll use the retour crate for function hooking

    crate::hooks::initialize();
    crate::bug_fixes::initialize();
    crate::command_line::initialize();
    crate::inventory::initialize();
    crate::name_plates::initialize();
    crate::misc::initialize();
    crate::unit_api::initialize();
}
