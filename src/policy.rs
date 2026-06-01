// ─── < Imports > ────────────────────────────────────────────────────

use crate::config::Config;
use crate::decision::{Decision, DecisionReason, RiskLevel};
use crate::request::Request;

// ─── < Public Functions > ───────────────────────────────────────────

pub fn decide(request: &Request, config: &Config) -> Decision {
    if let Some(decision) = decide_action_rules(request, config) {
        return decision;
    }

    if let Some(decision) = decide_console_rules(request, config) {
        return decision;
    }

    if let Some(decision) = decide_resource_rules(request, config) {
        return decision;
    }

    if let Some(decision) = decide_http_rules(request, config) {
        return decision;
    }

    if config.actions.is_allowed_action(&request.action) {
        if config.actions.action_should_ask(&request.action) {
            return Decision::ask_with_risk(DecisionReason::ActionRequiresApproval, risk_for_allowed_request(request));
        }

        return Decision::allow_with_risk(DecisionReason::ActionAllowed, risk_for_allowed_request(request));
    }

    Decision::deny_with_risk(DecisionReason::ActionNotConfigured, RiskLevel::High)
}

// ─── < Private Functions > ──────────────────────────────────────────

fn decide_action_rules(request: &Request, config: &Config) -> Option<Decision> {
    if config.actions.is_blocked_action(&request.action) {
        return Some(Decision::deny_with_risk(DecisionReason::ActionBlocked, RiskLevel::Critical));
    }

    if config.actions.action_needs_resource(&request.action) && !request.has_resource() {
        return Some(Decision::deny_with_risk(DecisionReason::ResourceRequired, RiskLevel::Medium));
    }

    None
}

fn decide_console_rules(request: &Request, config: &Config) -> Option<Decision> {
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

    for argument in request.command_args() {
        if is_blocked_console_argument(argument, config) {
            return Some(Decision::deny_with_risk(DecisionReason::ConsoleArgumentBlocked, RiskLevel::Critical));
        }
    }

    if config.console.command_should_ask(command_name) {
        return Some(Decision::ask_with_risk(
            DecisionReason::ConsoleCommandRequiresApproval,
            risk_for_allowed_console_command(command_name),
        ));
    }

    None
}

fn decide_resource_rules(request: &Request, config: &Config) -> Option<Decision> {
    if !request.has_resource() {
        return None;
    }

    if config.resources.is_protected_resource(&request.resource) {
        return Some(Decision::deny_with_risk(DecisionReason::ResourceProtected, RiskLevel::Critical));
    }

    if config.resources.is_blocked_path(&request.resource) {
        return Some(Decision::deny_with_risk(DecisionReason::PathBlocked, RiskLevel::Critical));
    }

    None
}

fn decide_http_rules(request: &Request, config: &Config) -> Option<Decision> {
    if !request.is_http_get() {
        return None;
    }

    if !is_valid_http_url(&request.resource) {
        return Some(Decision::deny_with_risk(DecisionReason::InvalidHttpUrl, RiskLevel::Medium));
    }

    if config.http.is_blocked_target(&request.resource) {
        return Some(Decision::deny_with_risk(DecisionReason::HttpTargetBlocked, RiskLevel::High));
    }

    None
}

fn is_valid_http_url(resource: &str) -> bool {
    resource.starts_with("https://") || resource.starts_with("http://")
}

fn is_blocked_console_argument(argument: &str, config: &Config) -> bool {
    if config.console.is_blocked_argument(argument) {
        return true;
    }

    if config.resources.is_protected_resource(argument) {
        return true;
    }

    if config.resources.is_blocked_path(argument) {
        return true;
    }

    false
}

fn risk_for_allowed_request(request: &Request) -> RiskLevel {
    if request.is_http_get() {
        return RiskLevel::Medium;
    }

    RiskLevel::Low
}

fn risk_for_allowed_console_command(command_name: &str) -> RiskLevel {
    match command_name {
        "echo" | "pwd" | "ls" => RiskLevel::Low,
        "cat" => RiskLevel::Medium,
        _ => RiskLevel::Medium,
    }
}
