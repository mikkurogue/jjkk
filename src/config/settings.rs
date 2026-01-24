use std::path::PathBuf;

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Settings {
    #[serde(default)]
    pub theme: ThemeSettings,
    #[serde(default)]
    pub ui: UiSettings,
    #[serde(default)]
    pub auto_track_local: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeSettings {
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiSettings {
    #[serde(default)]
    pub diff_context_lines: usize,
    #[serde(default)]
    pub visible_diff_lines: usize,
    #[serde(default)]
    pub log_commits_count:  usize,
}

impl Default for ThemeSettings {
    fn default() -> Self {
        Self {
            name: "catppuccin-mocha".to_owned(),
        }
    }
}

impl Default for UiSettings {
    fn default() -> Self {
        Self {
            diff_context_lines: 3,
            visible_diff_lines: 30,
            log_commits_count:  100,
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
        let settings: Self = toml::from_str(&content)?;
        Ok(settings)
    }

    pub fn config_path() -> anyhow::Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?;
        Ok(config_dir.join("jjkk").join("config.toml"))
    }
}
