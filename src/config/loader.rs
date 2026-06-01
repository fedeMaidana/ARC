// ─── < Imports > ────────────────────────────────────────────────────

use std::fs;
use std::path::{Path, PathBuf};

use super::{Config, ConfigError, resolve_config_path};

// ─── < Public Functions > ───────────────────────────────────────────

pub fn load(path: impl AsRef<Path>) -> Result<Config, ConfigError> {
    let path = path.as_ref();
    let path_display = path.display().to_string();

    let content = fs::read_to_string(path).map_err(|source| ConfigError::Read {
        path: path_display.clone(),
        source,
    })?;

    toml::from_str(&content).map_err(|source| ConfigError::Parse { path: path_display, source })
}

pub fn load_from_default_locations() -> Result<(Config, PathBuf), ConfigError> {
    let Some(path) = resolve_config_path() else {
        return Err(ConfigError::NotFound);
    };

    let config = load(&path)?;

    Ok((config, path))
}
