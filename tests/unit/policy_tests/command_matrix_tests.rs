// ─── < Imports > ────────────────────────────────────────────────────

use arc::decision::{DecisionReason, DecisionStatus, RiskLevel};
use arc::policy;

use super::common::{assert_decision, default_config, request};

// ─── < Tests > ──────────────────────────────────────────────────────

#[test]
fn allows_configured_command_subcommand() {
    let config = default_config();
    let request = request("run", &["git", "status"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Allow, DecisionReason::ActionAllowed, RiskLevel::Low);
}

#[test]
fn asks_for_configured_command_subcommand() {
    let config = default_config();
    let request = request("run", &["git", "commit"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Ask, DecisionReason::ConsoleSubcommandRequiresApproval, RiskLevel::Medium);
}

#[test]
fn denies_blocked_command_subcommand() {
    let config = default_config();
    let request = request("run", &["git", "push"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::ConsoleSubcommandBlocked, RiskLevel::Critical);
}

#[test]
fn denies_unlisted_command_subcommand_when_allowlist_exists() {
    let config = default_config();
    let request = request("run", &["git", "rebase"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::ConsoleSubcommandNotAllowed, RiskLevel::High);
}

#[test]
fn denies_missing_command_subcommand_when_allowlist_exists() {
    let config = default_config();
    let request = request("run", &["git"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::ConsoleSubcommandRequired, RiskLevel::Medium);
}

#[test]
fn denies_command_specific_blocked_argument() {
    let config = default_config();
    let request = request("run", &["git", "status", "--upload-pack"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::ConsoleArgumentBlocked, RiskLevel::Critical);
}

#[test]
fn asks_for_command_specific_argument() {
    let config = default_config();
    let request = request("run", &["cargo", "check", "--release"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Ask, DecisionReason::ConsoleArgumentRequiresApproval, RiskLevel::Medium);
}
