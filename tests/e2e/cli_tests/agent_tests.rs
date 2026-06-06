// ─── < Imports > ────────────────────────────────────────────────────

use std::fs;

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
fn agents_sync_persists_detected_known_agents_in_registry() {
    let mut fixture = TestFixture::new("agents-sync").without_system_path();

    let command_path = fixture.create_path_command("opencode");
    let output = fixture.run(&["agents", "sync"]);

    assert_success(&output);

    let stdout = stdout(&output);

    assert!(stdout.contains("Agent registry synced"));
    assert!(stdout.contains("detected"));
    assert!(stdout.contains("registered"));
    assert!(stdout.contains("added"));

    let registry_content = fs::read_to_string(fixture.registry_path()).expect("registry file should exist");

    assert!(registry_content.contains("\"id\": \"opencode\""));
    assert!(registry_content.contains("\"display_name\": \"OpenCode\""));
    assert!(registry_content.contains("\"command\": \"opencode\""));
    assert!(registry_content.contains(&command_path.display().to_string()));
}

#[test]
fn agents_sync_ignores_arc_launcher_directory_when_discovering_agents() {
    let mut fixture = TestFixture::new("agents-sync-ignores-arc-launchers").without_system_path();

    let real_command_path = fixture.create_path_command("opencode");
    let launcher_path = fixture.launcher_dir().join("opencode");

    fs::create_dir_all(fixture.launcher_dir()).expect("launcher dir should be created");
    fs::write(&launcher_path, "#!/usr/bin/env sh\nexit 99\n").expect("fake launcher should be written");
    make_test_executable(&launcher_path);

    let real_bin_dir = real_command_path.parent().expect("real command should have parent directory");

    fixture.set_env("PATH", format!("{}:{}", fixture.launcher_dir().display(), real_bin_dir.display()));

    let output = fixture.run(&["agents", "sync"]);

    assert_success(&output);

    let registry_content = fs::read_to_string(fixture.registry_path()).expect("registry file should exist");

    assert!(registry_content.contains("\"id\": \"opencode\""));
    assert!(registry_content.contains(&real_command_path.display().to_string()));
    assert!(!registry_content.contains(&format!("\"path\": \"{}\"", launcher_path.display())));
}

#[test]
fn init_syncs_detected_agents_into_registry() {
    let mut fixture = TestFixture::new("init-syncs-agents").without_system_path();

    let command_path = fixture.create_path_command("opencode");
    let output = fixture.run(&["init"]);

    assert_success(&output);

    let stdout = stdout(&output);

    assert!(stdout.contains("ARC initialized"));
    assert!(stdout.contains("Policy"));
    assert!(stdout.contains("Agents"));
    assert!(stdout.contains("Agent registry synced"));

    let registry_content = fs::read_to_string(fixture.registry_path()).expect("registry file should exist");

    assert!(registry_content.contains("\"id\": \"opencode\""));
    assert!(registry_content.contains("\"display_name\": \"OpenCode\""));
    assert!(registry_content.contains("\"command\": \"opencode\""));
    assert!(registry_content.contains(&command_path.display().to_string()));
}

#[test]
fn agents_list_loads_sources_from_internal_registry() {
    let mut fixture = TestFixture::new("agents-list-registry").without_system_path();

    fixture.create_path_command("opencode");

    let sync_output = fixture.run(&["agents", "sync"]);
    assert_success(&sync_output);

    let list_output = fixture.run(&["agents", "list"]);
    assert_success(&list_output);

    let stdout = stdout(&list_output);

    assert!(stdout.contains("opencode"));
    assert!(stdout.contains("OpenCode"));
    assert!(stdout.contains("Detected command: opencode"));
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

#[cfg(unix)]
fn make_test_executable(path: &std::path::Path) {
    use std::os::unix::fs::PermissionsExt;

    let mut permissions = fs::metadata(path)
        .expect("test executable metadata should be readable")
        .permissions();

    permissions.set_mode(0o755);

    fs::set_permissions(path, permissions).expect("test executable permissions should be updated");
}

#[cfg(not(unix))]
fn make_test_executable(_path: &std::path::Path) {}
