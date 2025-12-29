use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub name: String,
    pub version: String,
    pub repo: String,
    pub enabled: bool,
    pub installed: bool,
    pub author: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleGitHubResult {
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub html_url: String,
    pub owner: ModuleOwner,
    pub stars: u32,
    pub language: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleOwner {
    pub login: String,
    pub avatar_url: Option<String>,
}

impl Module {
    pub fn modules_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let exec_dir = std::env::current_exe()?
            .parent()
            .ok_or("Cannot determine executable directory")?
            .to_path_buf();
        Ok(exec_dir.join("modules"))
    }

    pub fn installed_modules() -> Result<Vec<Module>, Box<dyn std::error::Error>> {
        let path = Self::modules_path()?;
        let mut modules = Vec::new();

        if path.exists() {
            for entry in fs::read_dir(path)? {
                let entry = entry?;
                if entry.path().is_dir() {
                    let mod_info_path = entry.path().join("module.json");
                    if mod_info_path.exists() {
                        if let Ok(content) = fs::read_to_string(mod_info_path) {
                            if let Ok(module) = serde_json::from_str::<Module>(&content) {
                                modules.push(module);
                            }
                        }
                    }
                }
            }
        }
        
        Ok(modules)
    }
    
    pub fn install_from_repo(owner: &str, repo: &str) -> Result<(), Box<dyn std::error::Error>> {
        let modules_path = Self::modules_path()?;
        let module_path = modules_path.join(repo);
        
        fs::create_dir_all(&module_path)?;
        
        let module = Module {
            name: repo.to_string(),
            version: "1.0.0".to_string(),
            repo: format!("{}/{}", owner, repo),
            enabled: true,
            installed: true,
            author: owner.to_string(),
            description: format!("Module from {}/{}", owner, repo),
        };
        
        let config_path = module_path.join("module.json");
        fs::write(config_path, serde_json::to_string_pretty(&module)?)?;
        
        Ok(())
    }
    
    pub fn uninstall(module_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let modules_path = Self::modules_path()?;
        let module_path = modules_path.join(module_name);
        
        if module_path.exists() {
            fs::remove_dir_all(module_path)?;
        }
        
        Ok(())
    }
}
