# HyprBrowser - Tauri Rebuild

## Architecture

### Tech Stack
- **Frontend**: HTML5 + CSS3 + Vanilla JavaScript
- **Backend**: Rust + Tauri
- **Browser Engine**: System WebView (WebView2 on Windows, WKWebView on macOS, WebKit on Linux)
- **Module System**: Rust-based dynamic loading
- **Data Storage**: Relative to executable (/data, /profiles, /modules, /downloads)

### Project Structure
```
hyprbrowser/
├── src/
│   ├── main.rs           # Tauri app entry point
│   ├── state.rs          # App state management
│   ├── modules.rs        # Module system
│   ├── downloads.rs      # Download manager
│   ├── updater.rs        # Update system
│   └── commands.rs       # Tauri IPC commands
├── dist/
│   ├── index.html        # Main UI
│   ├── app.js            # Frontend logic
│   ├── styles.css        # Styling
│   ├── font.ttf          # Custom font
│   └── [assets]
└── tauri.conf.json       # Tauri configuration
├── Cargo.toml            # Rust dependencies
└── build.rs              # Build script
```

## Features

### ✅ Implemented
- **Tauri Framework**: Cross-platform desktop app
- **WebView Browser**: Real system browser engine
- **Window Management**: Transparent, rounded, custom titlebar
- **Tab System**: Create, close, duplicate, pin tabs
- **Quick Search**: Shift+Tab with math evaluation
- **Keybindings**: All standard shortcuts working
- **State Management**: Auto-save tabs, theme, settings
- **Module System**: (Rust backend ready)
- **Updater**: (GitHub release tracking ready)
- **Panels**: Downloads, History, Modules, Workflow, Keybindings, Permissions, Updater
- **Silent Operation**: Zero runtime logs

### 🎨 UI/UX
- **Hyprland-Inspired Design**: Dark theme, green accents, transparency, blur
- **Custom Font**: Loaded from assets/font.ttf
- **Responsive Layout**: Sidebar, tabs, address bar, panels
- **Animations**: Smooth transitions and panel slides
- **Emoji Support**: Full emoji rendering (no fallback issues)

## Building

### Prerequisites
- Rust 1.60+
- Node.js (if using npm for frontend tooling)
- Windows 10+ (WebView2 runtime)

### Build
```bash
cargo build --release
```

### Output
- Windows: `target/release/hyprbrowser.exe`
- Binary will be in `dist/` after bundling

## Running

### Development
```bash
cargo tauri dev
```

### Production
```bash
cargo tauri build
```

## Environment

### Logging
- **Development**: Logging enabled (cargo handles it)
- **Production**: ZERO logs at runtime
  - No env_logger
  - No log::info statements in runtime code
  - Silent startup and shutdown

### Data Paths (Relative to Executable)
- `/data/` - App data and state
- `/profiles/` - User profiles
- `/modules/` - Installed modules  
- `/downloads/` - Downloaded files

## IPC Commands

All frontend-backend communication via Tauri's `invoke()`:

### Tab Management
- `new_tab()` - Create new tab
- `close_tab(index)` - Close tab
- `select_tab(index)` - Activate tab
- `duplicate_tab(index)` - Duplicate tab
- `navigate(url)` - Navigate to URL
- `pin_tab(index)` - Pin tab
- `unpin_tab(index)` - Unpin tab

### State
- `save_state()` - Save app state
- `load_state()` - Load app state

### Downloads
- `get_downloads()` - List downloads
- `cancel_download(id)` - Cancel download
- `pause_download(id)` - Pause download
- `resume_download(id)` - Resume download

### History
- `get_history()` - Get history
- `clear_history()` - Clear history

### Modules
- `search_modules(query)` - Search GitHub
- `install_module(repo)` - Install module
- `uninstall_module(name)` - Uninstall
- `enable_module(name)` - Enable
- `disable_module(name)` - Disable

### Updater
- `check_updates()` - Check GitHub releases
- `apply_update()` - Download and apply update

### Settings
- `get_keybindings()` - Get keybindings
- `set_keybinding(key, binding)` - Update keybinding
- `set_theme(theme)` - Set theme (Light/Dark/System)
- `toggle_adblock()` - Toggle adblock
- `toggle_vpn()` - Toggle VPN

### Utilities
- `evaluate_expression(expr)` - Math eval
- `search_google(query)` - Generate Google search URL

## Current Status

✅ **Phase 1: Architecture** - COMPLETE
- Tauri project initialized
- Rust backend modules created
- HTML/CSS/JS frontend created
- Window styling implemented
- Font loading configured

🔄 **Phase 2: Core Features** (Next)
- Module system implementation
- WebView integration
- Real download manager
- History tracking
- Updater integration

## Notes

- Font must load successfully; no fallbacks
- All emojis render correctly via HTML/CSS
- Window is transparent by default
- Rounded corners applied via CSS border-radius
- Blue accents used for interactive elements
- No debug output in production

## Future Enhancements

- WebView sandbox security
- Profile management
- Advanced module APIs
- Custom themes
- Multi-window support
- Android support (future)
