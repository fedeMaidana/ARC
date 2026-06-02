// ─── < Imports > ────────────────────────────────────────────────────

use super::common::{TestFixture, assert_success, stdout};

// ─── < Tests > ──────────────────────────────────────────────────────

#[test]
fn settings_show_prints_runtime_settings() {
    let fixture = TestFixture::new("settings-show");
    let output = fixture.run(&["settings", "show"]);

    assert_success(&output);

    let stdout = stdout(&output);

    assert!(stdout.contains("Runtime settings"));
    assert!(stdout.contains("source:"));
    assert!(stdout.contains("runtime defaults + ARC_* environment"));
    assert!(stdout.contains("Policy"));
    assert!(stdout.contains("Agents"));
    assert!(stdout.contains("Actions"));
    assert!(stdout.contains("Console"));
    assert!(stdout.contains("Execution"));
    assert!(stdout.contains("inherit environment"));
    assert!(stdout.contains("command rules"));
    assert!(stdout.contains("git"));
}

#[test]
fn config_show_still_works_as_compatibility_alias() {
    let fixture = TestFixture::new("config-show");
    let output = fixture.run(&["config", "show"]);

    assert_success(&output);

    let stdout = stdout(&output);

    assert!(stdout.contains("Runtime settings"));
    assert!(stdout.contains("runtime defaults + ARC_* environment"));
}

#[test]
fn settings_path_prints_runtime_settings_source() {
    let fixture = TestFixture::new("settings-path");
    let output = fixture.run(&["settings", "path"]);

    assert_success(&output);

    let stdout = stdout(&output);

    assert!(stdout.contains("Runtime settings source"));
    assert!(stdout.contains("runtime defaults + ARC_* environment"));
}

#[test]
fn config_path_still_works_as_compatibility_alias() {
    let fixture = TestFixture::new("config-path");
    let output = fixture.run(&["config", "path"]);

    assert_success(&output);

    let stdout = stdout(&output);

    assert!(stdout.contains("Runtime settings source"));
    assert!(stdout.contains("runtime defaults + ARC_* environment"));
}

#[test]
fn settings_check_prints_success_for_runtime_settings() {
    let fixture = TestFixture::new("settings-check-valid");
    let output = fixture.run(&["settings", "check"]);

    assert_success(&output);

    let stdout = stdout(&output);

    assert!(stdout.contains("Runtime settings are valid"));
    assert!(stdout.contains("runtime defaults + ARC_* environment"));
}

#[test]
fn config_check_still_works_as_compatibility_alias() {
    let fixture = TestFixture::new("config-check-valid");
    let output = fixture.run(&["config", "check"]);

    assert_success(&output);

    let stdout = stdout(&output);

    assert!(stdout.contains("Runtime settings are valid"));
    assert!(stdout.contains("runtime defaults + ARC_* environment"));
}

#[test]
fn settings_check_prints_validation_errors_for_invalid_runtime_settings() {
    let fixture = TestFixture::with_env("settings-check-invalid", "ARC_POLICY_ENGINE", "magic");
    let output = fixture.run(&["settings", "check"]);

    assert_eq!(output.status.code(), Some(2));

    let stdout = stdout(&output);

    assert!(stdout.contains("Runtime settings error"));
    assert!(stdout.contains("policy.engine"));
    assert!(stdout.contains("unsupported value \"magic\""));
}
