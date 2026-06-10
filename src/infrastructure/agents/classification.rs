// ─── < Imports > ────────────────────────────────────────────────────

use std::env;

use thiserror::Error;

use crate::config::AgentsConfig;
use crate::domain::agent::AgentSource;

// ─── < Errors > ─────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum AgentSourceError {
    #[error("agent source '{source_id}' is disabled")]
    Disabled { source_id: String },

    #[error("agent source '{source_id}' is not registered and agents.allow_unknown_sources=false")]
    Unknown { source_id: String },
}

// ─── < Public Functions > ───────────────────────────────────────────

pub fn resolve_source_from_environment(default_source: &str, agents: &AgentsConfig) -> Result<AgentSource, AgentSourceError> {
    let Some(source) = source_from_environment() else {
        return Ok(AgentSource::builtin(default_source));
    };

    classify_source(&source, agents)
}

pub fn classify_source(source: &str, agents: &AgentsConfig) -> Result<AgentSource, AgentSourceError> {
    let source = source.trim();

    if let Some(configured_source) = agents.sources.iter().find(|configured_source| configured_source.id == source) {
        if configured_source.enabled {
            return Ok(AgentSource::registered(source));
        }

        return Err(AgentSourceError::Disabled {
            source_id: source.to_string(),
        });
    }

    if agents.allow_unknown_sources {
        return Ok(AgentSource::unknown(source));
    }

    Err(AgentSourceError::Unknown {
        source_id: source.to_string(),
    })
}

// ─── < Private Functions > ──────────────────────────────────────────

fn source_from_environment() -> Option<String> {
    env::var("ARC_SOURCE")
        .ok()
        .map(|source| source.trim().to_string())
        .filter(|source| !source.is_empty())
}
