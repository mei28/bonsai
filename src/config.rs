use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::{BonsaiError, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub version: String,
    #[serde(default)]
    pub defaults: Defaults,
    #[serde(default)]
    pub hooks: Hooks,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Defaults {
    pub worktree_dir: String,
}

impl Default for Defaults {
    fn default() -> Self {
        Self {
            worktree_dir: ".bonsai".to_string(),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Hooks {
    #[serde(default)]
    pub post_create: Vec<Hook>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Hook {
    #[serde(rename = "copy")]
    Copy { from: String, to: String },
    #[serde(rename = "symlink")]
    Symlink { from: String, to: String },
    #[serde(rename = "command")]
    Command {
        command: String,
        #[serde(default)]
        env: HashMap<String, String>,
    },
}

impl Config {
    pub fn default_config() -> Self {
        Self {
            version: "1".to_string(),
            defaults: Defaults::default(),
            hooks: Hooks::default(),
        }
    }

    pub fn load(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path).map_err(|_| BonsaiError::NotInitialized)?;
        toml::from_str(&content)
            .map_err(|e| BonsaiError::Config(format!("failed to parse {}: {e}", path.display())))
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| BonsaiError::Config(format!("failed to serialize config: {e}")))?;
        std::fs::write(path, content)?;
        Ok(())
    }
}
