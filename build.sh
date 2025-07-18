#!/bin/bash

# Build script for Awesome WotLK Rust port
# This script builds the patcher for both Linux and Windows, and the DLL for Windows only

set -e

echo "Building Awesome WotLK Rust port..."

# Build for Linux (patcher only, no DLL needed)
echo "Building patcher for Linux..."
cargo build --release --bin patcher

# Build for Windows (both patcher and DLL)
echo "Cross-compiling for Windows..."
cargo build --release --target x86_64-pc-windows-gnu

# Create release directory
mkdir -p release

# Copy Linux binaries (patcher only)
echo "Copying Linux binaries..."
cp target/release/patcher release/patcher-linux

# Copy Windows binaries (patcher + DLL)
echo "Copying Windows binaries..."
cp target/x86_64-pc-windows-gnu/release/patcher.exe release/
cp target/x86_64-pc-windows-gnu/release/awesome_wotlk_lib.dll release/

# Show file sizes
echo "Build complete! File sizes:"
ls -la release/

echo ""
echo "Release files created in 'release/' directory:"
echo "  - patcher.exe (Windows patcher)"
echo "  - awesome_wotlk_lib.dll (Windows library - works on Wine too)"
echo "  - patcher-linux (Linux patcher)"
echo ""
echo "For Windows WoW (native or Wine): use awesome_wotlk_lib.dll"
echo "The .dll works on both Windows and Linux (via Wine)"
