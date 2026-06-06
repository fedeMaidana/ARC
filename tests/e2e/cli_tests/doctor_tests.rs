// ─── < Imports > ────────────────────────────────────────────────────

use super::common::{TestFixture, assert_success, stdout};

// ─── < Tests > ──────────────────────────────────────────────────────

#[test]
fn doctor_reports_failures_when_launcher_dir_is_not_in_path() {
    let mut fixture = TestFixture::new("doctor-path-missing").without_system_path();

    fixture.create_path_command("opencode");

    let init_output = fixture.run(&["init"]);
    assert_success(&init_output);

    let shims_output = fixture.run(&["shims", "install"]);
    assert_success(&shims_output);

    let doctor_output = fixture.run(&["doctor"]);

    assert_eq!(doctor_output.status.code(), Some(1));

    let stdout = stdout(&doctor_output);

    assert!(stdout.contains("ARC doctor"));
    assert!(stdout.contains("launcher PATH"));
    assert!(stdout.contains("launcher directory is not in PATH"));
    assert!(stdout.contains("ARC is not ready yet"));
}

#[test]
fn doctor_passes_when_registered_agent_commands_resolve_to_launchers() {
    let mut fixture = TestFixture::new("doctor-ready").without_system_path();

    fixture.create_path_command("opencode");

    let init_output = fixture.run(&["init"]);
    assert_success(&init_output);

    let shims_output = fixture.run(&["shims", "install"]);
    assert_success(&shims_output);

    fixture.set_env("PATH", fixture.launcher_dir().display().to_string());

    let doctor_output = fixture.run(&["doctor"]);
    assert_success(&doctor_output);

    let stdout = stdout(&doctor_output);

    assert!(stdout.contains("ARC doctor"));
    assert!(stdout.contains("policy"));
    assert!(stdout.contains("agent registry"));
    assert!(stdout.contains("registered agents"));
    assert!(stdout.contains("launcher shims"));
    assert!(stdout.contains("runtime shims"));
    assert!(stdout.contains("launcher PATH"));
    assert!(stdout.contains("agent command resolution"));
    assert!(stdout.contains("ARC is ready"));
}
