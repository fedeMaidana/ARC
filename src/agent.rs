// ─── < Modules > ────────────────────────────────────────────────────

mod source;

// ─── < Public Exports > ─────────────────────────────────────────────

pub use self::source::{AgentSource, AgentSourceError, AgentSourceStatus, classify_source, resolve_source_from_environment};
