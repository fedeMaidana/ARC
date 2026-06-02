// ─── < Imports > ────────────────────────────────────────────────────

use super::common::{TestFixture, assert_success, json_stdout};

// ─── < Tests > ──────────────────────────────────────────────────────

#[test]
fn decide_json_allowed_command_returns_machine_readable_response() {
    let fixture = TestFixture::new("decide-json-allowed-command");

    let output = fixture.run_with_stdin(&["decide", "--json"], r#"{"action":"run","command":["echo","hello"]}"#);

    assert_success(&output);

    let response = json_stdout(&output);

    assert_eq!(response["ok"], true);
    assert_eq!(response["api_version"], "1");
    assert_eq!(response["kind"], "decision");

    assert_eq!(response["request"]["mode"], "check");
    assert_eq!(response["request"]["action"], "run");
    assert_eq!(response["request"]["resource"], "echo hello");

    assert_eq!(response["decision"]["status"], "allow");
    assert_eq!(response["decision"]["reason"], "request matches an allowed policy");
    assert_eq!(response["decision"]["reason_code"], "action_allowed");

    assert_eq!(response["execution"]["kind"], "check_mode");
    assert_eq!(response["execution"]["allowed"], true);
    assert_eq!(response["execution"]["executed"], false);
    assert_eq!(response["execution"]["exit_code"], 0);
}

#[test]
fn decide_json_blocked_command_returns_non_zero_machine_readable_response() {
    let fixture = TestFixture::new("decide-json-blocked-command");

    let output = fixture.run_with_stdin(&["decide", "--json"], r#"{"action":"run","command":["rm","-rf","/"]}"#);

    assert_eq!(output.status.code(), Some(1));

    let response = json_stdout(&output);

    assert_eq!(response["ok"], true);
    assert_eq!(response["api_version"], "1");
    assert_eq!(response["kind"], "decision");

    assert_eq!(response["decision"]["status"], "deny");
    assert_eq!(response["decision"]["reason"], "command is explicitly blocked by console policy");
    assert_eq!(response["decision"]["reason_code"], "console_command_blocked");

    assert_eq!(response["execution"]["kind"], "check_mode");
    assert_eq!(response["execution"]["allowed"], false);
    assert_eq!(response["execution"]["executed"], false);
    assert_eq!(response["execution"]["exit_code"], 1);
}

#[test]
fn decide_json_invalid_request_returns_json_error() {
    let fixture = TestFixture::new("decide-json-invalid-request");

    let output = fixture.run_with_stdin(&["decide", "--json"], r#"{"action":"run"}"#);

    assert_eq!(output.status.code(), Some(2));

    let response = json_stdout(&output);

    assert_eq!(response["ok"], false);
    assert_eq!(response["api_version"], "1");
    assert_eq!(response["kind"], "error");
    assert_eq!(response["error_code"], "missing_command");
    assert_eq!(response["error"], "run action requires a command array");
}
