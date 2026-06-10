// ─── < Imports > ────────────────────────────────────────────────────

use anyhow::Result;

use crate::agent::AgentSource;
use crate::audit::AuditEvent;
use crate::config::Config;
use crate::decision::Decision;
use crate::executor::ExecutionReport;
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
