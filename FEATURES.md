# HyprBrowser Features

Complete documentation of all HyprBrowser features and how to use them. Built with Tauri + WebView2 + HTML5.

## Table of Contents

1. [Core Browser](#core-browser)
2. [Tab Management](#tab-management)
3. [Address Bar](#address-bar)
4. [Quick Search](#quick-search)
5. [Sidebar & Panels](#sidebar--panels)
6. [Privacy & Security](#privacy--security)
7. [Downloads](#downloads)
8. [History](#history)
9. [Modules/Extensions](#modulesextensions)
10. [Developer Tools](#developer-tools)
11. [Themes & Appearance](#themes--appearance)
12. [Keyboard Shortcuts](#keyboard-shortcuts)
13. [Easter Eggs](#easter-eggs)

## Core Browser

### Window Management

**Borderless Design**

- Clean, modern interface inspired by Hyprland
- Transparent titlebar with rounded corners
- Smooth animations and transitions
- Minimalistic but fully functional

**Multi-Window Support**

- Open multiple HyprBrowser windows
- Independent tab sets per window
- Each window maintains its own state

### Navigation

**Back/Forward/Reload**

- Back button: Navigate to previous page
- Forward button: Navigate to next page
- Reload button: Refresh current page

**Home Button**

- Returns to home page (Google by default)
- Customize home page in settings

## Tab Management

### Creating Tabs

**New Tab**

- Shortcut: `Shift+T`
- Opens new tab with home page
- Auto-selects new tab

**New Incognito Tab**

- Shortcut: `Shift+Ctrl+T`
- Private browsing mode
- No history recording
- Separate from regular tabs

**Duplicate Tab**

- Shortcut: `Shift+D`
- Copies current tab URL
- Useful for comparing pages

### Tab Organization

**Pin/Unpin Tabs**

- Click 📌 icon on tab
- Pinned tabs appear in sidebar for quick access
- Pinned tabs stay at top of tab bar

**Close Tab**

- Click ✕ button on tab
- Shortcut: Close current with Ctrl+W (when implemented)

**Close Other Tabs**

- Shortcut: `Shift+O`
- Close all tabs except current
- Useful for cleanup

**Select Tab**

- Click on tab to select
- Navigate with keyboard: Ctrl+Tab, Ctrl+Shift+Tab

### Multi-Panel Layout

**Toggle Multi-Panel**

- Shortcut: `Shift+P`
- View multiple tabs side-by-side
- Auto-layout with grid
- Useful for multitasking

**Panel Features**

- Draggable panel dividers
- Custom size adjustment
- Full-screen tab on demand

## Address Bar

### Navigation

**Enter URL**

- Click address bar (or press `Shift+U`)
- Type URL
- Press Enter to navigate

**Smart URL Detection**

- `google.com` → `https://www.google.com`
- `localhost:3000` → `http://localhost:3000`
- `google search` → Google search

**History Suggestions**

- URL auto-complete from history
- Dropdown suggestions while typing

### Focus Address Bar

**Shortcut**: `Shift+U`

- Focuses the address bar
- Selects all text
- Ready for new input

### Go Home

**Shortcut**: `Shift+H`

- Navigates to home page
- Resets zoom level
- Clears search focus

## Quick Search

### Quick Search Bar

**Toggle**

- Shortcut: `Shift+Tab`
- Separate from address bar
- Slides up/down smoothly

**Features**

- Google instant search
- Built-in calculator
- Smart detection

### Instant Calculations

Type any math expression:

- `2+2` → `= 4`
- `10*5` → `= 50`
- `sqrt(16)` → `= 4`
- `sin(90)` → `= 1`

Supported operations:

- Basic: `+`, `-`, `*`, `/`, `%`
- Functions: `sqrt()`, `sin()`, `cos()`, `tan()`, `log()`, `exp()`
- Parentheses: `(1+2)*3`

### Google Search

Type search query:

- `best restaurants` → Google search results
- `python tutorial` → Google search results
- Automatic URL encoding

### Execute Search

**Methods**

1. Type query → Press Enter
2. Type query → Click "Search" button
3. Click suggestion from history

## Sidebar & Panels

### Sidebar Icons

**Home** 🏠

- Click to open new tab with home page

**Downloads** 📥

- View and manage downloads
- See download speed and progress
- Access downloaded files

**History** ⏱

- Browse visited pages
- Search history
- Clear all history

**Modules** 🧩

- Manage extensions
- Search GitHub for modules
- Install/enable/disable features

**Workflow** ⚙

- Theme selection
- Save/restore browser state
- Settings

**Keybindings** ⌨

- View all shortcuts
- Future: Edit custom keybindings

**Permissions** 🔒

- Manage site permissions
- Camera, microphone, location access
- Notification settings

**Updater** 🔄

- Check for new versions
- Auto-download updates
- One-click installation

### Pinned Tabs Section

**View Pinned Sites**

- Shows all pinned tabs
- Quick access sidebar
- Click to switch tab

**Pin/Unpin from Sidebar**

- Right-click tab → Pin
- Or use 📌 button on tab

## Privacy & Security

### Adblock Engine

**Toggle Adblock**

- Shortcut: `Shift+B`
- Blocks ads and trackers
- Reduces page load time
- Improves privacy

**Blocked Elements**

- Ad networks (Google Ads, etc.)
- Tracking pixels
- Analytics scripts
- Promotional content

**Statistics**

- Shows count of blocked ads
- Blocks per session
- Data saved (from ad bandwidth)

### Tracker Blocking

**Included Domains**

- google-analytics.com
- facebook.com
- doubleclick.net
- scorecardresearch.com
- hotjar.com
- mixpanel.com

**Automatic Blocking**

- No configuration needed
- Transparent to user
- Privacy reports available

### VPN Toggle

**Enable/Disable VPN**

- Quick toggle in gear menu
- Default: OFF
- Various server locations

**VPN Features**

- US, EU, Asia servers
- No DNS leaks
- Kill switch
- Auto-reconnect

**Note**: Placeholder for actual VPN integration

### Permissions Panel

**Manage Site Permissions**

- Camera access
- Microphone access
- Location access
- Notification permission

**Privacy Controls**

- Per-site settings
- Clear cache/cookies
- Clear browsing data
- Third-party cookie blocking

## Downloads

### Downloads Panel

**View Downloads**

- List all active/completed downloads
- Download progress bars
- Speed indicators
- Time remaining

**Download Actions**

- Pause: Pause active download
- Resume: Continue paused download
- Cancel: Stop and delete download
- Open: Open file when complete
- Show Folder: Open folder in file explorer

### Parallel Downloads

**Toggle Parallel**

- Downloads Panel → "Parallel Downloads" checkbox
- Multiple concurrent downloads
- Faster overall speed
- Configurable limits

**Download History**

- View completed downloads
- Clear old downloads
- Download statistics

### Download Notifications

- Notification on completion
- Sound alert (optional)
- Auto-open files (optional)

## History

### History Panel

**Browse History**

- List all visited pages
- With timestamps
- Grouped by date (Today, Yesterday, etc.)

**Search History**

- Search box at top
- Filter by URL or title
- Full-text search

**Clear History**

- Clear all history: "🗑 Clear" button
- Clear by date range
- Clear specific sites
- Clear cached data

**Privacy**

- Incognito tabs: Not saved to history
- Option to disable history entirely
- Auto-delete old entries

## Modules/Extensions

### Modules Panel

**Browse Modules**

- View installed modules
- View available modules
- Search by name/description

**Filter Modules**

- All modules
- Installed only
- Enabled/disabled status

**Install Module**

**From GitHub**:

1. Open Modules Panel
2. Search for `hyprbrowser_mod_<name>`
3. Click "Install"
4. Wait for download

**From File**:

1. Open Modules Panel
2. Click "📁 Upload Local Module"
3. Select `.rs` file
4. Click Upload

### Enable/Disable Modules

**Enable Module**

- Click checkbox or toggle button
- Module loads immediately
- Settings saved

**Disable Module**

- Uncheck to disable
- Module unloads without restart
- Settings preserved

### Uninstall Module

**Remove Module**

- Click "✕" delete button
- Confirms deletion
- Settings deleted

## Developer Tools

### Developer Tools Panel

**Access DevTools**

- Shortcut: F12 (when implemented)
- Or: Workflow Panel → DevTools

### Console

**View Logs**

- Application logs
- Error messages
- Debug output

**Execute Commands**

- Run JavaScript (future feature)
- Inspect variables
- Debug modules

### Network Inspector

**Monitor Requests**

- HTTP requests list
- Response status codes
- Response times
- Request headers
- Response preview

**Filtering**

- Filter by status
- Filter by domain
- Filter by type

### Element Inspector

**Inspect Page Structure**

- HTML tree view
- CSS styles
- Element properties
- Computed styles

**Edit Elements** (read-only for now)

- View element hierarchy
- See applied styles
- Inspect computed values

## Themes & Appearance

### Theme Selection

**Available Themes**

- Light: Clean, bright interface
- Dark: Easy on eyes, reduced blue light
- System: Follow OS theme

**Theme Toggle**

- Workflow Panel → Theme selector
- Immediately applies changes
- Saves preference

### Font

**Default Font**

- Claude Garamond (if available)
- Falls back to system font
- Customizable per theme

**Font Size**

- Interface font size adjustment (future)
- Zoom levels for pages
- Accessibility settings

### Appearance Options

**Window Decoration**

- Borderless mode (default)
- Rounded corners
- Transparency support

**Color Scheme**

- Automatic color detection
- High contrast mode
- Custom color theme (future)

## Keyboard Shortcuts

### Tab Operations

| Shortcut | Action |
|----------|--------|
| `Shift+T` | New Tab |
| `Shift+Ctrl+T` | New Incognito Tab |
| `Shift+D` | Duplicate Tab |
| `Shift+O` | Close Other Tabs |
| `Shift+P` | Toggle Multi-Panel |

### Navigation

| Shortcut | Action |
|----------|--------|
| `Shift+U` | Focus URL Bar |
| `Shift+H` | Go Home |
| `Shift+Tab` | Quick Search Toggle |

### Features

| Shortcut | Action |
|----------|--------|
| `Shift+B` | Toggle Adblock |
| `Shift+V` | Toggle VPN (custom) |

### Panels

| Shortcut | Action |
|----------|--------|
| Click 📥 | Toggle Downloads |
| Click ⏱ | Toggle History |
| Click 🧩 | Toggle Modules |
| Click ⚙ | Toggle Workflow |
| Click ⌨ | Toggle Keybindings |
| Click 🔒 | Toggle Permissions |
| Click 🔄 | Toggle Updater |

## Easter Eggs

### Snow Effect ❄️

**Trigger**

- Type `letitsnow` in address bar
- Duration: 5 seconds
- Realistic falling snow animation

**Shader Effect**

- GPU-accelerated with wgpu
- Dynamic particle physics
- Beautiful ambient effect

**Features**

- Wind simulation
- Gravity effects
- Fade out animation
- Auto-disables after 5 seconds

### Hidden Features

Keep exploring! There might be more surprises... 👀

---

## Accessibility

### Keyboard Navigation

- Full keyboard control
- Tab navigation
- Arrow keys for panels
- Enter to confirm

### High Contrast Mode

- Available in theme settings
- Improved readability
- Better for low vision users

### Text Scaling

- Zoom in/out on pages
- Font size adjustment
- Line spacing control

## Performance Features

### Memory Management

- Smart cache management
- Automatic cleanup
- Memory profiling tools

### Network Optimization

- Parallel downloads
- Request batching
- Connection pooling
- Compression support

### Rendering

- GPU acceleration (wgpu)
- Lazy rendering
- Frame rate limiting
- Smooth animations

---

**Master all features and enjoy ultra-fast browsing! 🚀**
