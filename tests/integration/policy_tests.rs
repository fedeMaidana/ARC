// ─── < Imports > ────────────────────────────────────────────────────

use arc::decision::{DecisionReason, RiskLevel};
use arc::policy;
use arc::request::{Request, RequestMode};

use crate::common::test_config;

// ─── < Tests > ──────────────────────────────────────────────────────

#[test]
fn allows_allowed_console_command() {
    let config = test_config();

    let request = Request::new(RequestMode::Execute, "run".to_string(), vec!["echo".to_string(), "hello".to_string()]);

    let decision = policy::decide(&request, &config);

    assert!(decision.is_allowed());
    assert_eq!(decision.reason, DecisionReason::ActionAllowed);
    assert_eq!(decision.risk, RiskLevel::Low);
}

#[test]
fn denies_blocked_console_command() {
    let config = test_config();

    let request = Request::new(RequestMode::Execute, "run".to_string(), vec!["rm".to_string(), "-rf".to_string(), "/".to_string()]);

    let decision = policy::decide(&request, &config);

    assert!(!decision.is_allowed());
    assert_eq!(decision.reason, DecisionReason::ConsoleCommandBlocked);
    assert_eq!(decision.risk, RiskLevel::Critical);
}

#[test]
fn denies_console_command_that_is_not_allowed() {
    let config = test_config();

    let request = Request::new(RequestMode::Execute, "run".to_string(), vec!["cat".to_string(), "Cargo.toml".to_string()]);

    let decision = policy::decide(&request, &config);

    assert!(!decision.is_allowed());
    assert_eq!(decision.reason, DecisionReason::ConsoleCommandNotAllowed);
    assert_eq!(decision.risk, RiskLevel::High);
}

#[test]
fn denies_blocked_console_argument() {
    let config = test_config();

    let request = Request::new(RequestMode::Execute, "run".to_string(), vec!["echo".to_string(), "--danger".to_string()]);

    let decision = policy::decide(&request, &config);

    assert!(!decision.is_allowed());
    assert_eq!(decision.reason, DecisionReason::ConsoleArgumentBlocked);
    assert_eq!(decision.risk, RiskLevel::Critical);
}

#[test]
fn denies_missing_required_resource() {
    let config = test_config();

    let request = Request::new(RequestMode::Execute, "http_get".to_string(), Vec::new());

    let decision = policy::decide(&request, &config);

    assert!(!decision.is_allowed());
    assert_eq!(decision.reason, DecisionReason::ResourceRequired);
    assert_eq!(decision.risk, RiskLevel::Medium);
}

#[test]
fn denies_invalid_http_url() {
    let config = test_config();

    let request = Request::new(RequestMode::Execute, "http_get".to_string(), vec!["ftp://example.com".to_string()]);

    let decision = policy::decide(&request, &config);

    assert!(!decision.is_allowed());
    assert_eq!(decision.reason, DecisionReason::InvalidHttpUrl);
    assert_eq!(decision.risk, RiskLevel::Medium);
}

#[test]
fn denies_blocked_http_target() {
    let config = test_config();

    let request = Request::new(RequestMode::Execute, "http_get".to_string(), vec!["http://localhost:3000/status".to_string()]);

    let decision = policy::decide(&request, &config);

    assert!(!decision.is_allowed());
    assert_eq!(decision.reason, DecisionReason::HttpTargetBlocked);
    assert_eq!(decision.risk, RiskLevel::High);
}

#[test]
fn asks_for_configured_action() {
    let mut config = test_config();
    config.actions.ask = crate::common::strings(&["http_get"]);

    let request = Request::new(RequestMode::Execute, "http_get".to_string(), vec!["https://example.com".to_string()]);

    let decision = policy::decide(&request, &config);

    assert!(decision.should_ask());
    assert_eq!(decision.reason, DecisionReason::ActionRequiresApproval);
    assert_eq!(decision.risk, RiskLevel::Medium);
}

#[test]
fn asks_for_configured_console_command() {
    let mut config = test_config();
    config.console.ask_commands = crate::common::strings(&["echo"]);

    let request = Request::new(RequestMode::Execute, "run".to_string(), vec!["echo".to_string(), "hello".to_string()]);

    let decision = policy::decide(&request, &config);

    assert!(decision.should_ask());
    assert_eq!(decision.reason, DecisionReason::ConsoleCommandRequiresApproval);
    assert_eq!(decision.risk, RiskLevel::Low);
}
