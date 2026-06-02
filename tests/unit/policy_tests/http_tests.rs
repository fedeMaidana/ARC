// ─── < Imports > ────────────────────────────────────────────────────

use arc::decision::{DecisionReason, DecisionStatus, RiskLevel};
use arc::policy;

use super::common::{assert_decision, default_config, request};

// ─── < Tests > ──────────────────────────────────────────────────────

#[test]
fn denies_invalid_http_url() {
    let config = default_config();
    let request = request("http_get", &["example.com"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::InvalidHttpUrl, RiskLevel::Medium);
}

#[test]
fn denies_unsupported_http_scheme() {
    let config = default_config();
    let request = request("http_get", &["file:///etc/passwd"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::InvalidHttpUrl, RiskLevel::Medium);
}

#[test]
fn denies_blocked_http_target() {
    let config = default_config();
    let request = request("http_get", &["http://localhost:3000"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::HttpTargetBlocked, RiskLevel::High);
}

#[test]
fn denies_blocked_ipv6_loopback_http_target() {
    let config = default_config();
    let request = request("http_get", &["http://[::1]:3000"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::HttpTargetBlocked, RiskLevel::High);
}

#[test]
fn denies_blocked_unspecified_ipv4_http_target() {
    let config = default_config();
    let request = request("http_get", &["http://0.0.0.0:3000"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::HttpTargetBlocked, RiskLevel::High);
}

#[test]
fn denies_private_ipv4_http_target() {
    let config = default_config();
    let request = request("http_get", &["http://192.168.1.10/status"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::HttpTargetBlocked, RiskLevel::High);
}

#[test]
fn denies_private_ipv6_unique_local_http_target() {
    let config = default_config();
    let request = request("http_get", &["http://[fc00::1]/status"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::HttpTargetBlocked, RiskLevel::High);
}

#[test]
fn denies_link_local_metadata_http_target() {
    let config = default_config();
    let request = request("http_get", &["http://169.254.169.254/latest/meta-data"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::HttpTargetBlocked, RiskLevel::High);
}

#[test]
fn denies_localhost_http_target_with_trailing_dot() {
    let config = default_config();
    let request = request("http_get", &["http://localhost.:3000/status"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::HttpTargetBlocked, RiskLevel::High);
}

#[test]
fn allows_domain_that_only_starts_with_blocked_host_text() {
    let config = default_config();
    let request = request("http_get", &["http://localhost.evil.com"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Allow, DecisionReason::ActionAllowed, RiskLevel::Medium);
}

#[test]
fn allows_external_http_url_with_medium_risk() {
    let config = default_config();
    let request = request("http_get", &["https://example.com"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Allow, DecisionReason::ActionAllowed, RiskLevel::Medium);
}
