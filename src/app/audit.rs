// ─── < Imports > ────────────────────────────────────────────────────

use anyhow::{Context, Result};

use crate::agent::{self, AgentSource};
use crate::audit as audit_log;
use crate::audit::AuditEvent;
use crate::config::{AgentsConfig, AuditConfig};
use crate::decision::Decision;
use crate::executor::ExecutionReport;
use crate::request::Request;

// ─── < Public Functions > ───────────────────────────────────────────

pub fn prepare(config: &AuditConfig) -> Result<()> {
    audit_log::ensure_audit_log_is_writable(config).context("could not prepare audit log")
}

pub fn resolve_source(default_source: &str, agents_config: &AgentsConfig) -> Result<AgentSource> {
    agent::resolve_source_from_environment(default_source, agents_config).context("could not resolve ARC source")
}

pub fn record(
    source: &AgentSource,
    audit_config: &AuditConfig,
    request: &Request,
    decision: &Decision,
    execution_report: &ExecutionReport,
) -> Result<()> {
    let event = AuditEvent::from_parts(source, request, decision, execution_report);

    audit_log::record_event(audit_config, &event).context("could not write audit log")
}
