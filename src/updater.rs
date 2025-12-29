use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInfo {
    pub current_version: String,
    pub latest_version: String,
    pub release_url: String,
    pub download_url: String,
    pub changelog: String,
    pub available: bool,
}

#[derive(Debug, Deserialize)]
pub struct GithubRelease {
    pub tag_name: String,
    pub body: String,
    pub assets: Vec<GithubAsset>,
    pub html_url: String,
}

#[derive(Debug, Deserialize)]
pub struct GithubAsset {
    pub name: String,
    pub browser_download_url: String,
}

impl UpdateInfo {
    pub fn current() -> Self {
        Self {
            current_version: env!("CARGO_PKG_VERSION").to_string(),
            latest_version: env!("CARGO_PKG_VERSION").to_string(),
            release_url: String::new(),
            download_url: String::new(),
            changelog: String::new(),
            available: false,
        }
    }

    pub async fn check() -> Result<Self, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let response = client
            .get("https://api.github.com/repos/pro-grammer-SD/hyprbrowser/releases/latest")
            .header("User-Agent", "HyprBrowser")
            .send()
            .await?;

        if !response.status().is_success() {
            return Ok(Self::current());
        }

        let release: GithubRelease = response.json().await?;
        let current = env!("CARGO_PKG_VERSION").to_string();
        let latest = release.tag_name.trim_start_matches('v').to_string();
        
        // Find release.zip in assets
        let mut download_url = String::new();
        for asset in &release.assets {
            if asset.name == "release.zip" || asset.name.ends_with(".zip") {
                download_url = asset.browser_download_url.clone();
                break;
            }
        }

        Ok(Self {
            current_version: current.clone(),
            latest_version: latest.clone(),
            release_url: release.html_url,
            download_url,
            changelog: release.body,
            available: latest > current,
        })
    }
    
    pub async fn download_update(&self) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let response = client.get(&self.download_url).send().await?;
        
        if !response.status().is_success() {
            return Err("Failed to download update".into());
        }
        
        let bytes = response.bytes().await?;
        
        // Save to temp directory
        let download_path = std::env::temp_dir().join(format!("hyprbrowser-{}.zip", self.latest_version));
        fs::write(&download_path, bytes)?;
        
        Ok(download_path)
    }
}
