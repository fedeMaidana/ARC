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
fn agents_scan_detects_known_agent_from_path_without_printing_missing_known_agents() {
    let mut fixture = TestFixture::new("agents-scan-detects-opencode").without_system_path();

    let command_path = fixture.create_path_command("opencode");
    let output = fixture.run(&["agents", "scan"]);

    assert_success(&output);

    let stdout = stdout(&output);

    assert!(stdout.contains("Agent scan"));
    assert!(stdout.contains("Detected agents"));
    assert!(stdout.contains("opencode"));
    assert!(stdout.contains("OpenCode"));
    assert!(stdout.contains("command: opencode"));
    assert!(stdout.contains(&command_path.display().to_string()));

    assert!(!stdout.contains("codex"));
    assert!(!stdout.contains("status: not found"));
}

#[test]
fn agents_scan_known_prints_missing_known_agents() {
    let mut fixture = TestFixture::new("agents-scan-known").without_system_path();

    let command_path = fixture.create_path_command("opencode");
    let output = fixture.run(&["agents", "scan", "--known"]);

    assert_success(&output);

    let stdout = stdout(&output);

    assert!(stdout.contains("opencode"));
    assert!(stdout.contains("OpenCode"));
    assert!(stdout.contains(&command_path.display().to_string()));

    assert!(stdout.contains("Known agents not found"));
    assert!(stdout.contains("claude-code"));
    assert!(stdout.contains("Claude Code"));
    assert!(stdout.contains("codex"));
    assert!(stdout.contains("Codex"));
    assert!(stdout.contains("status: not found"));
}

#[test]
fn agents_scan_detects_possible_agent_candidates_from_path() {
    let mut fixture = TestFixture::new("agents-scan-candidates").without_system_path();

    let command_path = fixture.create_path_command("my-agent");
    let output = fixture.run(&["agents", "scan"]);

    assert_success(&output);

    let stdout = stdout(&output);

    assert!(stdout.contains("Possible agents"));
    assert!(stdout.contains("my-agent"));
    assert!(stdout.contains(&command_path.display().to_string()));
    assert!(stdout.contains("command name contains \"agent\""));
}

#[test]
fn agents_scan_detects_ai_token_candidates_without_matching_random_words() {
    let mut fixture = TestFixture::new("agents-scan-ai-token").without_system_path();

    let command_path = fixture.create_path_command("local-ai");
    fixture.create_path_command("tail");

    let output = fixture.run(&["agents", "scan"]);

    assert_success(&output);

    let stdout = stdout(&output);

    assert!(stdout.contains("local-ai"));
    assert!(stdout.contains(&command_path.display().to_string()));
    assert!(stdout.contains("command name has token \"ai\""));
    assert!(!stdout.contains("tail"));
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
