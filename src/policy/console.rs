// ─── < Imports > ────────────────────────────────────────────────────

use crate::config::{Config, ConsoleSubcommandPolicy};
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

    if let Some(decision) = decide_command_rule(command_name, request, config) {
        return Some(decision);
    }

    if request
        .command_args()
        .iter()
        .any(|argument| is_blocked_argument(command_name, argument, config))
    {
        return Some(Decision::deny_with_risk(DecisionReason::ConsoleArgumentBlocked, RiskLevel::Critical));
    }

    if request
        .command_args()
        .iter()
        .any(|argument| config.console.command_argument_should_ask(command_name, argument))
    {
        return Some(Decision::ask_with_risk(
            DecisionReason::ConsoleArgumentRequiresApproval,
            risk::for_allowed_console_command(command_name),
        ));
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

fn decide_command_rule(command_name: &str, request: &Request, config: &Config) -> Option<Decision> {
    let command_rule = config.console.command_rule(command_name)?;

    match command_rule.subcommand_policy(primary_subcommand(request)) {
        ConsoleSubcommandPolicy::Allowed => None,
        ConsoleSubcommandPolicy::Ask => Some(Decision::ask_with_risk(
            DecisionReason::ConsoleSubcommandRequiresApproval,
            risk::for_allowed_console_command(command_name),
        )),
        ConsoleSubcommandPolicy::Blocked => Some(Decision::deny_with_risk(DecisionReason::ConsoleSubcommandBlocked, RiskLevel::Critical)),
        ConsoleSubcommandPolicy::NotAllowed => Some(Decision::deny_with_risk(DecisionReason::ConsoleSubcommandNotAllowed, RiskLevel::High)),
        ConsoleSubcommandPolicy::Required => Some(Decision::deny_with_risk(DecisionReason::ConsoleSubcommandRequired, RiskLevel::Medium)),
    }
}

fn primary_subcommand(request: &Request) -> Option<&str> {
    let subcommand = request.command_args().first()?.as_str();

    if subcommand.starts_with('-') {
        return None;
    }

    Some(subcommand)
}

fn is_blocked_argument(command_name: &str, argument: &str, config: &Config) -> bool {
    config.console.is_blocked_argument(argument)
        || config.console.is_blocked_command_argument(command_name, argument)
        || config.resources.is_protected_resource(argument)
        || config.resources.is_blocked_path(argument)
}
