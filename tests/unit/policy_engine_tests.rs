// ─── < Imports > ────────────────────────────────────────────────────

use arc::decision::{DecisionReason, DecisionStatus};
use arc::policy::{NativePolicyEngine, PolicyEngine, PolicyInput};

use crate::common;

// ─── < Tests > ──────────────────────────────────────────────────────

#[test]
fn native_policy_engine_decides_using_policy_input() {
    let config = common::test_config();
    let request = arc::request::Request::new(arc::request::RequestMode::Check, "run".to_string(), common::strings(&["echo", "hello"]));

    let engine = NativePolicyEngine::new();
    let decision = engine.decide(PolicyInput::new(&request, &config)).into_decision();

    assert_eq!(decision.status, DecisionStatus::Allow);
    assert_eq!(decision.reason, DecisionReason::ActionAllowed);
}

#[test]
fn public_policy_decide_still_uses_native_engine_behavior() {
    let config = common::test_config();
    let request = arc::request::Request::new(arc::request::RequestMode::Check, "run".to_string(), common::strings(&["rm", "--danger"]));

    let decision = arc::policy::decide(&request, &config);

    assert_eq!(decision.status, DecisionStatus::Deny);
    assert_eq!(decision.reason, DecisionReason::ConsoleCommandBlocked);
}
