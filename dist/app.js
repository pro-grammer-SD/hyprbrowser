// HyprBrowser Frontend - Complete Implementation
const { invoke } = window.__TAURI__.tauri;
const { appWindow } = window.__TAURI__.window;

class HyprBrowser {
    constructor() {
        this.tabs = [];
        this.currentTab = 0;
        this.currentPanel = null;
        this.adblockEnabled = true;
        this.vpnEnabled = false;
        this.theme = 'system';
        
        this.init();
    }

    async init() {
        // Load saved state
        try {
            const state = await invoke('load_state');
            this.tabs = state.tabs;
            this.currentTab = state.current_tab;
            this.theme = state.theme;
            this.adblockEnabled = state.adblock_enabled;
            this.vpnEnabled = state.vpn_enabled;
        } catch (e) {
            console.error('Failed to load state');
            this.tabs = [{
                url: 'https://www.google.com',
                title: 'Google',
                pinned: false,
                incognito: false,
                history: ['https://www.google.com'],
                history_pos: 0
            }];
        }

        this.setupEventListeners();
        this.renderTabs();
        this.renderWebView();
    }

    setupEventListeners() {
        // Titlebar drag region
        const titlebar = document.querySelector('.titlebar');
        if (titlebar) {
            titlebar.setAttribute('data-tauri-drag-region', '');
        }

        // Titlebar controls
        const closeBtn = document.getElementById('close-btn');
        const minimizeBtn = document.getElementById('minimize-btn');
        const maximizeBtn = document.getElementById('maximize-btn');

        if (closeBtn) closeBtn.addEventListener('click', () => appWindow.close());
        if (minimizeBtn) minimizeBtn.addEventListener('click', () => appWindow.minimize());
        if (maximizeBtn) maximizeBtn.addEventListener('click', () => appWindow.toggleMaximize());

        // Sidebar panel buttons
        document.querySelectorAll('.sidebar-btn').forEach(btn => {
            btn.addEventListener('click', (e) => {
                const panel = btn.dataset.panel;
                this.switchPanel(panel);
            });
        });

        // Navigation buttons
        const navBtns = document.querySelectorAll('.nav-btn');
        if (navBtns[0]) navBtns[0].addEventListener('click', () => this.back());
        if (navBtns[1]) navBtns[1].addEventListener('click', () => this.forward());
        if (navBtns[2]) navBtns[2].addEventListener('click', () => this.reload());

        // URL input - MUST be editable and navigate
        const urlInput = document.getElementById('url-input');
        if (urlInput) {
            urlInput.addEventListener('keydown', (e) => {
                if (e.key === 'Enter') {
                    e.preventDefault();
                    this.navigate(urlInput.value);
                }
            });
            urlInput.addEventListener('focus', () => urlInput.select());
        }

        // Tab bar - new tab button
        const newTabBtn = document.querySelector('.new-tab-btn');
        if (newTabBtn) {
            newTabBtn.addEventListener('click', () => this.newTab());
        }

        this.setupKeybindings();
    }

    setupKeybindings() {
        document.addEventListener('keydown', (e) => {
            const shift = e.shiftKey;
            const ctrl = e.ctrlKey;
            const key = e.key.toUpperCase();

            // Shift+T: New tab
            if (shift && key === 'T' && !ctrl) {
                e.preventDefault();
                this.newTab();
            }
            // Shift+Ctrl+T: New incognito tab
            else if (shift && ctrl && key === 'T') {
                e.preventDefault();
                this.newIncognitoTab();
            }
            // Shift+D: Duplicate tab
            else if (shift && key === 'D') {
                e.preventDefault();
                this.duplicateTab();
            }
            // Shift+Ctrl+W: Close tab
            else if (shift && ctrl && key === 'W') {
                e.preventDefault();
                this.closeTab(this.currentTab);
            }
            // Shift+U: Focus URL bar
            else if (shift && key === 'U') {
                e.preventDefault();
                const urlInput = document.getElementById('url-input');
                if (urlInput) urlInput.focus();
            }
            // Shift+H: Home
            else if (shift && key === 'H') {
                e.preventDefault();
                this.navigate('https://www.google.com');
            }
            // Shift+Tab: Quick search (without shift, it's normal tab navigation)
            else if (shift && key === 'Tab') {
                e.preventDefault();
                this.openQuickSearch();
            }
        });
    }

