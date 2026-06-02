// ─── < Imports > ────────────────────────────────────────────────────

use std::env;
use std::path::PathBuf;

use super::ConfigError;

// ─── < Public Functions > ───────────────────────────────────────────

pub fn runtime_config_source_path() -> PathBuf {
    PathBuf::from("runtime defaults + ARC_* environment")
}

pub fn default_user_policies_dir() -> Result<PathBuf, ConfigError> {
    let home = env::var("HOME").map_err(|source| ConfigError::MissingHome { source })?;

    Ok(PathBuf::from(home).join(".config").join("arc").join("policies.d"))
}

pub fn default_user_policy_path() -> Result<PathBuf, ConfigError> {
    Ok(default_user_policies_dir()?.join("arc.rego"))
}
