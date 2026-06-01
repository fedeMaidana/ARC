// ─── < Imports > ────────────────────────────────────────────────────

use std::fs;
use std::path::PathBuf;

use super::{ConfigError, default_user_config_path};

// ─── < Constants > ──────────────────────────────────────────────────

const DEFAULT_CONFIG_CONTENT: &str = include_str!("../../arc.default.toml");

// ─── < Enums > ──────────────────────────────────────────────────────

pub enum ConfigInitResult {
    Created(PathBuf),
    AlreadyExists(PathBuf),
}

// ─── < Public Functions > ───────────────────────────────────────────

pub fn init_default_config() -> Result<ConfigInitResult, ConfigError> {
    let path = default_user_config_path()?;

    if path.exists() {
        return Ok(ConfigInitResult::AlreadyExists(path));
    }

    let Some(parent_dir) = path.parent() else {
        return Err(ConfigError::MissingParent {
            path: path.display().to_string(),
        });
    };

    fs::create_dir_all(parent_dir).map_err(|source| ConfigError::CreateDir {
        path: parent_dir.display().to_string(),
        source,
    })?;

    fs::write(&path, DEFAULT_CONFIG_CONTENT).map_err(|source| ConfigError::Write {
        path: path.display().to_string(),
        source,
    })?;

    Ok(ConfigInitResult::Created(path))
}
