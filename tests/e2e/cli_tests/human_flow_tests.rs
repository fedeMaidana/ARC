// ─── < Imports > ────────────────────────────────────────────────────

use super::common::{TestFixture, assert_success, stdout};

// ─── < Tests > ──────────────────────────────────────────────────────

#[test]
fn check_allowed_command_returns_success_without_running_command() {
    let fixture = TestFixture::new("check-allowed-command");
    let output = fixture.run(&["check", "run", "echo", "hello"]);

    assert_success(&output);

    let stdout = stdout(&output);

    assert!(stdout.contains("Decision"));
    assert!(stdout.contains("allow"));
    assert!(stdout.contains("Check mode"));
    assert!(stdout.contains("Execution skipped"));
}

#[test]
fn run_allowed_command_executes_and_prints_output() {
    let fixture = TestFixture::new("run-allowed-command");
    let output = fixture.run(&["run", "echo", "hello"]);

    assert_success(&output);

    let stdout = stdout(&output);

    assert!(stdout.contains("Decision"));
    assert!(stdout.contains("allow"));
    assert!(stdout.contains("Execution"));
    assert!(stdout.contains("Output"));
    assert!(stdout.contains("hello"));
}

#[test]
fn check_blocked_command_returns_non_zero_exit_code() {
    let fixture = TestFixture::new("check-blocked-command");
    let output = fixture.run(&["check", "run", "rm", "-rf", "/"]);

    assert_eq!(output.status.code(), Some(1));

    let stdout = stdout(&output);

    assert!(stdout.contains("Decision"));
    assert!(stdout.contains("deny"));
    assert!(stdout.contains("command is explicitly blocked by console policy"));
    assert!(stdout.contains("Check mode"));
}

#[test]
fn check_blocked_command_subcommand_returns_non_zero_exit_code() {
    let fixture = TestFixture::new("check-blocked-subcommand");
    let output = fixture.run(&["check", "run", "git", "push"]);

    assert_eq!(output.status.code(), Some(1));

    let stdout = stdout(&output);

    assert!(stdout.contains("Decision"));
    assert!(stdout.contains("deny"));
    assert!(stdout.contains("subcommand is explicitly blocked by command policy"));
}

#[test]
fn check_ask_command_subcommand_returns_success_but_marks_ask() {
    let fixture = TestFixture::new("check-ask-subcommand");
    let output = fixture.run(&["check", "run", "git", "commit"]);

    assert_success(&output);

    let stdout = stdout(&output);

    assert!(stdout.contains("Decision"));
    assert!(stdout.contains("ask"));
    assert!(stdout.contains("subcommand requires manual approval"));
    assert!(stdout.contains("Check mode"));
}
