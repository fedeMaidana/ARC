// ─── < Imports > ────────────────────────────────────────────────────

use std::time::{SystemTime, UNIX_EPOCH};

use serde::Serialize;

use crate::decision::Decision;
use crate::executor::ExecutionReport;
use crate::request::Request;

use super::sanitizer::sanitize_field;

// ─── < Constants > ──────────────────────────────────────────────────

pub const AUDIT_SCHEMA_VERSION: &str = "1";

// ─── < Structs > ────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct AuditEvent {
    pub audit_schema_version: String,
    pub timestamp_unix_seconds: u64,
    pub source: String,
    pub mode: String,
    pub action: String,
    pub resource: Option<String>,
    pub decision: String,
    pub reason: String,
    pub reason_code: String,
    pub risk: String,
    pub executed: bool,
    pub exit_code: i32,
    pub execution: AuditExecution,
}

// ─── < Enums > ──────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AuditExecution {
    CheckMode {
        allowed: bool,
    },
    SkippedDenied,
    AskRequired,
    AskDeclined,
    NoExecutionNeeded,
    MissingCommand,
    CommandFinished {
        command_line: String,
        status: String,
        success: bool,
        stdout_truncated: bool,
        stderr_truncated: bool,
    },
    CommandTimedOut {
        command_line: String,
        timeout_seconds: u64,
        stdout_truncated: bool,
        stderr_truncated: bool,
    },
    CommandFailed {
        command_line: String,
        details: String,
    },
}

// ─── < Implementations > ────────────────────────────────────────────

impl AuditEvent {
    pub fn from_parts(source: impl Into<String>, request: &Request, decision: &Decision, execution_report: &ExecutionReport) -> Self {
        Self {
            audit_schema_version: AUDIT_SCHEMA_VERSION.to_string(),
            timestamp_unix_seconds: current_timestamp_unix_seconds(),
            source: sanitize_field(&source.into()),
            mode: request_mode_text(request).to_string(),
            action: sanitize_field(&request.action),
            resource: audit_resource(request),
            decision: decision.status.as_text().to_string(),
            reason: decision.reason.as_text().to_string(),
            reason_code: decision.reason.as_code().to_string(),
            risk: decision.risk.as_text().to_string(),
            executed: was_executed(execution_report),
            exit_code: execution_report.exit_code(),
            execution: AuditExecution::from_execution_report(execution_report),
        }
    }
}

impl AuditExecution {
    fn from_execution_report(execution_report: &ExecutionReport) -> Self {
        match execution_report {
            ExecutionReport::CheckMode { allowed } => Self::CheckMode { allowed: *allowed },
            ExecutionReport::SkippedDenied => Self::SkippedDenied,
            ExecutionReport::AskRequired => Self::AskRequired,
            ExecutionReport::AskDeclined => Self::AskDeclined,
            ExecutionReport::NoExecutionNeeded => Self::NoExecutionNeeded,
            ExecutionReport::MissingCommand => Self::MissingCommand,
            ExecutionReport::CommandFinished(report) => Self::CommandFinished {
                command_line: sanitize_field(&report.command_line),
                status: sanitize_field(&report.status),
                success: report.success,
                stdout_truncated: report.stdout_truncated,
                stderr_truncated: report.stderr_truncated,
            },
            ExecutionReport::CommandTimedOut(report) => Self::CommandTimedOut {
                command_line: sanitize_field(&report.command_line),
                timeout_seconds: report.timeout_seconds,
                stdout_truncated: report.stdout_truncated,
                stderr_truncated: report.stderr_truncated,
            },
            ExecutionReport::CommandFailed(error) => Self::CommandFailed {
                command_line: sanitize_field(&error.command_line),
                details: sanitize_field(&error.details),
            },
        }
    }
}

// ─── < Private Functions > ──────────────────────────────────────────

fn current_timestamp_unix_seconds() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs()
}

fn request_mode_text(request: &Request) -> &'static str {
    if request.is_check_mode() { "check" } else { "execute" }
}

fn audit_resource(request: &Request) -> Option<String> {
    if request.has_resource() {
        Some(sanitize_field(&request.resource))
    } else {
        None
    }
}

fn was_executed(execution_report: &ExecutionReport) -> bool {
    matches!(execution_report, ExecutionReport::CommandFinished(_))
}
