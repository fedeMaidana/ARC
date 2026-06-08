// ─── < Imports > ────────────────────────────────────────────────────

use anyhow::{Context, Result};

use crate::agent::{self, AgentSource};
use crate::ask::{self, AskAnswer};
use crate::audit::{self as audit_log, AuditEvent};
use crate::config::Config;
use crate::decision::Decision;
use crate::executor::{self, ExecutionReport};
use crate::request::Request;

// ─── < Enums > ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApprovalMode {
    Interactive,
    NonInteractive,
}

// ─── < Structs > ────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ActionReview {
    source: AgentSource,
    decision: Decision,
}

pub struct ActionReviewReport {
    source: AgentSource,
    decision: Decision,
    execution_report: ExecutionReport,
}

// ─── < Public Functions > ───────────────────────────────────────────

pub fn review_action(request: &Request, config: &Config, default_source: &str, approval_mode: ApprovalMode) -> Result<ActionReviewReport> {
    let review = prepare_action_review(request, config, default_source)?;

    complete_action_review(request, config, &review, approval_mode)
}

pub fn prepare_action_review(request: &Request, config: &Config, default_source: &str) -> Result<ActionReview> {
    let source = resolve_source(default_source, config)?;
    prepare_audit_log(config)?;

    let decision = crate::policy::decide(request, config);

    Ok(ActionReview { source, decision })
}

pub fn complete_action_review(
    request: &Request,
    config: &Config,
    review: &ActionReview,
    approval_mode: ApprovalMode,
) -> Result<ActionReviewReport> {
    let execution_report = execute_reviewed_action(request, config, review.decision(), approval_mode)?;

    record_action_review(request, config, review, &execution_report)?;

    Ok(ActionReviewReport {
        source: review.source().clone(),
        decision: *review.decision(),
        execution_report,
    })
}

// ─── < Implementations > ────────────────────────────────────────────

impl ActionReview {
    pub fn source(&self) -> &AgentSource {
        &self.source
    }

    pub fn decision(&self) -> &Decision {
        &self.decision
    }
}

impl ActionReviewReport {
    pub fn source(&self) -> &AgentSource {
        &self.source
    }

    pub fn decision(&self) -> &Decision {
        &self.decision
    }

    pub fn execution_report(&self) -> &ExecutionReport {
        &self.execution_report
    }
}

// ─── < Private Functions > ──────────────────────────────────────────

fn resolve_source(default_source: &str, config: &Config) -> Result<AgentSource> {
    agent::resolve_source_from_environment(default_source, &config.agents).context("could not resolve ARC source")
}

fn prepare_audit_log(config: &Config) -> Result<()> {
    audit_log::ensure_audit_log_is_writable(&config.audit).context("could not prepare audit log")
}

fn execute_reviewed_action(
    request: &Request,
    config: &Config,
    decision: &Decision,
    approval_mode: ApprovalMode,
) -> Result<ExecutionReport> {
    if should_ask_user(request, decision, approval_mode) {
        return ask_and_maybe_execute(request, config);
    }

    Ok(executor::execute(request, decision, &config.execution, &config.console))
}

fn should_ask_user(request: &Request, decision: &Decision, approval_mode: ApprovalMode) -> bool {
    matches!(approval_mode, ApprovalMode::Interactive) && decision.should_ask() && !request.is_check_mode()
}

fn ask_and_maybe_execute(request: &Request, config: &Config) -> Result<ExecutionReport> {
    let prompt = approval_prompt(request);

    let answer = ask::ask_yes_no(&prompt).context("could not ask for request approval")?;

    match answer {
        AskAnswer::Yes => Ok(executor::execute_approved(request, &config.execution, &config.console)),
        AskAnswer::No => Ok(ExecutionReport::AskDeclined),
    }
}

fn approval_prompt(request: &Request) -> String {
    if request.has_resource() {
        return format!("ARC wants to execute `{}`", request.resource);
    }

    format!("ARC wants to perform `{}`", request.action)
}

fn record_action_review(request: &Request, config: &Config, review: &ActionReview, execution_report: &ExecutionReport) -> Result<()> {
    let event = AuditEvent::from_parts(review.source(), request, review.decision(), execution_report);

    audit_log::record_event(&config.audit, &event).context("could not write audit log")
}
