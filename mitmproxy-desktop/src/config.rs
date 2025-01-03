use anyhow::Result;
use directories::ProjectDirs;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

static PROJECT_DIRS: Lazy<ProjectDirs> = Lazy::new(|| {
    ProjectDirs::from("org", "mitmproxy", "desktop")
        .expect("Failed to determine project directories")
});

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub last_port: Option<u16>,
    pub last_certificate_path: Option<String>,
    pub theme: Theme,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            last_port: Some(8080),
            last_certificate_path: None,
            theme: Theme::Dark,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        if !config_path.exists() {
            return Ok(Self::default());
        }

        let contents = std::fs::read_to_string(config_path)?;
        Ok(serde_json::from_str(&contents)?)
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let contents = serde_json::to_string_pretty(self)?;
        std::fs::write(config_path, contents)?;
        Ok(())
    }

    fn config_path() -> Result<PathBuf> {
        Ok(PROJECT_DIRS.config_dir().join("config.json"))
    }
}
