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

fn load_text_file(file_path: &str) -> String {
    match fs::read_to_string(file_path) {
        Ok(content) => {
            let size = content.len();
            let lines = content.lines().count();
            format!("File: {} ({} bytes, {} lines)\n{}\n{}",
                file_path, size, lines,
                "=".repeat(50),
                content)
        },
        Err(e) => format!("❌ Error loading file '{}':\n{}\n\nPlease check that the file exists and is readable.", file_path, e),
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

    // Set up file loading callback
    ui.on_load_file(move |file_path| {
        load_text_file(&file_path.to_string()).into()
    });

    // Create a sample text file for demonstration
    let sample_file_path = "sample.txt";
    let sample_content = "# Sample Text File\n\nThis is a demonstration of the text file viewer card.\n\nFeatures:\n- Displays text file contents\n- Monospace font for code/data files\n- Theme-aware styling\n- Error handling for missing files\n\nYou can modify this file or create new ones to test the functionality.\n\nLorem ipsum dolor sit amet, consectetur adipiscing elit.\nSed do eiusmod tempor incididunt ut labore et dolore magna aliqua.\nUt enim ad minim veniam, quis nostrud exercitation ullamco laboris.";

    // Create sample file if it doesn't exist
    if !std::path::Path::new(sample_file_path).exists() {
        fs::write(sample_file_path, sample_content).ok();
    }

    // Load initial file content
    let initial_content = load_text_file(sample_file_path);

    // Set initial file content in the UI
    ui.set_file_content(initial_content.into());
    ui.set_file_path(sample_file_path.into());

    ui.run()
}
