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
fn decision_reason_has_human_readable_text() {
    assert_eq!(DecisionReason::ActionAllowed.as_text(), "request matches an allowed policy");

    assert_eq!(DecisionReason::ResourceRequired.as_text(), "action requires a resource");

    assert_eq!(DecisionReason::ConsoleCommandBlocked.as_text(), "command is explicitly blocked by console policy");

    assert_eq!(DecisionReason::ConsoleCommandRequiresApproval.as_text(), "command requires manual approval");

    assert_eq!(DecisionReason::ConsoleSubcommandNotAllowed.as_text(), "subcommand is not allowed for this command");

    assert_eq!(DecisionReason::ConsoleArgumentRequiresApproval.as_text(), "argument requires manual approval");
}

#[test]
fn decision_reason_has_stable_codes() {
    assert_eq!(DecisionReason::ActionAllowed.as_code(), "action_allowed");
    assert_eq!(DecisionReason::ConsoleCommandBlocked.as_code(), "console_command_blocked");

    assert_eq!(DecisionReason::ConsoleSubcommandRequiresApproval.as_code(), "console_subcommand_requires_approval");

    assert_eq!(DecisionReason::ConsoleArgumentRequiresApproval.as_code(), "console_argument_requires_approval");
}

#[test]
fn risk_level_has_stable_text() {
    assert_eq!(RiskLevel::Low.as_text(), "low");
    assert_eq!(RiskLevel::Medium.as_text(), "medium");
    assert_eq!(RiskLevel::High.as_text(), "high");
    assert_eq!(RiskLevel::Critical.as_text(), "critical");
}
