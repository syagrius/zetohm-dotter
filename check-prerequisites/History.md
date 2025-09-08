# History - Check Prerequisites

## Version 1.0.2 (2025-09-07)

### ✨ New Features

**Enhanced Package Detection**
- Added `dotter` to Scoop packages list for dotfile management tool verification
- Added `rust` to manual packages list with `rustc` command verification
- Enhanced manual package detection shows version information for both conda and rustc

### 📋 Changes Made

1. **src/main.rs:8-10**: Added "dotter" to `SCOOP_PACKAGES` constant array
2. **src/main.rs:14-17**: Added ("rust", "rustc") to `MANUAL_PACKAGES` constant array  
3. **Cargo.toml:3**: Updated version to 1.0.2

### ✅ Package Detection Summary

- **Manual packages**: 2/2 (miniconda3 via conda, rust via rustc)
- **Scoop packages**: 10/10 (including new dotter package)
- **Chocolatey packages**: 2/2 (wezterm, miniconda3 with cross-reference detection)

## Version 1.0.1 (2025-09-07)

### ✨ New Features

**Added Version Information Support**
- Added `--version` and `-V` command line flags to display version information
- Version information is automatically extracted from `Cargo.toml` using `env!("CARGO_PKG_VERSION")`
- Updated help text to include version flag documentation

### 🔧 Technical Improvements

**Windows Binary Metadata**
- Added `build.rs` with Windows resource compilation support
- Integrated Windows binary metadata including:
  - Product name: "Zet'ohm Prerequisites Checker"
  - File description: "Zet'ohm Development Environment Prerequisites Checker"
  - Company: "Zet'ohm"
  - Version info: "1.0.1.0"
  - Copyright: "© 2025 Zet'ohm"
- Added `winres` build dependency for Windows resource management

**Build Optimizations**
- Added release profile optimizations:
  - Strip debug symbols (`strip = true`)
  - Link Time Optimization (`lto = true`)
  - Single codegen unit for maximum optimization (`codegen-units = 1`)
  - Abort on panic for smaller binary size (`panic = "abort"`)

### 📋 Changes Made

1. **src/main.rs:42-46**: Updated `Args` struct to include `version` field
2. **src/main.rs:48-66**: Enhanced `parse_args()` function with version flag handling
3. **src/main.rs:77**: Added version flag to help text
4. **Cargo.toml:3**: Updated version to 1.0.1
5. **Cargo.toml:15-22**: Added build dependencies and release profile optimizations
6. **build.rs**: New file for Windows binary metadata compilation

## Version 1.0.0 (2025-09-07)

### 🐛 Bug Fixes

**Fixed Scoop Export JSON Parsing Issue**
- **Problem**: `scoop export` command was failing when called from Rust code due to JSON format incompatibility
- **Root Cause**: The code expected a simple `{"apps": [...]}` format, but `scoop export` returns a more complex structure with `{"buckets": [...], "apps": [...]}`
- **Solution**: 
  - Updated `ScoopExport` struct to include `buckets` field with new `ScoopBucket` struct
  - Changed `ScoopApp.version` field from `String` to `Option<String>` to handle null versions
  - Enhanced error handling by capturing stderr instead of discarding it
  - Added proper filtering for apps without versions during processing

### 🔧 Technical Improvements

- **Better Error Diagnostics**: Added detailed error messages showing JSON parsing failures and stderr output
- **Robust Version Handling**: Apps with failed installations or missing versions are now properly skipped
- **Enhanced Debug Information**: Added informative error messages for troubleshooting parsing issues

### 📋 Changes Made

1. **src/main.rs:18-24**: Updated `ScoopApp` struct to make `version` field optional
2. **src/main.rs:26-32**: Added new `ScoopBucket` struct for bucket information
3. **src/main.rs:34-38**: Extended `ScoopExport` struct to include `buckets` field
4. **src/main.rs:214-248**: Completely rewrote `get_scoop_installed_packages()` function with:
   - Proper stderr capture and error reporting
   - Enhanced JSON parsing with detailed error messages  
   - Safe handling of apps with null/empty versions
   - Better debugging output for troubleshooting

### ✅ Verification

- Scoop package detection now works correctly
- Application successfully detects installed packages with their versions
- Manual testing shows proper functionality across all package managers
- Build completes successfully with only minor dead code warnings (expected)