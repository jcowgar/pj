use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// Directories to scan for projects
    pub scan_paths: Vec<PathBuf>,

    /// Project markers to look for (in order of preference)
    pub project_markers: Vec<String>,

    /// Maximum depth to scan
    #[serde(default = "default_max_depth")]
    pub max_depth: usize,
}

fn default_max_depth() -> usize {
    5
}

impl Default for Config {
    fn default() -> Self {
        Self {
            scan_paths: vec![
                dirs::home_dir()
                    .unwrap_or_else(|| PathBuf::from("."))
                    .join("Projects"),
            ],
            project_markers: vec![
                ".git".to_string(),
                ".jj".to_string(),
                ".hg".to_string(),
                ".project".to_string(),
            ],
            max_depth: 5,
        }
    }
}

impl Config {
    /// Load config from ~/.config/pj/config.toml or use defaults
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        if config_path.exists() {
            let contents =
                std::fs::read_to_string(&config_path).context("Failed to read config file")?;
            let config: Config =
                toml::from_str(&contents).context("Failed to parse config file")?;
            Ok(config)
        } else {
            Ok(Self::default())
        }
    }

    fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .context("Could not determine config directory")?
            .join("pj");

        Ok(config_dir.join("config.toml"))
    }

    /// Create a default config file
    pub fn create_default_config() -> Result<PathBuf> {
        let config_path = Self::config_path()?;
        let config_dir = config_path.parent().unwrap();

        std::fs::create_dir_all(config_dir).context("Failed to create config directory")?;

        let default_config = Self::default();
        let toml_string =
            toml::to_string_pretty(&default_config).context("Failed to serialize config")?;

        std::fs::write(&config_path, toml_string).context("Failed to write config file")?;

        Ok(config_path)
    }

    /// Load config from a specific TOML string (for testing)
    #[cfg(test)]
    pub fn from_toml_str(toml_str: &str) -> Result<Self> {
        Ok(toml::from_str(toml_str)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();

        assert_eq!(config.max_depth, 5);
        assert_eq!(config.scan_paths.len(), 1);
        assert_eq!(config.project_markers.len(), 4);
        assert!(config.project_markers.contains(&".git".to_string()));
        assert!(config.project_markers.contains(&".jj".to_string()));
        assert!(config.project_markers.contains(&".hg".to_string()));
        assert!(config.project_markers.contains(&".project".to_string()));
    }

    #[test]
    fn test_config_serialization() {
        let config = Config::default();
        let toml_str = toml::to_string(&config).unwrap();
        let deserialized: Config = toml::from_str(&toml_str).unwrap();

        assert_eq!(config.max_depth, deserialized.max_depth);
        assert_eq!(config.project_markers, deserialized.project_markers);
    }

    #[test]
    fn test_config_from_toml() {
        let toml_str = r#"
            scan_paths = ["/home/user/code", "/home/user/projects"]
            project_markers = [".git", "Cargo.toml"]
            max_depth = 3
        "#;

        let config = Config::from_toml_str(toml_str).unwrap();

        assert_eq!(config.scan_paths.len(), 2);
        assert_eq!(config.project_markers.len(), 2);
        assert_eq!(config.max_depth, 3);
    }

    #[test]
    fn test_config_with_default_max_depth() {
        let toml_str = r#"
            scan_paths = ["/home/user/code"]
            project_markers = [".git"]
        "#;

        let config = Config::from_toml_str(toml_str).unwrap();
        assert_eq!(config.max_depth, 5); // Should use default
    }

    #[test]
    fn test_config_missing_fields_fails() {
        let toml_str = r#"
            max_depth = 3
        "#;

        let result = Config::from_toml_str(toml_str);
        assert!(result.is_err()); // Missing required fields
    }
}
