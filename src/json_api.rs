// ─── < Imports > ────────────────────────────────────────────────────

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::decision::Decision;
use crate::executor::ExecutionReport;
use crate::request::{Request, RequestMode};

// ─── < Errors > ─────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum JsonApiError {
    #[error("invalid JSON request")]
    InvalidJson {
        #[source]
        source: serde_json::Error,
    },

    #[error("action is required")]
    MissingAction,

    #[error("run action requires a command array")]
    MissingCommand,

    #[error("command array cannot be empty")]
    EmptyCommand,

    #[error("run action cannot use resource; use command instead")]
    ResourceNotAllowedForRun,

    #[error("command can only be used with run action")]
    CommandOnlyAllowedForRun,
}

// ─── < Input Structs > ──────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct JsonRequestInput {
    pub action: String,

    #[serde(default)]
    pub resource: Option<String>,

    #[serde(default)]
    pub command: Option<Vec<String>>,
}

// ─── < Output Structs > ─────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct JsonDecisionResponse {
    pub ok: bool,
    pub request: JsonRequestOutput,
    pub decision: JsonDecisionOutput,
    pub execution: JsonExecutionOutput,
}

#[derive(Debug, Serialize)]
pub struct JsonErrorResponse {
    pub ok: bool,
    pub error: String,
}

#[derive(Debug, Serialize)]
pub struct JsonRequestOutput {
    pub mode: String,
    pub action: String,
    pub resource: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct JsonDecisionOutput {
    pub status: String,
    pub reason: String,
    pub risk: String,
}

#[derive(Debug, Serialize)]
pub struct JsonExecutionOutput {
    pub kind: String,
    pub allowed: bool,
    pub executed: bool,
    pub exit_code: i32,
}

// ─── < Public Functions > ───────────────────────────────────────────

pub fn request_from_json(input: &str) -> Result<Request, JsonApiError> {
    let input: JsonRequestInput = serde_json::from_str(input).map_err(|source| JsonApiError::InvalidJson { source })?;

    input.into_request()
}

pub fn decision_response_from_parts(request: &Request, decision: &Decision, execution_report: &ExecutionReport) -> JsonDecisionResponse {
    JsonDecisionResponse {
        ok: true,
        request: JsonRequestOutput {
            mode: request_mode_text(request).to_string(),
            action: request.action.clone(),
            resource: request_resource(request),
        },
        decision: JsonDecisionOutput {
            status: decision.status.as_text().to_string(),
            reason: decision.reason.as_text().to_string(),
            risk: decision.risk.as_text().to_string(),
        },
        execution: JsonExecutionOutput {
            kind: execution_kind(execution_report).to_string(),
            allowed: execution_allowed(execution_report),
            executed: execution_was_run(execution_report),
            exit_code: execution_report.exit_code(),
        },
    }
}

pub fn error_response(error: &impl std::fmt::Display) -> JsonErrorResponse {
    JsonErrorResponse {
        ok: false,
        error: error.to_string(),
    }
}

// ─── < Implementations > ────────────────────────────────────────────

impl JsonRequestInput {
    fn into_request(self) -> Result<Request, JsonApiError> {
        if self.action.trim().is_empty() {
            return Err(JsonApiError::MissingAction);
        }

        if self.action == "run" {
            return self.into_run_request();
        }

        self.into_resource_request()
    }

    fn into_run_request(self) -> Result<Request, JsonApiError> {
        if self.resource.is_some() {
            return Err(JsonApiError::ResourceNotAllowedForRun);
        }

        let Some(command_parts) = self.command else {
            return Err(JsonApiError::MissingCommand);
        };

        if command_parts.is_empty() {
            return Err(JsonApiError::EmptyCommand);
        }

        Ok(Request::new(RequestMode::Check, self.action, command_parts))
    }

    fn into_resource_request(self) -> Result<Request, JsonApiError> {
        if self.command.is_some() {
            return Err(JsonApiError::CommandOnlyAllowedForRun);
        }

        let command_parts = match self.resource {
            Some(resource) => vec![resource],
            None => Vec::new(),
        };

        Ok(Request::new(RequestMode::Check, self.action, command_parts))
    }
}

// ─── < Private Functions > ──────────────────────────────────────────

fn request_mode_text(request: &Request) -> &'static str {
    if request.is_check_mode() { "check" } else { "execute" }
}

fn request_resource(request: &Request) -> Option<String> {
    if request.has_resource() {
        Some(request.resource.clone())
    } else {
        None
    }
}

fn execution_kind(execution_report: &ExecutionReport) -> &'static str {
    match execution_report {
        ExecutionReport::CheckMode { .. } => "check_mode",
        ExecutionReport::SkippedDenied => "skipped_denied",
        ExecutionReport::AskRequired => "ask_required",
        ExecutionReport::AskDeclined => "ask_declined",
        ExecutionReport::NoExecutionNeeded => "no_execution_needed",
        ExecutionReport::MissingCommand => "missing_command",
        ExecutionReport::CommandFinished(_) => "command_finished",
        ExecutionReport::CommandTimedOut(_) => "command_timed_out",
        ExecutionReport::CommandFailed(_) => "command_failed",
    }
}

fn execution_allowed(execution_report: &ExecutionReport) -> bool {
    match execution_report {
        ExecutionReport::CheckMode { allowed } => *allowed,
        ExecutionReport::CommandFinished(report) => report.success,
        ExecutionReport::NoExecutionNeeded => true,
        _ => false,
    }
}

fn execution_was_run(execution_report: &ExecutionReport) -> bool {
    matches!(execution_report, ExecutionReport::CommandFinished(_))
}
