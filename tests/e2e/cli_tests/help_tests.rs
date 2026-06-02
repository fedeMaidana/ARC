// ─── < Imports > ────────────────────────────────────────────────────

use super::common::{run_arc, stdout};

// ─── < Tests > ──────────────────────────────────────────────────────

#[test]
fn help_command_prints_usage() {
    let output = run_arc(&["help"]);

    assert_eq!(output.status.code(), Some(2));

    let stdout = stdout(&output);

    assert!(stdout.contains("ARC"));

    assert!(stdout.contains("Setup"));
    assert!(stdout.contains("arc init"));
    assert!(stdout.contains("Create the default Rego policy file"));

    assert!(stdout.contains("Runtime settings"));
    assert!(stdout.contains("arc config path"));
    assert!(stdout.contains("arc config check"));
    assert!(stdout.contains("arc config show"));

    assert!(stdout.contains("Policy"));
    assert!(stdout.contains("arc run <command> [args...]"));
    assert!(stdout.contains("arc check run <command> [args...]"));
    assert!(stdout.contains("arc decide --json"));

    assert!(stdout.contains("Interactive"));
    assert!(stdout.contains("arc monitor"));
    assert!(stdout.contains("arc tui"));

    assert!(stdout.contains("Environment"));
    assert!(stdout.contains("ARC_POLICY_ENGINE=native"));
    assert!(stdout.contains("ARC_POLICY_ENGINE=rego"));
    assert!(stdout.contains("ARC_REGO_POLICY_PATH"));
    assert!(stdout.contains("ARC_AUDIT_ENABLED"));
}

#[test]
fn config_help_command_prints_config_usage() {
    let output = run_arc(&["config", "help"]);

    assert_eq!(output.status.code(), Some(2));

    let stdout = stdout(&output);

    assert!(stdout.contains("Runtime settings usage"));
    assert!(stdout.contains("arc config path"));
    assert!(stdout.contains("arc config check"));
    assert!(stdout.contains("arc config show"));
    assert!(stdout.contains("ARC_* environment variables"));
}

#[test]
fn unknown_config_command_prints_cli_error() {
    let output = run_arc(&["config", "nope"]);

    assert_eq!(output.status.code(), Some(2));

    let stdout = stdout(&output);

    assert!(stdout.contains("CLI error"));
    assert!(stdout.contains("unknown config command 'nope'"));
    assert!(stdout.contains("Setup"));
    assert!(stdout.contains("Runtime settings"));
    assert!(stdout.contains("Policy"));
    assert!(stdout.contains("Interactive"));
}

#[test]
fn check_without_action_prints_cli_error() {
    let output = run_arc(&["check"]);

    assert_eq!(output.status.code(), Some(2));

    let stdout = stdout(&output);

    assert!(stdout.contains("CLI error"));
    assert!(stdout.contains("missing action after 'check'"));
    assert!(stdout.contains("Setup"));
    assert!(stdout.contains("Runtime settings"));
    assert!(stdout.contains("Policy"));
    assert!(stdout.contains("Interactive"));
}
