# Awesome WotLK Patcher (Rust)

**⚠️ EXPERIMENTAL PORT - WORK IN PROGRESS ⚠️**

This is an experimental attempt to port the original C++ Awesome WotLK project to Rust. The goal was to enable building both the patcher and DLL on Linux using cross-compilation, eliminating the need for Windows development environments.

## Current Status

- ✅ **Patcher utility**: Works! Successfully builds and runs on Linux with MinGW
- ❌ **DLL library**: Has compilation issues with Detours dependency and Rust FFI boundaries
- 🔄 **This is a learning/experimentation project** - not production ready

The original C++ version remains the stable, recommended implementation.

To only build the patcher. Release mode will use the `AwesomeWotlkLib.dll` from the main repo.
```
cargo build --bin patcher --release --locked
``

## Features

- **Cross-platform development**: Build on Linux, Windows, or macOS
- **Safe and reliable**: Rust's memory safety prevents common C++ pitfalls
- **Modern error handling**: Clear error messages with full context
- **CLI interface**: Command-line arguments with proper help text
- **Lightweight**: Optimized for small binary size
- **No Windows SDK required**: Pure Rust with minimal dependencies

## Building

### Prerequisites

Install Rust from [rustup.rs](https://rustup.rs/)

### Native Build (Linux/Windows/macOS)

```bash
# Debug build
cargo build

# Optimized release build
cargo build --release
```

### Cross-compilation for Windows (from Linux)

```bash
# Install Windows target
rustup target add x86_64-pc-windows-gnu

# Install MinGW cross-compiler (Ubuntu/Debian)
sudo apt install gcc-mingw-w64-x86-64

# Or on Fedora
sudo dnf install mingw64-gcc

# Build for Windows
cargo build --release --target x86_64-pc-windows-gnu
```

## Usage

```bash
# Auto-detect WoW executable in current directory
./patcher

# Specify executable path
./patcher /path/to/Wow.exe

# Skip DLL copying
./patcher --skip-dll-copy

# Verbose output
./patcher --verbose

# Show help
./patcher --help
```

## What it does

1. **Finds WoW executable**: Automatically detects `Wow.exe`, `WowCircle.exe`, or `run.exe`
2. **Applies binary patches**: Injects code at specific virtual addresses to load the AwesomeWotlk library
3. **Copies DLL**: Places `AwesomeWotlkLib.dll` in the game directory if available (Windows-only DLL)
4. **Shows results**: User-friendly success/error messages (Windows message boxes on Windows)

## Patches Applied

- **0x004DCCF0**: `lua_ScanDllStart` hook - Returns 0 to bypass DLL scanning
- **0x004E5CB0**: `ScanDllStart` injection point - Loads AwesomeWotlkLib.dll and calls initialization
- **0x0040B7D0**: `StartAddress` hook - Jumps to injection point

## Advantages over C++ version

- **No Windows SDK dependency**: Works on any platform with Rust
- **Memory safety**: Eliminates buffer overflows, null pointer dereferences
- **Better error handling**: Detailed error context instead of Windows error codes
- **Modern tooling**: Cargo for dependencies, testing, documentation
- **Smaller binary**: Optimized release builds (~200KB vs 1.4MB)
- **Cross-compilation**: Build Windows binaries from Linux without Wine/VM

## Project Structure

```
src/
├── main.rs          # Entry point and CLI handling
├── patches.rs       # Patch definitions and hex data
├── pe_patcher.rs    # PE file parsing and patching logic
├── ui.rs           # User interface (message boxes, console output)
└── lib/            # AwesomeWotlkLib.dll library source
    ├── mod.rs       # DLL entry point and main exports
    ├── entry.rs     # Initialization and module loading
    ├── hooks.rs     # Function hooking system
    ├── game_client.rs # WoW client API definitions
    ├── bug_fixes.rs # Memory patches and bug fixes
    ├── command_line.rs # Command line argument handling
    ├── inventory.rs # Inventory-related functionality
    ├── misc.rs      # Miscellaneous features
    ├── name_plates.rs # Nameplate customization
    ├── unit_api.rs  # Unit/player API extensions
    └── utils.rs     # Utility functions (clipboard, memory)
```

## DLL Architecture

The project builds two components:

1. **Patcher** (`patcher.exe` / `patcher-linux`): Patches the WoW executable and copies the DLL
2. **Library** (`AwesomeWotlkLib.dll`): Windows-only DLL that enhances WoW gameplay

The DLL is **Windows-only** and works seamlessly on both:
- Native Windows with WoW.exe
- Linux with WoW.exe running under Wine

This design choice ensures maximum compatibility since WoW is a Windows game that uses Windows APIs, regardless of the host OS.

## Testing

```bash
# Run unit tests
cargo test

# Run with verbose output
cargo test -- --nocapture
```

## License

[Add your license here]
