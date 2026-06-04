// ─── < Imports > ────────────────────────────────────────────────────

use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::config::{AgentSourceConfig, ConfigError};

use super::discovery::{AgentDiscovery, scan_installed_agents};

// ─── < Constants > ──────────────────────────────────────────────────

const AGENT_REGISTRY_SCHEMA_VERSION: u32 = 1;
const DETECTED_SOURCE: &str = "detected";

// ─── < Structs > ────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentRegistry {
    #[serde(default = "default_schema_version")]
    schema_version: u32,

    #[serde(default)]
    agents: Vec<AgentRegistration>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentRegistration {
    id: String,
    display_name: String,
    kind: String,
    enabled: bool,
    command: String,
    path: String,
    source: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentRegistrySyncReport {
    registry_path: String,
    detected_count: usize,
    registered_count: usize,
    added_count: usize,
    updated_count: usize,
}

// ─── < Public Functions > ───────────────────────────────────────────

pub fn load_agent_registry(path: &Path) -> Result<AgentRegistry, ConfigError> {
    if !path.exists() {
        return Ok(AgentRegistry::default());
    }

    let content = fs::read_to_string(path).map_err(|source| ConfigError::Read {
        path: path.display().to_string(),
        source,
    })?;

    if content.trim().is_empty() {
        return Ok(AgentRegistry::default());
    }

    serde_json::from_str(&content).map_err(|source| ConfigError::Json {
        path: path.display().to_string(),
        source,
    })
}

pub fn save_agent_registry(path: &Path, registry: &AgentRegistry) -> Result<(), ConfigError> {
    let Some(parent_dir) = path.parent() else {
        return Err(ConfigError::MissingParent {
            path: path.display().to_string(),
        });
    };

    fs::create_dir_all(parent_dir).map_err(|source| ConfigError::CreateDir {
        path: parent_dir.display().to_string(),
        source,
    })?;

    let content = serde_json::to_string_pretty(registry).map_err(|source| ConfigError::Json {
        path: path.display().to_string(),
        source,
    })?;

    fs::write(path, format!("{content}\n")).map_err(|source| ConfigError::Write {
        path: path.display().to_string(),
        source,
    })
}

pub fn sync_agent_registry(path: &Path) -> Result<AgentRegistrySyncReport, ConfigError> {
    let scan = scan_installed_agents();
    let detected_agents = scan.detected_agents();

    let mut registry = load_agent_registry(path)?;
    let mut added_count = 0;
    let mut updated_count = 0;

    for detected_agent in detected_agents {
        match registry.upsert_detected_agent(detected_agent) {
            RegistryUpsertResult::Added => added_count += 1,
            RegistryUpsertResult::Updated => updated_count += 1,
            RegistryUpsertResult::Unchanged => {}
        }
    }

    registry.sort();

    save_agent_registry(path, &registry)?;

    Ok(AgentRegistrySyncReport {
        registry_path: path.display().to_string(),
        detected_count: detected_agents.len(),
        registered_count: registry.agents.len(),
        added_count,
        updated_count,
    })
}

// ─── < Enums > ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RegistryUpsertResult {
    Added,
    Updated,
    Unchanged,
}

// ─── < Implementations > ────────────────────────────────────────────

impl Default for AgentRegistry {
    fn default() -> Self {
        Self {
            schema_version: AGENT_REGISTRY_SCHEMA_VERSION,
            agents: Vec::new(),
        }
    }
}

impl AgentRegistry {
    pub fn schema_version(&self) -> u32 {
        self.schema_version
    }

    pub fn agents(&self) -> &[AgentRegistration] {
        &self.agents
    }

    pub fn to_agent_sources(&self) -> Vec<AgentSourceConfig> {
        self.agents.iter().map(AgentRegistration::to_agent_source).collect()
    }

    fn upsert_detected_agent(&mut self, detected_agent: &AgentDiscovery) -> RegistryUpsertResult {
        let registration = AgentRegistration::from_detected_agent(detected_agent);

        if let Some(existing_agent) = self.agents.iter_mut().find(|agent| agent.id == registration.id) {
            if existing_agent == &registration {
                return RegistryUpsertResult::Unchanged;
            }

            *existing_agent = registration;

            return RegistryUpsertResult::Updated;
        }

        self.agents.push(registration);

        RegistryUpsertResult::Added
    }

    fn sort(&mut self) {
        self.agents.sort_by(|left, right| left.id.cmp(&right.id));
    }
}

impl AgentRegistration {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    pub fn kind(&self) -> &str {
        &self.kind
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn command(&self) -> &str {
        &self.command
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    fn from_detected_agent(agent: &AgentDiscovery) -> Self {
        Self {
            id: agent.id().to_string(),
            display_name: agent.display_name().to_string(),
            kind: "local_agent".to_string(),
            enabled: true,
            command: agent.detected_command().to_string(),
            path: agent.path().display().to_string(),
            source: DETECTED_SOURCE.to_string(),
        }
    }

    fn to_agent_source(&self) -> AgentSourceConfig {
        AgentSourceConfig {
            id: self.id.clone(),
            display_name: self.display_name.clone(),
            enabled: self.enabled,
            kind: self.kind.clone(),
            description: Some(format!("{} command: {} ({})", capitalize_source(&self.source), self.command, self.path)),
        }
    }
}

impl AgentRegistrySyncReport {
    pub fn registry_path(&self) -> &str {
        &self.registry_path
    }

    pub fn detected_count(&self) -> usize {
        self.detected_count
    }

    pub fn registered_count(&self) -> usize {
        self.registered_count
    }

    pub fn added_count(&self) -> usize {
        self.added_count
    }

    pub fn updated_count(&self) -> usize {
        self.updated_count
    }
}

// ─── < Private Functions > ──────────────────────────────────────────

fn default_schema_version() -> u32 {
    AGENT_REGISTRY_SCHEMA_VERSION
}

fn capitalize_source(source: &str) -> String {
    let mut chars = source.chars();

    let Some(first) = chars.next() else {
        return String::new();
    };

    let mut capitalized = String::new();

    capitalized.extend(first.to_uppercase());
    capitalized.push_str(chars.as_str());

    capitalized
}