    switchPanel(panelName) {
        this.currentPanel = panelName;

        // Update sidebar active state
        document.querySelectorAll('.sidebar-btn').forEach(btn => {
            btn.classList.toggle('active', btn.dataset.panel === panelName);
        });

        // Show/hide panels
        document.querySelectorAll('[data-panel]').forEach(panel => {
            panel.style.display = panel.dataset.panel === panelName ? 'flex' : 'none';
        });

        // If switching to browser, focus webview area
        if (panelName === 'browser') {
            const container = document.getElementById('webview-container');
            if (container) container.focus();
        }

        this.saveState();
    }

    updateSidebar(activeBtn) {
        document.querySelectorAll('.sidebar-btn').forEach(btn => {
            btn.classList.remove('active');
        });
        if (activeBtn) {
            activeBtn.classList.add('active');
        }
    }

    renderTabs() {
        const tabBar = document.getElementById('tab-bar');
        if (!tabBar) return;

        tabBar.innerHTML = '';
        
        this.tabs.forEach((tab, idx) => {
            const tabEl = document.createElement('div');
            tabEl.className = `tab ${idx === this.currentTab ? 'active' : ''}`;
            tabEl.dataset.index = idx;
            
            const icon = tab.pinned ? '📌' : (tab.incognito ? '🕵️' : '');
            const title = tab.title || 'New Tab';
            
            tabEl.innerHTML = `
                <span class="tab-title">${icon} ${title}</span>
                <button class="tab-close">✕</button>
            `;
            
            tabEl.addEventListener('click', () => this.selectTab(idx));
            tabEl.querySelector('.tab-close').addEventListener('click', (e) => {
                e.stopPropagation();
                this.closeTab(idx);
            });
            
            tabBar.appendChild(tabEl);
        });

        // Ensure new tab button exists
        if (!tabBar.querySelector('.new-tab-btn')) {
            const newTabBtn = document.createElement('button');
            newTabBtn.className = 'new-tab-btn';
            newTabBtn.textContent = '➕';
            newTabBtn.addEventListener('click', () => this.newTab());
            tabBar.appendChild(newTabBtn);
        }
    }

    renderWebView() {
        const container = document.getElementById('webview-container');
        if (!container) return;

        const tab = this.tabs[this.currentTab];
        const url = tab?.url || 'https://www.google.com';
        
        // Update address bar
        const urlInput = document.getElementById('url-input');
        if (urlInput) {
            urlInput.value = url;
        }
        
        // Invoke Tauri command to navigate
        invoke('navigate', { url });

        // Display current page info
        container.innerHTML = `
            <div style="display: flex; flex-direction: column; justify-content: center; align-items: center; height: 100%; background: linear-gradient(135deg, rgba(30,30,50,0.9), rgba(40,40,60,0.9)); color: #aaa; border-radius: 8px; padding: 20px; text-align: center;">
                <div style="font-size: 48px; margin-bottom: 15px;">🌐</div>
                <div style="font-weight: 600; font-size: 16px; color: #fff; margin-bottom: 10px;">${tab.title || 'Loading...'}</div>
                <div style="font-family: monospace; font-size: 12px; margin-bottom: 15px; opacity: 0.8; max-width: 400px; word-break: break-all;">${url}</div>
                <div style="font-size: 12px; opacity: 0.6;">WebView2 (Windows) • WKWebView (macOS) • WebKit (Linux)</div>
            </div>
        `;
    }
    
    navigate(url) {
        const normalized = this.normalizeUrl(url);
        const tab = this.tabs[this.currentTab];
        
        if (tab) {
            tab.url = normalized;
            tab.title = this.getTitleFromUrl(normalized);
            tab.history.push(normalized);
            tab.history_pos = tab.history.length - 1;
        }
        
        this.renderTabs();
        this.renderWebView();
        this.saveState();
        invoke('navigate', { url: normalized });
    }

    getTitleFromUrl(url) {
        try {
            const u = new URL(url);
            return u.hostname || url;
        } catch {
            return url;
        }
    }

    normalizeUrl(input) {
        let url = input.trim();
        
        if (url.startsWith('http://') || url.startsWith('https://') || 
            url.startsWith('about:') || url.startsWith('data:') ||
            url.startsWith('file:')) {
            return url;
        }
        
        if (url.includes('.') && !url.includes(' ')) {
            return 'https://' + url;
        }
        
        return 'https://www.google.com/search?q=' + encodeURIComponent(url);
    }

