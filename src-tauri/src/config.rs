use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

pub const DEFAULT_CONFIG_CONTENT: &str = r#"# AntiVadim Configuration

# Target URL to display in kiosk mode
url = "https://foxford.ru"

# Global shortcut to exit the application
exit_shortcut = "CmdOrCtrl+Shift+P"
"#;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
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

    pub fn load_from_path(primary_path: &Path) -> Self {
        Self::auto_seed_if_missing(primary_path);

        if primary_path.is_file() {
            if let Ok(content) = fs::read_to_string(primary_path) {
                if let Ok(config) = Self::parse_toml(&content) {
                    return config;
                }
            }
        }

        // Fallback: Built-in defaults
        Self::parse_toml(DEFAULT_CONFIG_CONTENT).unwrap()
    }

    pub fn load() -> Self {
        // 1. Primary config: ~/.config/antivadim/config.toml (auto-seeded if missing)
        if let Some(primary_path) = Self::primary_config_path() {
            return Self::load_from_path(&primary_path);
        }

        // 2. Fallback: Built-in defaults
        Self::parse_toml(DEFAULT_CONFIG_CONTENT).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use tauri_plugin_global_shortcut::Shortcut;
    use tempfile::tempdir;

    #[test]
    fn test_default_config_content_parses_successfully() {
        let config = AppConfig::parse_toml(DEFAULT_CONFIG_CONTENT)
            .expect("DEFAULT_CONFIG_CONTENT must be valid TOML");

        assert_eq!(config.url, "https://foxford.ru");
        assert_eq!(config.exit_shortcut, "CmdOrCtrl+Shift+P");
    }

    #[test]
    fn test_parse_valid_custom_toml() {
        let custom_toml = r#"
url = "https://example.com/custom"
exit_shortcut = "Ctrl+Shift+Q"
"#;
        let config = AppConfig::parse_toml(custom_toml)
            .expect("Custom valid TOML should parse without errors");

        assert_eq!(config.url, "https://example.com/custom");
        assert_eq!(config.exit_shortcut, "Ctrl+Shift+Q");
    }

    #[test]
    fn test_parse_toml_missing_url() {
        let toml_missing_url = r#"
exit_shortcut = "Ctrl+Shift+Q"
"#;
        let result = AppConfig::parse_toml(toml_missing_url);
        assert!(result.is_err(), "Parsing TOML without `url` must fail");
    }

    #[test]
    fn test_parse_toml_missing_exit_shortcut() {
        let toml_missing_shortcut = r#"
url = "https://example.com"
"#;
        let result = AppConfig::parse_toml(toml_missing_shortcut);
        assert!(
            result.is_err(),
            "Parsing TOML without `exit_shortcut` must fail"
        );
    }

    #[test]
    fn test_parse_toml_invalid_syntax() {
        let invalid_toml = r#"
url = "https://example.com
exit_shortcut = 12345
"#;
        let result = AppConfig::parse_toml(invalid_toml);
        assert!(
            result.is_err(),
            "Parsing malformed TOML syntax must return an error"
        );
    }

    #[test]
    fn test_primary_config_path_structure() {
        if let Some(path) = AppConfig::primary_config_path() {
            assert!(
                path.ends_with(".config/antivadim/config.toml"),
                "Primary config path must target .config/antivadim/config.toml, got: {:?}",
                path
            );
        }
    }

    #[test]
    fn test_auto_seed_creates_file_and_parent_directories() {
        let dir = tempdir().expect("Failed to create temp dir");
        let target_file = dir.path().join("nested").join("sub").join("config.toml");

        assert!(!target_file.exists());
        AppConfig::auto_seed_if_missing(&target_file);

        assert!(target_file.exists());
        assert!(target_file.is_file());

        let written_content =
            fs::read_to_string(&target_file).expect("Failed to read seeded config");
        assert_eq!(written_content, DEFAULT_CONFIG_CONTENT);
    }

    #[test]
    fn test_auto_seed_does_not_overwrite_existing_file() {
        let dir = tempdir().expect("Failed to create temp dir");
        let target_file = dir.path().join("config.toml");
        let custom_content = "url = \"https://custom.org\"\nexit_shortcut = \"Alt+F4\"\n";

        fs::write(&target_file, custom_content).expect("Failed to write initial file");

        AppConfig::auto_seed_if_missing(&target_file);

        let current_content =
            fs::read_to_string(&target_file).expect("Failed to read target file");
        assert_eq!(
            current_content, custom_content,
            "auto_seed_if_missing must not overwrite existing file content"
        );
    }

    #[test]
    fn test_load_from_path_reads_valid_custom_config() {
        let dir = tempdir().expect("Failed to create temp dir");
        let target_file = dir.path().join("config.toml");
        let custom_content = r#"
url = "https://internal.dashboard.local"
exit_shortcut = "Ctrl+Alt+Escape"
"#;
        fs::write(&target_file, custom_content).expect("Failed to write custom config");

        let loaded = AppConfig::load_from_path(&target_file);
        assert_eq!(loaded.url, "https://internal.dashboard.local");
        assert_eq!(loaded.exit_shortcut, "Ctrl+Alt+Escape");
    }

    #[test]
    fn test_load_from_path_missing_file_seeds_and_returns_default() {
        let dir = tempdir().expect("Failed to create temp dir");
        let target_file = dir.path().join("auto_seeded_config.toml");

        assert!(!target_file.exists());
        let loaded = AppConfig::load_from_path(&target_file);

        assert!(target_file.exists());
        assert_eq!(loaded.url, "https://foxford.ru");
        assert_eq!(loaded.exit_shortcut, "CmdOrCtrl+Shift+P");
    }

    #[test]
    fn test_load_from_path_corrupted_file_falls_back_to_defaults() {
        let dir = tempdir().expect("Failed to create temp dir");
        let target_file = dir.path().join("corrupted.toml");
        fs::write(&target_file, "this is not valid toml = [[[").expect("Failed to write corrupted config");

        let loaded = AppConfig::load_from_path(&target_file);
        assert_eq!(
            loaded.url, "https://foxford.ru",
            "Corrupted config must safely fall back to default URL"
        );
        assert_eq!(
            loaded.exit_shortcut, "CmdOrCtrl+Shift+P",
            "Corrupted config must safely fall back to default shortcut"
        );
    }

    #[test]
    fn test_shortcut_parsing_compatibility() {
        // Test default shortcut
        let default_config = AppConfig::parse_toml(DEFAULT_CONFIG_CONTENT).unwrap();
        let parsed = Shortcut::from_str(&default_config.exit_shortcut);
        assert!(
            parsed.is_ok(),
            "Default exit shortcut '{}' must be valid for Shortcut::from_str",
            default_config.exit_shortcut
        );

        // Test common valid shortcuts
        for shortcut_str in &[
            "CmdOrCtrl+Shift+P",
            "Ctrl+Shift+Q",
            "Alt+F4",
            "Command+Q",
            "Control+Shift+X",
        ] {
            assert!(
                Shortcut::from_str(shortcut_str).is_ok(),
                "Shortcut '{}' should parse successfully",
                shortcut_str
            );
        }

        // Test invalid shortcut format
        let invalid = Shortcut::from_str("NotAValidShortcutString!@#");
        assert!(
            invalid.is_err(),
            "Malformed shortcut string should return an error"
        );
    }

    #[test]
    fn test_url_parsing_compatibility() {
        let default_config = AppConfig::parse_toml(DEFAULT_CONFIG_CONTENT).unwrap();
        let parsed_url = url::Url::parse(&default_config.url);
        assert!(
            parsed_url.is_ok(),
            "Default URL '{}' must be valid URL format",
            default_config.url
        );

        let valid_urls = [
            "https://foxford.ru",
            "http://localhost:3000",
            "https://app.example.com/kiosk?fullscreen=true",
        ];
        for u in valid_urls {
            assert!(url::Url::parse(u).is_ok(), "URL '{}' should be valid", u);
        }

        let invalid_urls = ["not_a_valid_url", "://missing-scheme", ""];
        for u in invalid_urls {
            assert!(
                url::Url::parse(u).is_err(),
                "Invalid URL '{}' should fail to parse",
                u
            );
        }
    }
}
