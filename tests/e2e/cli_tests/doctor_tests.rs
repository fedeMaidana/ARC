// ─── < Imports > ────────────────────────────────────────────────────

use super::common::{TestFixture, assert_success, stdout};
use std::fs;

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

#[test]
fn doctor_fails_when_registered_agent_points_to_arc_launcher() {
    let fixture = TestFixture::new("doctor-contaminated-registry").without_system_path();

    let launcher_path = fixture.launcher_dir().join("opencode");

    fs::create_dir_all(fixture.launcher_dir()).expect("launcher dir should be created");
    fs::write(&launcher_path, "#!/usr/bin/env sh\nexit 99\n").expect("fake launcher should be written");
    make_test_executable(&launcher_path);

    let registry_content = format!(
        r#"{{
  "schema_version": 1,
  "agents": [
    {{
      "id": "opencode",
      "display_name": "OpenCode",
      "kind": "local_agent",
      "enabled": true,
      "command": "opencode",
      "path": "{}",
      "source": "detected"
    }}
  ]
}}
"#,
        launcher_path.display()
    );

    fs::write(fixture.registry_path(), registry_content).expect("registry should be written");

    let doctor_output = fixture.run(&["doctor"]);

    assert_eq!(doctor_output.status.code(), Some(1));

    let stdout = stdout(&doctor_output);

    assert!(stdout.contains("registered agent paths"));
    assert!(stdout.contains("opencode points to ARC-managed path"));
    assert!(stdout.contains("run `arc agents sync`"));
    assert!(stdout.contains("ARC is not ready yet"));
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
