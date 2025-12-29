# WebView Integration Implementation Guide

## Current Status

The HyprBrowser frontend is currently using an **iframe placeholder** for webpage rendering. To enable real webpage browsing, we need to replace this with actual Tauri WebView integration.

## What's Already Done

✅ **Rust Backend**: Fully configured to handle navigation commands
- `navigate(url)` command defined in `src/commands.rs`
- `back()`, `reload()`, `forward()` navigation methods ready
- State management for tab URLs implemented

✅ **Frontend Structure**: HTML and CSS ready
- Tab system with URL input working
- Address bar navigation hooked up
- Sidebar and panel system operational

✅ **Architecture**: Tauri configured for WebView
- Tauri 1.5 with WebView2 (Windows) support
- Security allowlist configured (`all: true`)
- CSP set to null (allows all content)

## What Needs Implementation

### Step 1: Implement `navigate()` Backend Command

**File**: `src/commands.rs`

Replace the stub with:
```rust
#[tauri::command]
pub fn navigate(url: String) -> Result<(), String> {
    // Update current tab URL in state
    // Trigger history recording
    // Validate and normalize URL
    Ok(())
}
```

### Step 2: Wire Frontend WebView Rendering

**File**: `dist/app.js`

In the `renderWebView()` method, replace iframe with Tauri WebView:

```javascript
renderWebView() {
    const container = document.getElementById('webview-container');
    const tab = this.tabs[this.currentTab];
    
    // Update address bar
    document.getElementById('url-input').value = tab.url;
    
    // Tauri automatically renders WebView in the main container
    // The webpage is displayed natively by the OS
    // Just ensure the container ID matches Tauri's configuration
}
```

### Step 3: URL Handling and Normalization

Add URL validation in frontend `navigate()`:

```javascript
navigate(url) {
    let normalizedUrl = url.trim();
    
    // Add protocol if missing
    if (!normalizedUrl.startsWith('http://') && 
        !normalizedUrl.startsWith('https://') &&
        !normalizedUrl.startsWith('about:') &&
        !normalizedUrl.startsWith('data:')) {
        // Treat as search query
        if (normalizedUrl.includes('.') && !normalizedUrl.includes(' ')) {
            normalizedUrl = 'https://' + normalizedUrl;
        } else {
            normalizedUrl = 'https://www.google.com/search?q=' + encodeURIComponent(normalizedUrl);
        }
    }
    
    this.tabs[this.currentTab].url = normalizedUrl;
    this.tabs[this.currentTab].title = normalizedUrl;
    
    // Call backend
    invoke('navigate', { url: normalizedUrl });
    
    // Update UI
    this.renderWebView();
    this.saveState();
}
```

### Step 4: History Management

Update `navigate()` to manage browser history:

```rust
#[tauri::command]
pub fn navigate(url: String) -> Result<(), String> {
    // Remove any forward history when navigating to new URL
    // this.history = this.history.slice(0, this.history_pos + 1);
    
    // Add new URL to history
    // this.history.push(url.clone());
    // this.history_pos = this.history.len() - 1;
    
    Ok(())
}
```

### Step 5: Back/Forward Navigation

Implement in `src/commands.rs`:

```rust
#[tauri::command]
pub fn back() -> Result<String, String> {
    // Decrease history_pos
    // Return URL at that position
    Ok("https://www.google.com".to_string())
}

#[tauri::command]
pub fn forward() -> Result<String, String> {
    // Increase history_pos
    // Return URL at that position
    Ok("https://www.google.com".to_string())
}

#[tauri::command]
pub fn reload() -> Result<(), String> {
    // Reload current tab URL
    Ok(())
}
```

### Step 6: Communication Between Tabs and WebView

Each tab should maintain its own WebView context:

```javascript
newTab() {
    const newTab = {
        url: 'https://www.google.com',
        title: 'New Tab',
        pinned: false,
        incognito: false,
        history: ['https://www.google.com'],
        history_pos: 0
    };
    
    this.tabs.push(newTab);
    this.currentTab = this.tabs.length - 1;
    
    this.renderTabs();
    this.renderWebView();  // This switches the WebView to show new tab's content
    this.saveState();
}

selectTab(index) {
    if (index >= 0 && index < this.tabs.length) {
        this.currentTab = index;
        this.renderTabs();
        this.renderWebView();  // Switch WebView to this tab's URL
    }
}
```

