use serde_json::json;
use anyhow::Result;

pub struct ExampleModule {
    pub name: String,
    pub version: String,
    pub description: String,
    pub enabled: bool,
    pub dark_mode: bool,
}

impl ExampleModule {
    pub fn new() -> Self {
        ExampleModule {
            name: "Example Module".to_string(),
            version: "1.0.0".to_string(),
            description: "Example module demonstrating custom Tauri module".to_string(),
            enabled: true,
            dark_mode: false,
        }
    }

    /// Render module UI configuration as JSON for Tauri webview
    pub fn render_panel(&self) -> serde_json::Value {
        json!({
            "name": self.name,
            "version": self.version,
            "description": self.description,
            "enabled": self.enabled,
            "dark_mode": self.dark_mode,
            "settings": {
                "dark_theme": self.dark_mode,
            }
        })
    }

    pub fn on_key_press(&mut self, _key: u32) -> Option<String> {
        None
    }

    pub fn save_state(&self) -> Result<()> {
        let config = json!({
            "enabled": self.enabled,
            "dark_mode": self.dark_mode,
        });

        let config_path = std::path::PathBuf::from("data/modules/example.json");
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(config_path, serde_json::to_string_pretty(&config)?)?;

        Ok(())
    }

    pub fn load_state(&mut self) -> Result<()> {
        let config_path = std::path::PathBuf::from("data/modules/example.json");
        if config_path.exists() {
            let contents = std::fs::read_to_string(config_path)?;
            if let Ok(config) = serde_json::from_str::<serde_json::Value>(&contents) {
                if let Some(enabled) = config.get("enabled").and_then(|v| v.as_bool()) {
                    self.enabled = enabled;
                }
                if let Some(dark_mode) = config.get("dark_mode").and_then(|v| v.as_bool()) {
                    self.dark_mode = dark_mode;
                }
            }
        }
        Ok(())
    }
}

pub fn init() -> ExampleModule {
    ExampleModule::new()
}

fn main() {
    let mut module = init();
    if let Err(e) = module.load_state() {
        eprintln!("failed to load module state: {}", e);
    }
    
    // Render module UI configuration as JSON (sent to Tauri webview)
    let config = module.render_panel();
    
    if let Err(e) = module.save_state() {
        eprintln!("failed to save module state: {}", e);
    }
    
    println!("ExampleModule initialized");
    println!("Config: {}", serde_json::to_string_pretty(&config).unwrap());
    println!("dark_mode={}", module.dark_mode);
}
