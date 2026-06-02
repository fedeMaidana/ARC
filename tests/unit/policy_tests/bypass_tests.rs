// ─── < Imports > ────────────────────────────────────────────────────

use arc::decision::{DecisionReason, DecisionStatus, RiskLevel};
use arc::policy;

use super::common::{assert_decision, default_config, request};

// ─── < Tests > ──────────────────────────────────────────────────────

#[test]
fn denies_absolute_command_path_as_unlisted_command() {
    let config = default_config();
    let request = request("run", &["/usr/bin/git", "status"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::ConsoleCommandNotAllowed, RiskLevel::High);
}

#[test]
fn denies_relative_command_path_as_unlisted_command() {
    let config = default_config();
    let request = request("run", &["./git", "status"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::ConsoleCommandNotAllowed, RiskLevel::High);
}

#[test]
fn denies_git_option_before_required_subcommand() {
    let config = default_config();
    let request = request("run", &["git", "-c", "alias.push=!rm -rf /", "push"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::ConsoleSubcommandRequired, RiskLevel::Medium);
}

#[test]
fn denies_chained_git_subcommand_text_as_unlisted_subcommand() {
    let config = default_config();
    let request = request("run", &["git", "status;push"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::ConsoleSubcommandNotAllowed, RiskLevel::High);
}

#[test]
fn denied_git_push_overrides_default_command_policy_allow() {
    let mut config = default_config();

    config.console.default_command_policy = "allow".to_string();

    let request = request("run", &["git", "push"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::ConsoleSubcommandBlocked, RiskLevel::Critical);
}

#[test]
fn denied_cargo_publish_overrides_default_command_policy_allow() {
    let mut config = default_config();

    config.console.default_command_policy = "allow".to_string();

    let request = request("run", &["cargo", "publish"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::ConsoleSubcommandBlocked, RiskLevel::Critical);
}

#[test]
fn denies_allowed_command_attempting_to_read_protected_resource_with_parent_traversal() {
    let config = default_config();
    let request = request("run", &["cat", "safe/../../.env"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::ConsoleArgumentBlocked, RiskLevel::Critical);
}

#[test]
fn denies_allowed_command_attempting_to_read_blocked_path_with_parent_traversal() {
    let config = default_config();
    let request = request("run", &["cat", "/tmp/../root/.ssh/id_rsa"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::ConsoleArgumentBlocked, RiskLevel::Critical);
}
