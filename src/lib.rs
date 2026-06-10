// ─── < Layers > ─────────────────────────────────────────────────────

pub(crate) mod application;
pub(crate) mod domain;
pub(crate) mod infrastructure;
pub(crate) mod interface;

pub mod app;

// ─── < Stable Module Paths: pure domain > ───────────────────────────

pub use domain::target::{http_target, resource};
pub use domain::{decision, request};

// ─── < Stable Module Paths: single-layer modules > ──────────────────

pub use infrastructure::config;

pub(crate) use infrastructure::{doctor, shims};

pub use interface::{cli, json_api};

pub(crate) use interface::{ask, output, tui, ui};

// ─── < Stable Module Paths: cross-layer facades > ───────────────────

pub mod policy {
    pub use crate::domain::policy::{ConsoleCommandPolicy, ConsoleSubcommandPolicy, DefaultPolicyAction, PolicyRules};
    pub use crate::infrastructure::policy_engine::{
        NativePolicyEngine, PolicyDecision, PolicyEngine, PolicyInput, RegoPolicyEngine, decide,
    };
}

pub mod audit {
    pub use crate::domain::audit::{AUDIT_SCHEMA_VERSION, AuditEvent, AuditExecution};
    pub use crate::infrastructure::audit_sink::{AuditError, ensure_audit_log_is_writable, record_event};
}

pub mod agent {
    pub use crate::domain::agent::{AgentSource, AgentSourceStatus};
    pub use crate::infrastructure::agents::{
        AgentCandidate, AgentDiscovery, AgentRegistration, AgentRegistry, AgentRegistrySyncReport, AgentScan, AgentSourceError,
        MissingKnownAgent, classify_source, load_agent_registry, resolve_source_from_environment, save_agent_registry,
        scan_installed_agents, scan_known_agents, sync_agent_registry,
    };
}

pub mod executor {
    pub use crate::domain::execution::{CommandExecutionError, CommandExecutionReport, CommandTimeoutReport, ExecutionReport};
    pub use crate::infrastructure::executor::{execute, execute_approved};
}
