// ─── < Imports > ────────────────────────────────────────────────────

use crate::config::Config;
use crate::decision::{Decision, DecisionReason, RiskLevel};
use crate::request::Request;

use super::risk;

// ─── < Public Functions > ───────────────────────────────────────────

pub fn decide(request: &Request, config: &Config) -> Option<Decision> {
    if !request.is_run_command() {
        return None;
    }

    let Some(command_name) = request.command_name() else {
        return Some(Decision::deny_with_risk(DecisionReason::ConsoleCommandRequired, RiskLevel::Medium));
    };

    if config.console.is_blocked_command(command_name) {
        return Some(Decision::deny_with_risk(DecisionReason::ConsoleCommandBlocked, RiskLevel::Critical));
    }

    if !config.console.is_allowed_command(command_name) {
        return Some(Decision::deny_with_risk(DecisionReason::ConsoleCommandNotAllowed, RiskLevel::High));
    }

    if request.command_args().iter().any(|argument| is_blocked_argument(argument, config)) {
        return Some(Decision::deny_with_risk(DecisionReason::ConsoleArgumentBlocked, RiskLevel::Critical));
    }

    if config.console.command_should_ask(command_name) {
        return Some(Decision::ask_with_risk(
            DecisionReason::ConsoleCommandRequiresApproval,
            risk::for_allowed_console_command(command_name),
        ));
    }

    None
}

// ─── < Private Functions > ──────────────────────────────────────────

fn is_blocked_argument(argument: &str, config: &Config) -> bool {
    config.console.is_blocked_argument(argument)
        || config.resources.is_protected_resource(argument)
        || config.resources.is_blocked_path(argument)
}
