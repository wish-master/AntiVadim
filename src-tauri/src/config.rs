use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

pub const DEFAULT_CONFIG_CONTENT: &str = r#"# AntiVadim Configuration

# Target URL to display in kiosk mode
url = "https://foxford.ru"

# Global shortcut to exit the application
exit_shortcut = "CmdOrCtrl+Shift+P"
"#;

#[derive(Deserialize)]
pub struct AppConfig {
    pub url: String,
    pub exit_shortcut: String,
}

impl AppConfig {
    pub fn parse_toml(content: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(content)
    }

    pub fn primary_config_path() -> Option<PathBuf> {
        std::env::var_os("HOME").map(|home| {
            PathBuf::from(home)
                .join(".config")
                .join("antivadim")
                .join("config.toml")
        })
    }

    pub fn auto_seed_if_missing(path: &Path) {
        if path.exists() {
            return;
        }

        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        let _ = fs::write(path, DEFAULT_CONFIG_CONTENT);
    }

    pub fn load() -> Self {
        // 1. Primary config: ~/.config/antivadim/config.toml (auto-seeded if missing)
        if let Some(primary_path) = Self::primary_config_path() {
            Self::auto_seed_if_missing(&primary_path);

            if primary_path.is_file() {
                if let Ok(content) = fs::read_to_string(&primary_path) {
                    if let Ok(config) = Self::parse_toml(&content) {
                        return config;
                    }
                }
            }
        }

        // 2. Fallback: Built-in defaults
        Self::parse_toml(DEFAULT_CONFIG_CONTENT).unwrap()
    }
}
