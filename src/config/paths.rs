// ─── < Imports > ────────────────────────────────────────────────────

use std::env;
use std::path::PathBuf;

use super::ConfigError;

// ─── < Public Functions > ───────────────────────────────────────────

pub fn resolve_config_path() -> Option<PathBuf> {
    if let Ok(path) = env::var("ARC_CONFIG") {
        return Some(PathBuf::from(path));
    }

    let local_path = PathBuf::from("arc.toml");

    if local_path.exists() {
        return Some(local_path);
    }

    let Ok(global_path) = default_user_config_path() else {
        return None;
    };

    if global_path.exists() {
        return Some(global_path);
    }

    None
}

pub fn default_user_config_path() -> Result<PathBuf, ConfigError> {
    let home = env::var("HOME").map_err(|source| ConfigError::MissingHome { source })?;

    Ok(PathBuf::from(home).join(".config").join("arc").join("arc.toml"))
}
