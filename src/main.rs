use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

slint::include_modules!();

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub theme: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: "light".to_string(),
        }
    }
}

fn get_config_path() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("slint-hmi-app");
    fs::create_dir_all(&path).ok();
    path.push("config.json");
    path
}

fn load_config() -> AppConfig {
    let config_path = get_config_path();
    if let Ok(content) = fs::read_to_string(config_path) {
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        AppConfig::default()
    }
}

fn save_config(config: &AppConfig) {
    let config_path = get_config_path();
    if let Ok(content) = serde_json::to_string_pretty(config) {
        fs::write(config_path, content).ok();
    }
}

fn main() -> Result<(), slint::PlatformError> {
    // Enable hot reload in debug mode
    #[cfg(debug_assertions)]
    {
        println!("🔥 Hot reload enabled! Edit .slint files and see changes instantly.");
    }

    let ui = AppWindow::new()?;
    
    // Load saved configuration
    let config = load_config();
    ui.set_current_theme(config.theme.clone().into());
    
    // Set up theme change callback
    let ui_handle = ui.as_weak();
    ui.on_theme_changed(move |theme| {
        let config = AppConfig {
            theme: theme.to_string(),
        };
        save_config(&config);
        
        if let Some(ui) = ui_handle.upgrade() {
            ui.set_current_theme(theme);
        }
    });
    
    ui.run()
}
