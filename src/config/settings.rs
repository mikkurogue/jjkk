use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub theme: ThemeSettings,
    #[serde(default)]
    pub ui: UiSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeSettings {
    #[serde(default = "default_theme_name")]
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiSettings {
    #[serde(default = "default_diff_context_lines")]
    pub diff_context_lines: usize,
    #[serde(default = "default_visible_diff_lines")]
    pub visible_diff_lines: usize,
    #[serde(default = "default_log_commits_count")]
    pub log_commits_count: usize,
}

fn default_theme_name() -> String {
    "catppuccin-mocha".to_string()
}

fn default_diff_context_lines() -> usize {
    3
}

fn default_visible_diff_lines() -> usize {
    30
}

fn default_log_commits_count() -> usize {
    10
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: ThemeSettings::default(),
            ui: UiSettings::default(),
        }
    }
}

impl Default for ThemeSettings {
    fn default() -> Self {
        Self {
            name: default_theme_name(),
        }
    }
}

impl Default for UiSettings {
    fn default() -> Self {
        Self {
            diff_context_lines: default_diff_context_lines(),
            visible_diff_lines: default_visible_diff_lines(),
            log_commits_count: default_log_commits_count(),
        }
    }
}

impl Settings {
    pub fn load() -> anyhow::Result<Self> {
        let config_path = Self::config_path()?;

        if !config_path.exists() {
            return Ok(Self::default());
        }

        let content = std::fs::read_to_string(&config_path)?;
        let settings: Settings = toml::from_str(&content)?;
        Ok(settings)
    }

    pub fn config_path() -> anyhow::Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;
        Ok(config_dir.join("jjkk").join("config.toml"))
    }
}
