// ─── < Imports > ────────────────────────────────────────────────────

use crate::config::ExecutionConfig;
use crate::decision::Decision;
use crate::request::Request;

use super::console;
use super::model::ExecutionReport;

// ─── < Public Functions > ───────────────────────────────────────────

pub fn execute(request: &Request, decision: &Decision, execution_config: &ExecutionConfig) -> ExecutionReport {
    if request.is_check_mode() {
        return ExecutionReport::CheckMode {
            allowed: !decision.is_denied(),
        };
    }

    if decision.is_denied() {
        return ExecutionReport::SkippedDenied;
    }

    if decision.should_ask() {
        return ExecutionReport::AskRequired;
    }

    execute_allowed_request(request, execution_config)
}

pub fn execute_approved(request: &Request, execution_config: &ExecutionConfig) -> ExecutionReport {
    if request.is_check_mode() {
        return ExecutionReport::CheckMode { allowed: true };
    }

    execute_allowed_request(request, execution_config)
}

// ─── < Private Functions > ──────────────────────────────────────────

fn execute_allowed_request(request: &Request, execution_config: &ExecutionConfig) -> ExecutionReport {
    if request.is_run_command() {
        return console::run(request, execution_config);
    }

    ExecutionReport::NoExecutionNeeded
}
