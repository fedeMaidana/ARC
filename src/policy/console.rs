// ─── < Imports > ────────────────────────────────────────────────────

use crate::decision::{Decision, DecisionReason, RiskLevel};
use crate::request::Request;

use super::risk;
use super::rules::{ConsoleCommandPolicy, ConsoleSubcommandPolicy, PolicyRules};

// ─── < Public Functions > ───────────────────────────────────────────

pub fn decide(request: &Request, rules: &impl PolicyRules) -> Option<Decision> {
    if !request.is_run_command() {
        return None;
    }

    let Some(command_name) = request.command_name() else {
        return Some(Decision::deny_with_risk(DecisionReason::ConsoleCommandRequired, RiskLevel::Medium));
    };

    if rules.is_blocked_command(command_name) {
        return Some(Decision::deny_with_risk(DecisionReason::ConsoleCommandBlocked, RiskLevel::Critical));
    }

    if let Some(decision) = decide_command_rule(command_name, request, rules) {
        return Some(decision);
    }

    let command_policy = rules.command_policy(command_name);

    if command_policy == ConsoleCommandPolicy::Deny {
        return Some(Decision::deny_with_risk(DecisionReason::ConsoleCommandNotAllowed, RiskLevel::High));
    }

    if request
        .command_args()
        .iter()
        .any(|argument| is_blocked_argument(command_name, argument, rules))
    {
        return Some(Decision::deny_with_risk(DecisionReason::ConsoleArgumentBlocked, RiskLevel::Critical));
    }

    if request
        .command_args()
        .iter()
        .any(|argument| rules.command_argument_should_ask(command_name, argument))
    {
        return Some(Decision::ask_with_risk(
            DecisionReason::ConsoleArgumentRequiresApproval,
            risk::for_allowed_console_command(command_name),
        ));
    }

    match command_policy {
        ConsoleCommandPolicy::Allow => None,
        ConsoleCommandPolicy::Ask => {
            Some(Decision::ask_with_risk(DecisionReason::ConsoleCommandRequiresApproval, risk::for_allowed_console_command(command_name)))
        }
        ConsoleCommandPolicy::Deny => Some(Decision::deny_with_risk(DecisionReason::ConsoleCommandNotAllowed, RiskLevel::High)),
    }
}

// ─── < Private Functions > ──────────────────────────────────────────

fn decide_command_rule(command_name: &str, request: &Request, rules: &impl PolicyRules) -> Option<Decision> {
    let subcommand_policy = rules.subcommand_policy(command_name, primary_subcommand(request))?;

    match subcommand_policy {
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

fn is_blocked_argument(command_name: &str, argument: &str, rules: &impl PolicyRules) -> bool {
    rules.is_blocked_console_argument(argument)
        || rules.is_blocked_command_argument(command_name, argument)
        || rules.is_protected_resource(argument)
        || rules.is_blocked_path(argument)
}
