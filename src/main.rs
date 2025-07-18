use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};

mod patches;
mod pe_patcher;
mod ui;

use patches::PATCHES;
use pe_patcher::PePatcher;

#[derive(Parser)]
#[command(name = "awesome-wotlk-patcher")]
#[command(about = "World of Warcraft: Wrath of the Lich King patch utility")]
struct Args {
    /// Path to the WoW executable to patch
    #[arg(help = "Path to Wow.exe (optional - will auto-detect if not provided)")]
    exe_path: Option<PathBuf>,

    /// Skip copying the DLL
    #[arg(long, help = "Skip copying library to game directory")]
    skip_dll_copy: bool,

    /// Verbose output
    #[arg(short, long, help = "Enable verbose output")]
    verbose: bool,
}

// @todo: We can't use our own dll yet, so for release we use the one from main repo
#[cfg(debug_assertions)]
const AWESOME_WOTLK_DLL: &str = "awesome_wotlk_lib.dll";
#[cfg(not(debug_assertions))]
const AWESOME_WOTLK_DLL: &str = "AwesomeWotlkLib.dll";

const WOW_EXECUTABLES: &[&str] = &["Project-Epoch.exe", "Wow.exe"];

fn find_wow_executable() -> Result<PathBuf> {
    let current_dir = std::env::current_dir().context("Failed to get current directory")?;

    for exe_name in WOW_EXECUTABLES {
        let exe_path = current_dir.join(exe_name);
        if exe_path.is_file() {
            return Ok(exe_path);
        }
    }

    anyhow::bail!(
        "World of Warcraft executable not found!\n\
        You must do one of the following:\n\
        - Move patcher to folder with Wow.exe\n\
        - Drag and drop Wow.exe onto the patcher\n\
        - Use command line: patcher.exe <path-to-wow.exe>"
    );
}

fn copy_dll_to_game_dir(exe_path: &Path, app_dir: &Path) -> Result<bool> {
    let game_dir = exe_path.parent().context("Failed to get game directory")?;

    let dll_in_game = game_dir.join(AWESOME_WOTLK_DLL);
    let dll_in_app = app_dir.join(AWESOME_WOTLK_DLL);

    // If DLL already exists in game directory, we're good
    if dll_in_game.exists() {
        return Ok(true);
    }

    // Try to copy from app directory
    if dll_in_app.exists() {
        fs::copy(&dll_in_app, &dll_in_game)
            .with_context(|| format!("Failed to copy {AWESOME_WOTLK_DLL} to game directory"))?;
        return Ok(true);
    }

    Ok(false)
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Determine executable path
    let exe_path = match args.exe_path {
        Some(path) => {
            if !path.is_file() {
                anyhow::bail!("Specified executable does not exist: {}", path.display());
            }
            path
        }
        None => find_wow_executable()?,
    };

    if args.verbose {
        println!("Target executable: {}", exe_path.display());
        println!("Applying {} patches...", PATCHES.len());
    }

    // Apply patches
    let mut patcher = PePatcher::new(&exe_path)?;

    for (i, patch) in PATCHES.iter().enumerate() {
        if args.verbose {
            println!(
                "Applying patch {}/{}: {}",
                i + 1,
                PATCHES.len(),
                patch.description
            );
        }

        patcher
            .apply_patch(patch)
            .with_context(|| format!("Failed to apply patch: {}", patch.description))?;
    }

    patcher
        .save()
        .context("Failed to save patched executable")?;

    // Handle DLL copying
    let app_dir = std::env::current_exe()
        .context("Failed to get current executable path")?
        .parent()
        .context("Failed to get application directory")?
        .to_path_buf();

    let dll_copied = if args.skip_dll_copy {
        exe_path.parent().unwrap().join(AWESOME_WOTLK_DLL).exists()
    } else {
        copy_dll_to_game_dir(&exe_path, &app_dir)?
    };

    // Show result message
    if dll_copied {
        ui::show_success_message(
            "Patch Successfully Applied",
            "Patch successfully applied!\nNow you can enter the game.",
        )?;
    } else {
        ui::show_warning_message(
            "Patch Applied - DLL Missing",
            &format!(
                "Patch successfully applied!\n\
                But it looks like '{AWESOME_WOTLK_DLL}' is missing.\n\
                Before entering the game, you must place it in the game folder."
            ),
        )?;
    }

    if args.verbose {
        println!("Patching completed successfully!");
    }

    Ok(())
}
