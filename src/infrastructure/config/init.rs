// ─── < Imports > ────────────────────────────────────────────────────

use std::fs;
use std::path::PathBuf;

use crate::agent::{AgentRegistrySyncReport, sync_agent_registry};

use super::{ConfigError, default_user_agent_registry_path, default_user_policy_path};

// ─── < Constants > ──────────────────────────────────────────────────

const DEFAULT_REGO_POLICY_CONTENT: &str = include_str!("../../../examples/rego/arc.rego");

// ─── < Structs > ────────────────────────────────────────────────────

pub struct ConfigInitResult {
    policy: PolicyInitResult,
    agent_registry: AgentRegistrySyncReport,
}

// ─── < Enums > ──────────────────────────────────────────────────────

pub enum PolicyInitResult {
    Created(PathBuf),
    AlreadyExists(PathBuf),
}

// ─── < Public Functions > ───────────────────────────────────────────

pub fn init_default_config() -> Result<ConfigInitResult, ConfigError> {
    let policy = init_default_policy()?;
    let registry_path = default_user_agent_registry_path()?;
    let agent_registry = sync_agent_registry(&registry_path)?;

    Ok(ConfigInitResult { policy, agent_registry })
}

// ─── < Implementations > ────────────────────────────────────────────

impl ConfigInitResult {
    pub fn policy(&self) -> &PolicyInitResult {
        &self.policy
    }

    pub fn agent_registry(&self) -> &AgentRegistrySyncReport {
        &self.agent_registry
    }
}

// ─── < Private Functions > ──────────────────────────────────────────

fn init_default_policy() -> Result<PolicyInitResult, ConfigError> {
    let path = default_user_policy_path()?;

    if path.exists() {
        return Ok(PolicyInitResult::AlreadyExists(path));
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

    fs::write(&path, DEFAULT_REGO_POLICY_CONTENT).map_err(|source| ConfigError::Write {
        path: path.display().to_string(),
        source,
    })?;

    Ok(PolicyInitResult::Created(path))
}