## Advanced Features

### JavaScript Injection (Optional)

To inject scripts into webpages:

```rust
#[tauri::command]
pub fn inject_script(js_code: String) -> Result<(), String> {
    // Use Tauri's WebView API to inject JavaScript
    // This enables adblocker, VPN scripts, etc.
    Ok(())
}
```

### Extension Hook Points

- **Before page load**: Run ad-blocking scripts
- **After page load**: Apply VPN/privacy scripts
- **On navigation**: Check permissions, block malicious sites
- **On DOM change**: Monitor for ads, pop-ups

### Incognito Mode

```rust
#[tauri::command]
pub fn toggle_incognito() -> Result<(), String> {
    // Set current tab incognito flag
    // Clear history on close
    // Disable cookie storage
    Ok(())
}
```

## Testing WebView Integration

### Test 1: Basic Navigation
1. Run `cargo run`
2. Type URL in address bar: `https://www.google.com`
3. Press Enter
4. Verify webpage loads

### Test 2: Tab Switching
1. Open multiple tabs (Shift+T)
2. Navigate to different URLs in each
3. Click between tabs
4. Verify each tab shows its URL

### Test 3: History Navigation
1. Navigate to 3 different pages
2. Press back button
3. Verify URL decrements through history
4. Press forward
5. Verify URL increments

### Test 4: Incognito Mode
1. Create incognito tab (Shift+Ctrl+T)
2. Navigate to websites
3. Close tab
4. Reopen incognito tab
5. Verify history is empty

## Performance Optimization

### WebView Pooling
- Pre-render WebView for next tab during idle
- Improve perceived performance

### Lazy Loading
- Don't load WebView until tab is visible
- Save memory for inactive tabs

### Cache Management
- Manage Tauri WebView cache directory
- Clear old cache periodically

## Security Considerations

### Content Security Policy
Already configured: `csp: null` allows all content (for now)

For production, tighten to:
```json
"csp": "default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'"
```

### JavaScript Isolation
- Modules cannot access browser internals
- Extensions run in sandboxed context
- Tauri handles permission boundaries

### Site Permissions
Track in `permissions.json`:
```json
{
  "https://google.com": { "camera": false, "microphone": false },
  "https://github.com": { "camera": false, "microphone": false }
}
```

## Debugging WebView Issues

### Windows-Specific
- Ensure WebView2 is installed
- Check Event Viewer for WebView errors
- Use Windows Sandbox for testing

### Logging WebView Events
Add to `src/commands.rs`:
```rust
#[tauri::command]
pub fn log_webview_event(event: String) -> Result<(), String> {
    eprintln!("WebView Event: {}", event);
    Ok(())
}
```

### Common Issues

**Issue**: WebView not rendering
- **Solution**: Check `frontendDist` path in tauri.conf.json

**Issue**: JavaScript doesn't execute
- **Solution**: Ensure `csp` allows scripts

**Issue**: Slow page load
- **Solution**: Check network, enable cache

## Reference Implementation

The complete implementation will be available in:
- `src/commands.rs`: Backend navigation logic
- `dist/app.js`: Frontend rendering and tab management
- `src/state.rs`: Tab state and history tracking

## Timeline for Integration

**Phase 1 (Immediate)**: Basic navigation working
- Implement `navigate()` backend command
- Wire frontend `renderWebView()` to Tauri WebView
- Test basic URL loading

**Phase 2 (Short-term)**: Full tab system
- Implement back/forward/reload
- Tab switching with proper history
- Session restoration

**Phase 3 (Medium-term)**: Advanced features
- Module system integration
- Extension hooks
- Script injection

**Phase 4 (Long-term)**: Optimization
- WebView pooling
- Performance tuning
- Cache management

---

**Status**: ✅ Architecture ready, awaiting implementation  
**Priority**: HIGH (core browser functionality)  
**Estimated Time**: 2-4 hours for full implementation
