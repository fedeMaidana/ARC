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
    assert!(stdout.contains("Create the default Rego policy and sync detected agents"));

    assert!(stdout.contains("Runtime settings"));
    assert!(stdout.contains("arc settings path"));
    assert!(stdout.contains("arc settings check"));
    assert!(stdout.contains("arc settings show"));
    assert!(stdout.contains("arc config ... is kept as a compatibility alias"));

    assert!(stdout.contains("Agents"));
    assert!(stdout.contains("arc agents scan"));
    assert!(stdout.contains("arc agents scan --known"));
    assert!(stdout.contains("arc agents sync"));
    assert!(stdout.contains("arc agents list"));
    assert!(stdout.contains("arc agents env <id>"));

    assert!(stdout.contains("Shims"));
    assert!(stdout.contains("arc shims path"));
    assert!(stdout.contains("arc shims install"));
    assert!(stdout.contains("arc shims list"));

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
    assert!(stdout.contains("ARC_AGENT_REGISTRY_PATH"));
    assert!(stdout.contains("ARC_LAUNCHER_DIR"));
    assert!(stdout.contains("ARC_RUNTIME_SHIMS_DIR"));
    assert!(stdout.contains("ARC_AGENT_SOURCES"));
    assert!(stdout.contains("ARC_SOURCE"));
    assert!(stdout.contains("ARC_AUDIT_ENABLED"));
}

#[test]
fn settings_help_command_prints_runtime_settings_usage() {
    let output = run_arc(&["settings", "help"]);

    assert_eq!(output.status.code(), Some(2));

    let stdout = stdout(&output);

    assert!(stdout.contains("Runtime settings usage"));
    assert!(stdout.contains("arc settings path"));
    assert!(stdout.contains("arc settings check"));
    assert!(stdout.contains("arc settings show"));
    assert!(stdout.contains("arc config path"));
    assert!(stdout.contains("ARC_* environment variables"));
}

#[test]
fn config_help_command_prints_runtime_settings_usage_as_compatibility_alias() {
    let output = run_arc(&["config", "help"]);

    assert_eq!(output.status.code(), Some(2));

    let stdout = stdout(&output);

    assert!(stdout.contains("Runtime settings usage"));
    assert!(stdout.contains("arc settings path"));
    assert!(stdout.contains("arc settings check"));
    assert!(stdout.contains("arc settings show"));
    assert!(stdout.contains("arc config path"));
    assert!(stdout.contains("ARC_* environment variables"));
}

#[test]
fn unknown_settings_command_prints_cli_error() {
    let output = run_arc(&["settings", "nope"]);

    assert_eq!(output.status.code(), Some(2));

    let stdout = stdout(&output);

    assert!(stdout.contains("CLI error"));
    assert!(stdout.contains("unknown runtime settings command 'nope'"));
    assert!(stdout.contains("Setup"));
    assert!(stdout.contains("Runtime settings"));
    assert!(stdout.contains("Agents"));
    assert!(stdout.contains("Shims"));
    assert!(stdout.contains("Policy"));
    assert!(stdout.contains("Interactive"));
}

#[test]
fn unknown_config_command_prints_cli_error_for_compatibility_alias() {
    let output = run_arc(&["config", "nope"]);

    assert_eq!(output.status.code(), Some(2));

    let stdout = stdout(&output);

    assert!(stdout.contains("CLI error"));
    assert!(stdout.contains("unknown runtime settings command 'nope'"));
    assert!(stdout.contains("Setup"));
    assert!(stdout.contains("Runtime settings"));
    assert!(stdout.contains("Agents"));
    assert!(stdout.contains("Shims"));
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
    assert!(stdout.contains("Agents"));
    assert!(stdout.contains("Shims"));
    assert!(stdout.contains("Policy"));
    assert!(stdout.contains("Interactive"));
}
