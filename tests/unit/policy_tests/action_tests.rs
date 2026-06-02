// ─── < Imports > ────────────────────────────────────────────────────

use arc::decision::{DecisionReason, DecisionStatus, RiskLevel};
use arc::policy;

use super::common::{assert_decision, default_config, request};

// ─── < Tests > ──────────────────────────────────────────────────────

#[test]
fn allows_configured_action_without_resource() {
    let config = default_config();
    let request = request("list_files", &[]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Allow, DecisionReason::ActionAllowed, RiskLevel::Low);
}

#[test]
fn denies_blocked_action() {
    let config = default_config();
    let request = request("delete_file", &["README.md"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::ActionBlocked, RiskLevel::Critical);
}

#[test]
fn denies_action_that_requires_missing_resource() {
    let config = default_config();
    let request = request("read_file", &[]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::ResourceRequired, RiskLevel::Medium);
}

#[test]
fn asks_for_configured_action_approval() {
    let mut config = default_config();

    config.actions.allowed.push("publish".to_string());
    config.actions.ask.push("publish".to_string());

    let request = request("publish", &["release"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Ask, DecisionReason::ActionRequiresApproval, RiskLevel::Low);
}

#[test]
fn denies_unconfigured_action() {
    let config = default_config();
    let request = request("teleport", &[]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::ActionNotConfigured, RiskLevel::High);
}

#[test]
fn allows_unconfigured_action_when_default_action_is_allow() {
    let mut config = default_config();

    config.policy.default_action = "allow".to_string();

    let request = request("teleport", &[]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Allow, DecisionReason::ActionAllowedByDefault, RiskLevel::Low);
}

#[test]
fn asks_for_unconfigured_action_when_default_action_is_ask() {
    let mut config = default_config();

    config.policy.default_action = "ask".to_string();

    let request = request("teleport", &[]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Ask, DecisionReason::ActionRequiresApprovalByDefault, RiskLevel::Low);
}

#[test]
fn explicit_blocked_action_overrides_default_action_allow() {
    let mut config = default_config();

    config.policy.default_action = "allow".to_string();

    let request = request("delete_file", &["README.md"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::ActionBlocked, RiskLevel::Critical);
}
