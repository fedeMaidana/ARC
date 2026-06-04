// ─── < Modules > ────────────────────────────────────────────────────

mod discovery;
mod source;

// ─── < Public Exports > ─────────────────────────────────────────────

pub use self::discovery::{AgentCandidate, AgentDiscovery, AgentScan, MissingKnownAgent, scan_installed_agents, scan_known_agents};
pub use self::source::{AgentSource, AgentSourceError, AgentSourceStatus, classify_source, resolve_source_from_environment};
