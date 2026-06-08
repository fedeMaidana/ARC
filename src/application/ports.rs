// ─── < Imports > ────────────────────────────────────────────────────

use anyhow::{Context, Result};

use crate::agent::{self, AgentSource};
use crate::audit::{self as audit_log, AuditEvent};
use crate::config::Config;
use crate::decision::Decision;
use crate::executor::{self, ExecutionReport};
use crate::request::Request;

// ─── < Traits > ─────────────────────────────────────────────────────

pub trait ReviewEnvironment {
    fn resolve_source(&self, default_source: &str, config: &Config) -> Result<AgentSource>;

    fn prepare_audit_log(&self, config: &Config) -> Result<()>;

    fn decide(&self, request: &Request, config: &Config) -> Decision;

    fn execute(&self, request: &Request, decision: &Decision, config: &Config) -> ExecutionReport;

    fn execute_approved(&self, request: &Request, config: &Config) -> ExecutionReport;

    fn request_approval(&self, prompt: &str) -> Result<bool>;

    fn record_audit_event(&self, config: &Config, event: &AuditEvent) -> Result<()>;
}

// ─── < Structs > ────────────────────────────────────────────────────

#[derive(Debug, Default, Clone, Copy)]
pub struct ArcReviewEnvironment;

// ─── < Implementations > ────────────────────────────────────────────

impl ReviewEnvironment for ArcReviewEnvironment {
    fn resolve_source(&self, default_source: &str, config: &Config) -> Result<AgentSource> {
        agent::resolve_source_from_environment(default_source, &config.agents).context("could not resolve ARC source")
    }

    fn prepare_audit_log(&self, config: &Config) -> Result<()> {
        audit_log::ensure_audit_log_is_writable(&config.audit).context("could not prepare audit log")
    }

    fn decide(&self, request: &Request, config: &Config) -> Decision {
        crate::policy::decide(request, config)
    }

    fn execute(&self, request: &Request, decision: &Decision, config: &Config) -> ExecutionReport {
        executor::execute(request, decision, &config.execution, &config.console)
    }

    fn execute_approved(&self, request: &Request, config: &Config) -> ExecutionReport {
        executor::execute_approved(request, &config.execution, &config.console)
    }

    fn request_approval(&self, prompt: &str) -> Result<bool> {
        let answer = crate::ask::ask_yes_no(prompt).context("could not ask for request approval")?;

        Ok(matches!(answer, crate::ask::AskAnswer::Yes))
    }

    fn record_audit_event(&self, config: &Config, event: &AuditEvent) -> Result<()> {
        audit_log::record_event(&config.audit, event).context("could not write audit log")
    }
}