    selectTab(idx) {
        this.currentTab = idx;
        this.renderTabs();
        this.renderWebView();
        this.saveState();
    }

    newTab() {
        this.tabs.push({
            url: 'https://www.google.com',
            title: 'Google',
            pinned: false,
            incognito: false,
            history: ['https://www.google.com'],
            history_pos: 0
        });
        this.currentTab = this.tabs.length - 1;
        this.renderTabs();
        this.renderWebView();
        this.saveState();
    }

    newIncognitoTab() {
        this.tabs.push({
            url: 'https://www.google.com',
            title: 'Google (Incognito)',
            pinned: false,
            incognito: true,
            history: ['https://www.google.com'],
            history_pos: 0
        });
        this.currentTab = this.tabs.length - 1;
        this.renderTabs();
        this.renderWebView();
        this.saveState();
    }

    closeTab(idx) {
        if (this.tabs.length === 1) return;
        this.tabs.splice(idx, 1);
        if (this.currentTab >= this.tabs.length) {
            this.currentTab = this.tabs.length - 1;
        }
        this.renderTabs();
        this.renderWebView();
        this.saveState();
    }

    duplicateTab() {
        const current = this.tabs[this.currentTab];
        if (!current) return;

        const duplicate = {
            url: current.url,
            title: current.title,
            pinned: false,
            incognito: current.incognito,
            history: [...current.history],
            history_pos: current.history_pos
        };
        
        this.tabs.push(duplicate);
        this.currentTab = this.tabs.length - 1;
        this.renderTabs();
        this.renderWebView();
        this.saveState();
        invoke('duplicate_tab', { index: this.currentTab });
    }

    closeOtherTabs() {
        const current = this.tabs[this.currentTab];
        this.tabs = [current];
        this.currentTab = 0;
        this.renderTabs();
        this.renderWebView();
        this.saveState();
    }

    pinTab(idx) {
        if (this.tabs[idx]) {
            this.tabs[idx].pinned = !this.tabs[idx].pinned;
            this.renderTabs();
            this.saveState();
            invoke('pin_tab', { index: idx });
        }
    }

    toggleIncognito() {
        const tab = this.tabs[this.currentTab];
        if (tab) {
            tab.incognito = !tab.incognito;
            tab.title = tab.incognito ? tab.title + ' (Incognito)' : tab.title.replace(' (Incognito)', '');
            this.renderTabs();
            this.saveState();
            invoke('toggle_incognito');
        }
    }

    openQuickSearch() {
        const searchInput = prompt('Quick Search (supports math):', '');
        if (searchInput) {
            invoke('evaluate_expression', { expr: searchInput }).then(result => {
                if (result !== null) {
                    this.navigate('https://www.google.com/search?q=' + encodeURIComponent(result.toString()));
                }
            }).catch(err => {
                this.navigate('https://www.google.com/search?q=' + encodeURIComponent(searchInput));
            });
        }
    }

    toggleAdblock() {
        this.adblockEnabled = !this.adblockEnabled;
        this.saveState();
        invoke('toggle_adblock');
    }

    toggleVpn() {
        this.vpnEnabled = !this.vpnEnabled;
        this.saveState();
        invoke('toggle_vpn');
    }

    setTheme(theme) {
        this.theme = theme;
        this.applyTheme();
        this.saveState();
        invoke('set_theme', { theme });
    }

    applyTheme() {
        const root = document.documentElement;
        if (this.theme === 'light') {
            root.style.filter = 'invert(1)';
        } else {
            root.style.filter = 'none';
        }
    }

    navigate(url) {
        const normalized = this.normalizeUrl(url);
        const tab = this.tabs[this.currentTab];
        
        if (tab) {
            tab.url = normalized;
            tab.title = this.getTitleFromUrl(normalized);
            tab.history.push(normalized);
            tab.history_pos = tab.history.length - 1;
        }
        
        this.renderTabs();
        this.renderWebView();
        this.saveState();
        invoke('navigate', { url: normalized });
    }

    async saveState() {
        try {
            await invoke('save_state', {
                tabs: this.tabs,
                current_tab: this.currentTab,
                theme: this.theme,
                adblock_enabled: this.adblockEnabled,
                vpn_enabled: this.vpnEnabled
            });
        } catch (e) {
            console.error('Failed to save state:', e);
        }
    }
}

// Initialize app when DOM is ready
window.addEventListener('DOMContentLoaded', () => {
    window.hyperBrowser = new HyprBrowser();
});
