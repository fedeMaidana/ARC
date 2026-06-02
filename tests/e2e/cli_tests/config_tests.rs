// ─── < Imports > ────────────────────────────────────────────────────

use super::common::{TestFixture, assert_success, invalid_config_content, stdout};

// ─── < Tests > ──────────────────────────────────────────────────────

#[test]
fn config_show_prints_loaded_config() {
    let fixture = TestFixture::new("config-show");
    let output = fixture.run(&["config", "show"]);

    assert_success(&output);

    let stdout = stdout(&output);

    assert!(stdout.contains("Config"));
    assert!(stdout.contains("Actions"));
    assert!(stdout.contains("Console"));
    assert!(stdout.contains("Execution"));
    assert!(stdout.contains("inherit environment"));
    assert!(stdout.contains("command rules"));
    assert!(stdout.contains("git"));
}

#[test]
fn config_path_prints_config_path_from_environment() {
    let fixture = TestFixture::new("config-path");
    let output = fixture.run(&["config", "path"]);

    assert_success(&output);

    let stdout = stdout(&output);

    assert!(stdout.contains("Config path"));
    assert!(stdout.contains(fixture.config_path.to_string_lossy().as_ref()));
}

#[test]
fn config_check_prints_success_for_valid_config() {
    let fixture = TestFixture::new("config-check-valid");
    let output = fixture.run(&["config", "check"]);

    assert_success(&output);

    let stdout = stdout(&output);

    assert!(stdout.contains("Config is valid"));
    assert!(stdout.contains(fixture.config_path.to_string_lossy().as_ref()));
}

#[test]
fn config_check_prints_validation_errors_for_invalid_config() {
    let fixture = TestFixture::with_config_content("config-check-invalid", invalid_config_content());
    let output = fixture.run(&["config", "check"]);

    assert_eq!(output.status.code(), Some(2));

    let stdout = stdout(&output);

    assert!(stdout.contains("Config error"));
    assert!(stdout.contains("console.commands[git].mode"));
    assert!(stdout.contains("unsupported value \"alow\""));
    assert!(stdout.contains("http.blocked_cidrs[0]"));
    assert!(stdout.contains("invalid CIDR \"192.168/33\""));
}
