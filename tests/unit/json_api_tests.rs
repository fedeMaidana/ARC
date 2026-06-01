// ─── < Imports > ────────────────────────────────────────────────────

use arc::decision::{Decision, DecisionReason, RiskLevel};
use arc::executor::ExecutionReport;
use arc::json_api;
use arc::request::RequestMode;

// ─── < Tests: Input > ───────────────────────────────────────────────

#[test]
fn parses_run_request_from_json_command_array() {
    let input = r#"{"action":"run","command":["echo","hola"]}"#;

    let request = json_api::request_from_json(input).expect("request should parse");

    assert!(request.is_check_mode());
    assert_eq!(request.action, "run");
    assert_eq!(request.command_name(), Some("echo"));
    assert_eq!(request.command_args(), &["hola".to_string()]);
    assert_eq!(request.resource, "echo hola");
}

#[test]
fn parses_resource_request_from_json_resource() {
    let input = r#"{"action":"http_get","resource":"https://example.com"}"#;

    let request = json_api::request_from_json(input).expect("request should parse");

    assert!(matches!(request.mode, RequestMode::Check));
    assert_eq!(request.action, "http_get");
    assert_eq!(request.resource, "https://example.com");
}

#[test]
fn rejects_run_without_command_array() {
    let input = r#"{"action":"run"}"#;

    let error = json_api::request_from_json(input).expect_err("request should fail");

    assert_eq!(error.code(), "missing_command");
    assert_eq!(error.to_string(), "run action requires a command array");
}

#[test]
fn rejects_command_for_non_run_action() {
    let input = r#"{"action":"http_get","command":["echo","hola"]}"#;

    let error = json_api::request_from_json(input).expect_err("request should fail");

    assert_eq!(error.code(), "command_only_allowed_for_run");
    assert_eq!(error.to_string(), "command can only be used with run action");
}

// ─── < Tests: Output > ──────────────────────────────────────────────

#[test]
fn builds_json_decision_response() {
    let request = json_api::request_from_json(r#"{"action":"run","command":["echo","hola"]}"#).expect("request should parse");

    let decision = Decision::ask_with_risk(DecisionReason::ConsoleCommandRequiresApproval, RiskLevel::Low);

    let execution_report = ExecutionReport::CheckMode { allowed: true };

    let response = json_api::decision_response_from_parts(&request, &decision, &execution_report);

    assert!(response.ok);
    assert_eq!(response.api_version, "1");
    assert_eq!(response.kind, "decision");
    assert_eq!(response.request.mode, "check");
    assert_eq!(response.request.action, "run");
    assert_eq!(response.request.resource, Some("echo hola".to_string()));
    assert_eq!(response.decision.status, "ask");
    assert_eq!(response.decision.reason, "command requires manual approval");
    assert_eq!(response.decision.reason_code, "console_command_requires_approval");
    assert_eq!(response.decision.risk, "low");
    assert_eq!(response.execution.kind, "check_mode");
    assert!(response.execution.allowed);
    assert!(!response.execution.executed);
    assert_eq!(response.execution.exit_code, 0);
}

#[test]
fn builds_json_error_response_with_default_error_code() {
    let response = json_api::error_response(&"boom");

    assert!(!response.ok);
    assert_eq!(response.api_version, "1");
    assert_eq!(response.kind, "error");
    assert_eq!(response.error_code, "application_error");
    assert_eq!(response.error, "boom");
}

#[test]
fn builds_json_error_response_with_custom_error_code() {
    let response = json_api::error_response_with_code("missing_command", &"run action requires a command array");

    assert!(!response.ok);
    assert_eq!(response.api_version, "1");
    assert_eq!(response.kind, "error");
    assert_eq!(response.error_code, "missing_command");
    assert_eq!(response.error, "run action requires a command array");
}
