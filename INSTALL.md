# HyprBrowser Installation Guide

Complete step-by-step instructions for building and installing HyprBrowser on Windows, Linux, and macOS using Tauri.

## Table of Contents

- [System Requirements](#system-requirements)
- [Windows Installation](#windows-installation)
- [Linux Installation](#linux-installation)
- [macOS Installation](#macos-installation)
- [Building from Source](#building-from-source)
- [Troubleshooting](#troubleshooting)

## System Requirements

### Minimum Requirements
- **Rust**: 1.70 or newer ([Install](https://rustup.rs/))
- **RAM**: 2GB
- **Disk Space**: 1GB (for build artifacts)
- **Internet**: For downloading dependencies

### Platform-Specific Requirements

**Windows 10+**
- WebView2 Runtime ([Install](https://developer.microsoft.com/en-us/microsoft-edge/webview2/))

**Linux (Ubuntu/Debian/Fedora)**
- GTK 3.0+ development libraries
- WebKit runtime

**macOS 10.15+**
- Xcode Command Line Tools
- WKWebView (included in macOS)

## Windows Installation

### Step 1: Install Rust

```powershell
# Download and run installer from https://rustup.rs/
# Or use:
Invoke-WebRequest -Uri https://win.rustup.rs -OutFile rustup.exe
.\rustup.exe
```

Verify installation:
```powershell
rustc --version
cargo --version
```

### Step 2: Install WebView2 (Required)

Download from: https://developer.microsoft.com/en-us/microsoft-edge/webview2/

Or install via package manager:
```powershell
winget install Microsoft.WebView2Runtime
```

### Step 3: Clone Repository

```powershell
git clone https://github.com/pro-grammer-SD/hyprbrowser
cd hyprbrowser
```

### Step 4: Build Using build.ps1

```powershell
# Release build (optimized)
.\build.ps1 release

# Or debug build
.\build.ps1 debug

# Or build and run
.\build.ps1 run
```

**Output**: `target/release/hyprbrowser.exe` (3MB optimized binary)
# 5. Install to system (optional)
cargo install --path .
# Then run: hyprbrowser
```

### Fedora/RHEL

```bash
# 1. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 2. Install dependencies
sudo dnf groupinstall "Development Tools"
sudo dnf install -y \
    openssl-devel \
    pkg-config \
    alsa-lib-devel \
    vulkan-devel

# 3. Clone and build
git clone https://github.com/pro-grammer-SD/hyprbrowser
cd hyprbrowser
cargo run --release
```

### Arch Linux

```bash
# 1. Install Rust and dependencies
sudo pacman -S rust cargo base-devel

# 2. Install build dependencies
sudo pacman -S openssl pkg-config alsa-lib vulkan-intel

# 3. Clone and build
git clone https://github.com/pro-grammer-SD/hyprbrowser
cd hyprbrowser
cargo run --release
```

## Windows Installation

### Using Windows Subsystem for Linux (WSL2) - Recommended

```powershell
# 1. Enable WSL2 and install Ubuntu
wsl --install -d Ubuntu

# 2. In WSL terminal:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 3. Install dependencies
sudo apt update && sudo apt install -y build-essential pkg-config libssl-dev

# 4. Clone and build
git clone https://github.com/pro-grammer-SD/hyprbrowser
cd hyprbrowser
cargo run --release
```

### Native Windows (MSVC)

```powershell
# 1. Install Rust (choose MSVC toolchain)
# Download from: https://www.rust-lang.org/tools/install

# 2. Install Visual Studio Build Tools (if not present)
# Download from: https://visualstudio.microsoft.com/downloads/
# Select: Desktop development with C++

# 3. Install Git
# Download from: https://git-scm.com/download/win

# 4. Clone repository
git clone https://github.com/pro-grammer-SD/hyprbrowser
cd hyprbrowser

# 5. Build and run
cargo run --release

# 6. Executable location: .\dist\hyprbrowser.exe
```

### Windows: Troubleshooting GPU

If you get GPU errors, try:

```powershell
# Use Vulkan backend (if installed)
$env:WGPU_BACKEND="vulkan"
cargo run --release

# Or use DirectX 12 (default)
$env:WGPU_BACKEND="dx12"
cargo run --release

# Or software rendering (slow)
$env:WGPU_BACKEND="cpu"
cargo run --release
```

## macOS Installation

### Intel Mac

```bash
# 1. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# 2. Install Xcode Command Line Tools
xcode-select --install

# 3. Install OpenSSL (if needed)
brew install openssl pkg-config

# 4. Clone and build
git clone https://github.com/pro-grammer-SD/hyprbrowser
cd hyprbrowser
cargo run --release

# 5. Create app bundle (optional)
./scripts/create-macos-app.sh
# Creates: HyprBrowser.app
```

### Apple Silicon Mac

```bash
# Same as Intel, but Rust will auto-detect ARM64
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install dependencies
brew install openssl pkg-config

# Build (will compile for ARM64 automatically)
git clone https://github.com/pro-grammer-SD/hyprbrowser
cd hyprbrowser
cargo run --release
```

## Building from Source

### Quick Build

```bash
# Clone
git clone https://github.com/pro-grammer-SD/hyprbrowser
cd hyprbrowser

# Build (debug mode - faster to compile, slower to run)
cargo build

# Run
./target/debug/hyprbrowser

# Or run directly
cargo run
```

### Optimized Build

```bash
# Build (release mode - optimized for speed)
cargo build --release

# Run
./target/release/hyprbrowser

# Or run directly
cargo run --release

# Copy to dist/
cargo build --release
cp target/release/hyprbrowser dist/
```

### Custom Build Options

```bash
# Build with all features
cargo build --release --all-features

# Build with specific features
cargo build --release --features "wgpu,debug"

# Build with logging
RUST_LOG=debug cargo run --release

# Build with optimizations
RUSTFLAGS="-C opt-level=3 -C lto=fat -C codegen-units=1" cargo build --release
```

## Post-Installation

### Setting Up HyprBrowser

1. **First Run**
   - HyprBrowser will create necessary directories
   - Default data folder: `~/.local/share/hyprbrowser/` (Linux) or `%APPDATA%/hyprbrowser/` (Windows)

2. **Create Desktop Shortcut** (optional)

   **Linux**:
   ```bash
   mkdir -p ~/.local/share/applications
   cat > ~/.local/share/applications/hyprbrowser.desktop << EOF
   [Desktop Entry]
   Name=HyprBrowser
   Exec=$HOME/.cargo/bin/hyprbrowser
   Type=Application
   Categories=Internet;
   EOF
   ```

   **Windows**:
   - Right-click dist/hyprbrowser.exe → Send to → Desktop (create shortcut)

3. **Add to PATH** (optional)

   **Linux/macOS**:
   ```bash
   cargo install --path .
   # Then: hyprbrowser
   ```

   **Windows**:
   - Add `C:\Users\YourUsername\AppData\Local\Cargo\bin` to PATH in Environment Variables

### First Launch Checklist

- [ ] Browser window appears
- [ ] Sidebar icons visible
- [ ] Can navigate to Google
- [ ] Can create new tab (Shift+T)
- [ ] Can open quick search (Shift+Tab)
- [ ] Preferences panel opens (Workflow button)

## Troubleshooting

### Compilation Errors

**Error**: `error: ld returned 1 exit status`

**Solution** (Linux):
```bash
sudo apt install libssl-dev pkg-config
cargo clean
cargo build --release
```

**Error**: `cargo: command not found`

**Solution**:
```bash
source $HOME/.cargo/env
# Or add to ~/.bashrc or ~/.zshrc:
export PATH="$HOME/.cargo/bin:$PATH"
```

### GPU/Rendering Errors

**Error**: `wgpu: No suitable GPU found`

**Solution**:
```bash
# Update GPU drivers first, then try:
WGPU_BACKEND=vulkan cargo run --release  # Linux
WGPU_BACKEND=dx12 cargo run --release     # Windows
WGPU_BACKEND=metal cargo run --release    # macOS
```

**Error**: `EGL not found` (Linux)

**Solution**:
```bash
sudo apt install libegl1-mesa-dev libgles2-mesa-dev libxkbcommon-dev
```

### Memory Issues

**Error**: `Cannot allocate memory` during build

**Solution**:
```bash
# Build with fewer threads
cargo build --release -j 2

# Or pre-allocate swap (Linux)
sudo fallocate -l 4G /swapfile
sudo chmod 600 /swapfile
sudo mkswap /swapfile
sudo swapon /swapfile
```

### Runtime Errors

**Error**: `State not found` warning

**Solution**: Normal on first run. HyprBrowser creates state automatically.

**Error**: Data directory permission denied

**Solution**:
```bash
# Linux
chmod -R 755 ~/.local/share/hyprbrowser/

# Windows - Run as Administrator
```

### Performance Issues

1. **Slow startup**: Clean build cache
   ```bash
   cargo clean
   cargo build --release
   ```

2. **Low FPS**: Check GPU drivers and try different backend

3. **High memory**: Close unused modules and tabs

### Getting Help

If you encounter issues:

1. Check existing [GitHub Issues](https://github.com/pro-grammer-SD/hyprbrowser/issues)
2. Create a new issue with:
   - OS and version
   - Rust version (`rustc --version`)
   - Full error message
   - Steps to reproduce

3. Join [GitHub Discussions](https://github.com/pro-grammer-SD/hyprbrowser/discussions)

## Next Steps

1. **Customize**: Edit theme in Workflow Panel
2. **Install Modules**: Go to Modules Panel and search for extensions
3. **Set Up**: Adjust keybindings and permissions
4. **Optimize**: Enable adblock (Shift+B) for faster browsing

---

**Installation Complete! 🎉**

Start browsing with HyprBrowser:
```bash
# From project directory:
cargo run --release

# Or if installed globally:
hyprbrowser
```
