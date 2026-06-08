// ─── < Imports > ────────────────────────────────────────────────────

use anyhow::Result;

use crate::agent::AgentSource;
use crate::audit::AuditEvent;
use crate::config::Config;
use crate::decision::Decision;
use crate::executor::ExecutionReport;
use crate::request::Request;

use super::ports::{ArcReviewEnvironment, ReviewEnvironment};

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
    let environment = ArcReviewEnvironment;

    review_action_with_environment(request, config, default_source, approval_mode, &environment)
}

pub fn review_action_with_environment(
    request: &Request,
    config: &Config,
    default_source: &str,
    approval_mode: ApprovalMode,
    environment: &impl ReviewEnvironment,
) -> Result<ActionReviewReport> {
    let review = prepare_action_review_with_environment(request, config, default_source, environment)?;

    complete_action_review_with_environment(request, config, &review, approval_mode, environment)
}

pub fn prepare_action_review(request: &Request, config: &Config, default_source: &str) -> Result<ActionReview> {
    let environment = ArcReviewEnvironment;

    prepare_action_review_with_environment(request, config, default_source, &environment)
}

pub fn prepare_action_review_with_environment(
    request: &Request,
    config: &Config,
    default_source: &str,
    environment: &impl ReviewEnvironment,
) -> Result<ActionReview> {
    let source = environment.resolve_source(default_source, config)?;

    environment.prepare_audit_log(config)?;

    let decision = environment.decide(request, config);

    Ok(ActionReview { source, decision })
}

pub fn complete_action_review(
    request: &Request,
    config: &Config,
    review: &ActionReview,
    approval_mode: ApprovalMode,
) -> Result<ActionReviewReport> {
    let environment = ArcReviewEnvironment;

    complete_action_review_with_environment(request, config, review, approval_mode, &environment)
}

pub fn complete_action_review_with_environment(
    request: &Request,
    config: &Config,
    review: &ActionReview,
    approval_mode: ApprovalMode,
    environment: &impl ReviewEnvironment,
) -> Result<ActionReviewReport> {
    let execution_report = execute_reviewed_action(request, config, review.decision(), approval_mode, environment)?;

    record_action_review(request, config, review, &execution_report, environment)?;

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

fn execute_reviewed_action(
    request: &Request,
    config: &Config,
    decision: &Decision,
    approval_mode: ApprovalMode,
    environment: &impl ReviewEnvironment,
) -> Result<ExecutionReport> {
    if should_ask_user(request, decision, approval_mode) {
        return ask_and_maybe_execute(request, config, environment);
    }

    Ok(environment.execute(request, decision, config))
}

fn should_ask_user(request: &Request, decision: &Decision, approval_mode: ApprovalMode) -> bool {
    matches!(approval_mode, ApprovalMode::Interactive) && decision.should_ask() && !request.is_check_mode()
}

fn ask_and_maybe_execute(request: &Request, config: &Config, environment: &impl ReviewEnvironment) -> Result<ExecutionReport> {
    let prompt = approval_prompt(request);
    let approved = environment.request_approval(&prompt)?;

    if approved {
        return Ok(environment.execute_approved(request, config));
    }

    Ok(ExecutionReport::AskDeclined)
}

fn approval_prompt(request: &Request) -> String {
    if request.has_resource() {
        return format!("ARC wants to execute `{}`", request.resource);
    }

    format!("ARC wants to perform `{}`", request.action)
}

fn record_action_review(
    request: &Request,
    config: &Config,
    review: &ActionReview,
    execution_report: &ExecutionReport,
    environment: &impl ReviewEnvironment,
) -> Result<()> {
    let event = AuditEvent::from_parts(review.source(), request, review.decision(), execution_report);

    environment.record_audit_event(config, &event)
}
