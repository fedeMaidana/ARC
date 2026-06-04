// ─── < Modules > ────────────────────────────────────────────────────

mod discovery;
mod registry;
mod source;

// ─── < Public Exports > ─────────────────────────────────────────────

pub use self::discovery::{AgentCandidate, AgentDiscovery, AgentScan, MissingKnownAgent, scan_installed_agents, scan_known_agents};
pub use self::registry::{
    AgentRegistration, AgentRegistry, AgentRegistrySyncReport, load_agent_registry, save_agent_registry, sync_agent_registry,
};
pub use self::source::{AgentSource, AgentSourceError, AgentSourceStatus, classify_source, resolve_source_from_environment};
