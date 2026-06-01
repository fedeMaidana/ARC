// ─── < Imports > ────────────────────────────────────────────────────

use std::env;

use anyhow::{Context, Result};

use crate::audit as audit_log;
use crate::audit::AuditEvent;
use crate::config::AuditConfig;
use crate::decision::Decision;
use crate::executor::ExecutionReport;
use crate::request::Request;

// ─── < Public Functions > ───────────────────────────────────────────

pub fn prepare(config: &AuditConfig) -> Result<()> {
    audit_log::ensure_audit_log_is_writable(config).context("could not prepare audit log")
}

pub fn record(default_source: &str, audit_config: &AuditConfig, request: &Request, decision: &Decision, execution_report: &ExecutionReport) -> Result<()> {
    let source = source_or(default_source);
    let event = AuditEvent::from_parts(source, request, decision, execution_report);

    audit_log::record_event(audit_config, &event).context("could not write audit log")
}

// ─── < Private Functions > ──────────────────────────────────────────

fn source_or(default_source: &str) -> String {
    env::var("ARC_SOURCE")
        .ok()
        .map(|source| source.trim().to_string())
        .filter(|source| !source.is_empty())
        .unwrap_or_else(|| default_source.to_string())
}
