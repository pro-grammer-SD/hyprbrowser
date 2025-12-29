use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;
use crate::downloads::Download;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub tabs: Vec<TabState>,
    pub current_tab: usize,
    pub theme: ThemeMode,
    pub adblock_enabled: bool,
    pub vpn_enabled: bool,
    pub downloads: Vec<Download>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TabState {
    pub url: String,
    pub title: String,
    pub pinned: bool,
    pub incognito: bool,
    pub history: Vec<String>,
    pub history_pos: usize,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ThemeMode {
    Light,
    Dark,
    System,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            tabs: vec![TabState {
                url: "https://www.google.com".to_string(),
                title: "Google".to_string(),
                pinned: false,
                incognito: false,
                history: vec!["https://www.google.com".to_string()],
                history_pos: 0,
            }],
            current_tab: 0,
            theme: ThemeMode::System,
            adblock_enabled: true,
            vpn_enabled: false,
            downloads: vec![],
        }
    }
}

impl AppState {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let state_path = Self::state_path()?;
        if state_path.exists() {
            let content = fs::read_to_string(state_path)?;
            Ok(serde_json::from_str(&content)?)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let state_path = Self::state_path()?;
        fs::create_dir_all(state_path.parent().unwrap())?;
        fs::write(state_path, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }

    fn state_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let exec_dir = std::env::current_exe()?
            .parent()
            .ok_or("Cannot determine executable directory")?
            .to_path_buf();
        Ok(exec_dir.join("data").join("state.json"))
    }
}
