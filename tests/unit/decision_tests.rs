// ─── < Imports > ────────────────────────────────────────────────────

use arc::decision::{Decision, DecisionReason, DecisionStatus, RiskLevel};

// ─── < Tests > ──────────────────────────────────────────────────────

#[test]
fn allow_decision_has_allow_status() {
    let decision = Decision::allow(DecisionReason::ActionAllowed);

    assert_eq!(decision.status, DecisionStatus::Allow);
    assert_eq!(decision.reason, DecisionReason::ActionAllowed);
    assert_eq!(decision.risk, RiskLevel::Low);
    assert!(decision.is_allowed());
    assert!(!decision.should_ask());
}

#[test]
fn deny_decision_has_deny_status() {
    let decision = Decision::deny(DecisionReason::ActionBlocked);

    assert_eq!(decision.status, DecisionStatus::Deny);
    assert_eq!(decision.reason, DecisionReason::ActionBlocked);
    assert_eq!(decision.risk, RiskLevel::High);
    assert!(!decision.is_allowed());
    assert!(decision.is_denied());
}

#[test]
fn ask_decision_has_ask_status() {
    let decision = Decision::ask(DecisionReason::ConsoleCommandRequiresApproval);

    assert_eq!(decision.status, DecisionStatus::Ask);
    assert_eq!(decision.reason, DecisionReason::ConsoleCommandRequiresApproval);
    assert_eq!(decision.risk, RiskLevel::Medium);
    assert!(!decision.is_allowed());
    assert!(decision.should_ask());
}

#[test]
fn decision_can_override_risk_level() {
    let decision = Decision::deny_with_risk(DecisionReason::ConsoleCommandBlocked, RiskLevel::Critical);

    assert_eq!(decision.status, DecisionStatus::Deny);
    assert_eq!(decision.reason, DecisionReason::ConsoleCommandBlocked);
    assert_eq!(decision.risk, RiskLevel::Critical);
}

#[test]
fn decision_reason_has_stable_text() {
    assert_eq!(DecisionReason::ConsoleCommandBlocked.as_text(), "console command is blocked");

    assert_eq!(DecisionReason::ResourceRequired.as_text(), "resource is required");

    assert_eq!(DecisionReason::ConsoleCommandRequiresApproval.as_text(), "console command requires user approval");

    assert_eq!(DecisionReason::ConsoleSubcommandNotAllowed.as_text(), "console subcommand is not allowed");

    assert_eq!(DecisionReason::ConsoleArgumentRequiresApproval.as_text(), "console argument requires user approval");
}

#[test]
fn risk_level_has_stable_text() {
    assert_eq!(RiskLevel::Low.as_text(), "low");
    assert_eq!(RiskLevel::Medium.as_text(), "medium");
    assert_eq!(RiskLevel::High.as_text(), "high");
    assert_eq!(RiskLevel::Critical.as_text(), "critical");
}
