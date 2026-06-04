// ─── < Imports > ────────────────────────────────────────────────────

use std::env;
use std::path::PathBuf;

use super::ConfigError;

// ─── < Public Functions > ───────────────────────────────────────────

pub fn runtime_config_source_path() -> PathBuf {
    PathBuf::from("runtime defaults + agent registry + ARC_* environment")
}

pub fn default_user_data_dir() -> Result<PathBuf, ConfigError> {
    let home = env::var("HOME").map_err(|source| ConfigError::MissingHome { source })?;

    Ok(PathBuf::from(home).join(".local").join("share").join("arc"))
}

pub fn default_user_agent_registry_path() -> Result<PathBuf, ConfigError> {
    if let Some(path) = optional_env_path("ARC_AGENT_REGISTRY_PATH") {
        return Ok(path);
    }

    Ok(default_user_data_dir()?.join("agents.json"))
}

pub fn default_user_policies_dir() -> Result<PathBuf, ConfigError> {
    let home = env::var("HOME").map_err(|source| ConfigError::MissingHome { source })?;

    Ok(PathBuf::from(home).join(".config").join("arc").join("policies.d"))
}

pub fn default_user_policy_path() -> Result<PathBuf, ConfigError> {
    Ok(default_user_policies_dir()?.join("arc.rego"))
}

// ─── < Private Functions > ──────────────────────────────────────────

fn optional_env_path(name: &str) -> Option<PathBuf> {
    env::var(name)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
}
