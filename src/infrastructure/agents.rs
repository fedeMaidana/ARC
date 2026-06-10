// ─── < Modules > ────────────────────────────────────────────────────

mod classification;
mod discovery;
mod registry;

// ─── < Public Exports > ─────────────────────────────────────────────

pub use self::classification::{AgentSourceError, classify_source, resolve_source_from_environment};
pub use self::discovery::{AgentCandidate, AgentDiscovery, AgentScan, MissingKnownAgent, scan_installed_agents, scan_known_agents};
pub use self::registry::{
    AgentRegistration, AgentRegistry, AgentRegistrySyncReport, load_agent_registry, save_agent_registry, sync_agent_registry,
};
