// ─── < Imports > ────────────────────────────────────────────────────

use arc::decision::{DecisionReason, DecisionStatus, RiskLevel};
use arc::policy;
use arc::request::{Request, RequestMode};

use crate::common;

// ─── < Tests > ──────────────────────────────────────────────────────

#[test]
fn rego_policy_engine_fails_closed_when_policy_cannot_be_evaluated() {
    let mut config = common::test_config();

    config.policy.engine = "rego".to_string();
    config.policy.rego.policy_path = "/definitely/missing/arc/policies.d".to_string();
    config.policy.rego.entrypoint = "data.arc.decision".to_string();
    config.policy.rego.timeout_seconds = 1;

    let request = Request::new(RequestMode::Check, "run".to_string(), common::strings(&["echo", "hello"]));

    let decision = policy::decide(&request, &config);

    assert_eq!(decision.status, DecisionStatus::Deny);
    assert_eq!(decision.reason, DecisionReason::PolicyEngineFailed);
    assert_eq!(decision.risk, RiskLevel::Critical);
}

#[test]
fn rego_policy_engine_fails_closed_when_entrypoint_is_missing() {
    let mut config = common::test_config();

    config.policy.engine = "rego".to_string();
    config.policy.rego.policy_path = "/definitely/missing/arc/policies.d".to_string();
    config.policy.rego.entrypoint = "data.arc.missing_decision".to_string();
    config.policy.rego.timeout_seconds = 1;

    let request = Request::new(RequestMode::Check, "run".to_string(), common::strings(&["git", "status"]));

    let decision = policy::decide(&request, &config);

    assert_eq!(decision.status, DecisionStatus::Deny);
    assert_eq!(decision.reason, DecisionReason::PolicyEngineFailed);
    assert_eq!(decision.risk, RiskLevel::Critical);
}

#[test]
fn native_policy_engine_still_works_when_rego_config_exists() {
    let mut config = common::test_config();

    config.policy.engine = "native".to_string();
    config.policy.rego.policy_path = "/definitely/missing/arc/policies.d".to_string();
    config.policy.rego.entrypoint = "data.arc.decision".to_string();
    config.policy.rego.timeout_seconds = 1;

    let request = Request::new(RequestMode::Check, "run".to_string(), common::strings(&["echo", "hello"]));

    let decision = policy::decide(&request, &config);

    assert_eq!(decision.status, DecisionStatus::Allow);
    assert_eq!(decision.reason, DecisionReason::ActionAllowed);
    assert_eq!(decision.risk, RiskLevel::Low);
}
