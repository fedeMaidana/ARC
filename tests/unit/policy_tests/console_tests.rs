// ─── < Imports > ────────────────────────────────────────────────────

use arc::decision::{DecisionReason, DecisionStatus, RiskLevel};
use arc::policy;

use super::common::{assert_decision, default_config, request};

// ─── < Tests > ──────────────────────────────────────────────────────

#[test]
fn allows_configured_console_command() {
    let config = default_config();
    let request = request("run", &["ls", "-la"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Allow, DecisionReason::ActionAllowed, RiskLevel::Low);
}

#[test]
fn denies_run_without_command_when_action_does_not_require_resource_first() {
    let mut config = default_config();

    config.actions.need_resource.clear();

    let request = request("run", &[]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::ConsoleCommandRequired, RiskLevel::Medium);
}

#[test]
fn denies_blocked_console_command() {
    let config = default_config();
    let request = request("run", &["rm", "-rf", "/"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::ConsoleCommandBlocked, RiskLevel::Critical);
}

#[test]
fn denies_console_command_not_in_allowlist() {
    let config = default_config();
    let request = request("run", &["python", "script.py"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::ConsoleCommandNotAllowed, RiskLevel::High);
}

#[test]
fn allows_unlisted_console_command_when_default_command_policy_is_allow() {
    let mut config = default_config();

    config.console.default_command_policy = "allow".to_string();

    let request = request("run", &["python", "script.py"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Allow, DecisionReason::ActionAllowed, RiskLevel::Low);
}

#[test]
fn asks_for_unlisted_console_command_when_default_command_policy_is_ask() {
    let mut config = default_config();

    config.console.default_command_policy = "ask".to_string();

    let request = request("run", &["python", "script.py"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Ask, DecisionReason::ConsoleCommandRequiresApproval, RiskLevel::Medium);
}

#[test]
fn explicit_allowed_console_command_overrides_default_command_policy_ask() {
    let mut config = default_config();

    config.console.default_command_policy = "ask".to_string();

    let request = request("run", &["echo", "hello"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Allow, DecisionReason::ActionAllowed, RiskLevel::Low);
}

#[test]
fn explicit_blocked_console_command_overrides_default_command_policy_allow() {
    let mut config = default_config();

    config.console.default_command_policy = "allow".to_string();

    let request = request("run", &["rm", "-rf", "/"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::ConsoleCommandBlocked, RiskLevel::Critical);
}

#[test]
fn blocked_argument_overrides_default_command_policy_allow() {
    let mut config = default_config();

    config.console.default_command_policy = "allow".to_string();

    let request = request("run", &["python", "--danger"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::ConsoleArgumentBlocked, RiskLevel::Critical);
}

#[test]
fn default_action_deny_overrides_default_command_policy_ask_for_unconfigured_run_action() {
    let mut config = default_config();

    config.actions.allowed.retain(|action| action != "run");
    config.console.default_command_policy = "ask".to_string();

    let request = request("run", &["python", "script.py"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::ActionNotConfigured, RiskLevel::High);
}

#[test]
fn asks_for_console_command_approval() {
    let mut config = default_config();

    config.console.ask_commands.push("cat".to_string());

    let request = request("run", &["cat", "README.md"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Ask, DecisionReason::ConsoleCommandRequiresApproval, RiskLevel::Medium);
}

#[test]
fn denies_console_argument_that_is_explicitly_blocked() {
    let config = default_config();
    let request = request("run", &["cat", "/"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::ConsoleArgumentBlocked, RiskLevel::Critical);
}

#[test]
fn denies_console_argument_that_targets_protected_resource() {
    let config = default_config();
    let request = request("run", &["cat", ".env"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::ConsoleArgumentBlocked, RiskLevel::Critical);
}

#[test]
fn denies_console_argument_that_targets_protected_resource_after_normalization() {
    let config = default_config();
    let request = request("run", &["cat", "config/../.env"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::ConsoleArgumentBlocked, RiskLevel::Critical);
}

#[test]
fn denies_console_argument_that_targets_blocked_path_after_normalization() {
    let config = default_config();
    let request = request("run", &["cat", "/tmp/../etc/passwd"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::ConsoleArgumentBlocked, RiskLevel::Critical);
}
