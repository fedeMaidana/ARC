// ─── < Imports > ────────────────────────────────────────────────────

use arc::decision::{DecisionReason, DecisionStatus, RiskLevel};
use arc::policy;

use super::common::{assert_decision, default_config, request};

// ─── < Tests > ──────────────────────────────────────────────────────

#[test]
fn denies_protected_resource() {
    let config = default_config();
    let request = request("read_file", &[".env"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::ResourceProtected, RiskLevel::Critical);
}

#[test]
fn denies_protected_resource_nested_in_folder() {
    let config = default_config();
    let request = request("read_file", &["config/.env"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::ResourceProtected, RiskLevel::Critical);
}

#[test]
fn denies_protected_resource_with_current_dir_segment() {
    let config = default_config();
    let request = request("read_file", &["./.env"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::ResourceProtected, RiskLevel::Critical);
}

#[test]
fn denies_protected_resource_after_parent_segment_normalization() {
    let config = default_config();
    let request = request("read_file", &["config/../.env"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::ResourceProtected, RiskLevel::Critical);
}

#[test]
fn denies_blocked_path_prefix() {
    let config = default_config();
    let request = request("read_file", &["/etc/passwd"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::PathBlocked, RiskLevel::Critical);
}

#[test]
fn denies_blocked_path_after_parent_segment_normalization() {
    let config = default_config();
    let request = request("read_file", &["/tmp/../etc/passwd"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::PathBlocked, RiskLevel::Critical);
}

#[test]
fn denies_parent_directory_traversal_after_normalization() {
    let config = default_config();
    let request = request("read_file", &["config/../../secrets.txt"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::ResourceProtected, RiskLevel::Critical);
}
