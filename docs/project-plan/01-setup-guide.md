# JustCall Setup Guide

## Prerequisites Installation

### 1. Rust & Cargo

**Download**: https://www.rust-lang.org/tools/install
- Click "Download Rustup" for your OS
- Run the installer (it will open Terminal/Command Prompt)
- Follow the prompts

**Verify Installation**:
```bash
rustc --version
# Should show: rustc 1.xx.x (...)

cargo --version  
# Should show: cargo 1.xx.x (...)
```

**Troubleshooting**:
- If commands not found: Restart terminal or run `source $HOME/.cargo/env`
- macOS: May need Xcode Command Line Tools - it will prompt to install
- Windows: Make sure to select "Add to PATH" during install

### 2. Node.js & npm

**Download**: https://nodejs.org/
- Choose LTS version (left button)
- Run the installer with default settings

**Verify Installation**:
```bash
node --version
# Should show: v20.x.x or higher

npm --version
# Should show: 10.x.x or higher
```

**Troubleshooting**:
- If old version exists: Uninstall first or use nvm (Node Version Manager)
- Permission errors: Never use `sudo` with npm, fix with: `npm config set prefix ~/.npm`

### 3. Tauri Prerequisites

#### macOS
- **Xcode Command Line Tools**: Will auto-prompt when you run `rustc` for first time
- **WebKit**: Pre-installed on macOS

#### Windows  
**Download WebView2**: https://developer.microsoft.com/en-us/microsoft-edge/webview2/
- Download "Evergreen Bootstrapper"
- Run installer

**Download Visual Studio Build Tools**: https://visualstudio.microsoft.com/visual-cpp-build-tools/
- During install, select:
  - "Desktop development with C++"
  - Windows 10/11 SDK

#### Linux
```bash
# For Ubuntu/Debian (run in terminal):
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev build-essential curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev

# For Fedora:
sudo dnf install webkit2gtk4.0-devel openssl-devel gtk3-devel libappindicator-gtk3-devel
```

### 4. Tauri CLI

After Rust is installed, run:
```bash
cargo install tauri-cli
```

**Verify Installation**:
```bash
cargo tauri --version
# Should show: tauri-cli 1.x.x
```

### 5. Development Tools

**VS Code** (Recommended): https://code.visualstudio.com/
- Install extensions:
  - rust-analyzer
  - Tauri
  - Even Better TOML

**Git**: https://git-scm.com/downloads
- Download and install with defaults

### 6. Platform-Specific Tools

#### macOS
- Enable "Developer Mode" in System Preferences → Privacy & Security
- For code signing later: Apple Developer account (free tier is fine)

#### Windows
- Run VS Code as Administrator when developing (for global hotkeys)
- Windows Defender may flag the app - add exclusion for project folder

#### Linux
- For Wayland: Additional packages may be needed for global hotkeys
- Test on both X11 and Wayland sessions

## Final Verification

Create a test file `verify-setup.sh`:
```bash
#!/bin/bash
echo "Checking installations..."
echo "Rust: $(rustc --version)"
echo "Cargo: $(cargo --version)"
echo "Node: $(node --version)"
echo "npm: $(npm --version)"
echo "Tauri CLI: $(cargo tauri --version)"
echo ""
echo "All tools installed successfully!"
```

Run with: `bash verify-setup.sh`

## Common Issues

1. **"command not found"**: Restart terminal or check PATH
2. **Permission denied**: Never use sudo with cargo/npm
3. **Slow downloads**: Use a VPN or mirror sites
4. **Build errors**: Usually missing system dependencies

## Next Steps

Once everything is installed:
1. Clone the repo (already done)
2. Navigate to project: `cd /Users/kumar/Documents/Projects/justcall/justcall`
3. Start with Phase 1 of the project plan
