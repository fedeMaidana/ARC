// ─── < Imports > ────────────────────────────────────────────────────

use super::common::{TestFixture, assert_success, run_arc, stdout};

// ─── < Tests > ──────────────────────────────────────────────────────

#[test]
fn agents_list_prints_builtin_sources() {
    let output = run_arc(&["agents", "list"]);

    assert_success(&output);

    let stdout = stdout(&output);

    assert!(stdout.contains("Agents"));
    assert!(stdout.contains("allow unknown sources"));
    assert!(stdout.contains("cli"));
    assert!(stdout.contains("ARC CLI"));
    assert!(stdout.contains("json_api"));
    assert!(stdout.contains("ARC JSON API"));
    assert!(!stdout.contains("opencode"));
}

#[test]
fn agents_list_prints_sources_from_environment() {
    let fixture = TestFixture::with_env(
        "agents-list-custom-source",
        "ARC_AGENT_SOURCES",
        "claude-code|Claude Code|local_agent|true|Claude Code routed through ARC",
    );

    let output = fixture.run(&["agents", "list"]);

    assert_success(&output);

    let stdout = stdout(&output);

    assert!(stdout.contains("claude-code"));
    assert!(stdout.contains("Claude Code"));
    assert!(stdout.contains("local_agent"));
    assert!(stdout.contains("enabled"));
    assert!(stdout.contains("Claude Code routed through ARC"));
}

#[test]
fn agents_env_prints_shell_exports() {
    let output = run_arc(&[
        "agents",
        "env",
        "claude-code",
        "--name",
        "Claude Code",
        "--description",
        "Claude Code routed through ARC",
    ]);

    assert_success(&output);

    let stdout = stdout(&output);

    assert!(stdout.contains("Agent environment"));
    assert!(stdout.contains("export ARC_AGENT_SOURCES='claude-code|Claude Code|local_agent|true|Claude Code routed through ARC'"));
    assert!(stdout.contains("export ARC_SOURCE='claude-code'"));
}

#[test]
fn agents_env_generates_display_name_from_id_when_name_is_missing() {
    let output = run_arc(&["agents", "env", "personal-agent"]);

    assert_success(&output);

    let stdout = stdout(&output);

    assert!(stdout.contains("export ARC_AGENT_SOURCES='personal-agent|Personal Agent|local_agent|true'"));
    assert!(stdout.contains("export ARC_SOURCE='personal-agent'"));
}

#[test]
fn agents_env_rejects_reserved_source_ids() {
    let output = run_arc(&["agents", "env", "cli"]);

    assert_eq!(output.status.code(), Some(2));

    let stdout = stdout(&output);

    assert!(stdout.contains("CLI error"));
    assert!(stdout.contains("agent source id 'cli' is reserved by ARC"));
}

#[test]
fn agents_env_rejects_invalid_source_ids() {
    let output = run_arc(&["agents", "env", "Claude Code"]);

    assert_eq!(output.status.code(), Some(2));

    let stdout = stdout(&output);

    assert!(stdout.contains("CLI error"));
    assert!(stdout.contains("invalid agent source id 'Claude Code'"));
}

#[test]
fn agents_env_rejects_invalid_kind() {
    let output = run_arc(&["agents", "env", "claude-code", "--kind", "remote_magic"]);

    assert_eq!(output.status.code(), Some(2));

    let stdout = stdout(&output);

    assert!(stdout.contains("CLI error"));
    assert!(stdout.contains("invalid agent kind 'remote_magic'"));
}
