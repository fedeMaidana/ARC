// ─── < Imports > ────────────────────────────────────────────────────

use std::fs;

use super::common::{TestFixture, assert_success, run_arc, stderr, stdout};

// ─── < Tests > ──────────────────────────────────────────────────────

#[test]
fn shims_path_prints_shim_directories() {
    let fixture = TestFixture::new("shims-path");
    let output = fixture.run(&["shims", "path"]);

    assert_success(&output);

    let stdout = stdout(&output);

    assert!(stdout.contains("ARC shims paths"));
    assert!(stdout.contains("launchers"));
    assert!(stdout.contains("runtime shims"));
    assert!(stdout.contains(&fixture.launcher_dir().display().to_string()));
    assert!(stdout.contains(&fixture.runtime_shims_dir().display().to_string()));
}

#[test]
fn shims_install_creates_launcher_and_runtime_shims() {
    let mut fixture = TestFixture::new("shims-install").without_system_path();

    let real_command_path = fixture.create_path_command("opencode");

    let sync_output = fixture.run(&["agents", "sync"]);
    assert_success(&sync_output);

    let install_output = fixture.run(&["shims", "install"]);
    assert_success(&install_output);

    let stdout = stdout(&install_output);
    let launcher_path = fixture.launcher_dir().join("opencode");
    let bash_runtime_shim = fixture.runtime_shims_dir().join("bash");
    let sh_runtime_shim = fixture.runtime_shims_dir().join("sh");

    assert!(stdout.contains("ARC shims installed"));
    assert!(stdout.contains("Launchers"));
    assert!(stdout.contains("Runtime shims"));
    assert!(stdout.contains("opencode"));
    assert!(stdout.contains("bash"));
    assert!(stdout.contains("sh"));
    assert!(stdout.contains(&launcher_path.display().to_string()));
    assert!(stdout.contains(&real_command_path.display().to_string()));

    let launcher_content = fs::read_to_string(&launcher_path).expect("launcher shim should exist");

    assert!(launcher_content.contains("ARC_AGENT_ID='opencode'"));
    assert!(launcher_content.contains("ARC_AGENT_COMMAND='opencode'"));
    assert!(launcher_content.contains(&format!("ARC_AGENT_REAL_PATH='{}'", real_command_path.display())));
    assert!(launcher_content.contains("export ARC_SOURCE=\"$ARC_AGENT_ID\""));
    assert!(launcher_content.contains("ARC_ORIGINAL_PATH=\"${ARC_ORIGINAL_PATH:-$PATH}\""));
    assert!(launcher_content.contains("PATH=\"$ARC_RUNTIME_SHIMS_DIR:$ARC_ORIGINAL_PATH\""));
    assert!(launcher_content.contains("exec \"$ARC_AGENT_REAL_PATH\" \"$@\""));

    let bash_runtime_content = fs::read_to_string(&bash_runtime_shim).expect("bash runtime shim should exist");
    let sh_runtime_content = fs::read_to_string(&sh_runtime_shim).expect("sh runtime shim should exist");

    assert!(bash_runtime_content.contains("__arc-shim shell bash"));
    assert!(sh_runtime_content.contains("__arc-shim shell sh"));
}

#[test]
fn shims_list_reports_missing_and_installed_shims() {
    let mut fixture = TestFixture::new("shims-list").without_system_path();

    fixture.create_path_command("opencode");

    let sync_output = fixture.run(&["agents", "sync"]);
    assert_success(&sync_output);

    let missing_output = fixture.run(&["shims", "list"]);
    assert_success(&missing_output);

    let missing_stdout = stdout(&missing_output);

    assert!(missing_stdout.contains("ARC shims"));
    assert!(missing_stdout.contains("opencode"));
    assert!(missing_stdout.contains("bash"));
    assert!(missing_stdout.contains("sh"));
    assert!(missing_stdout.contains("missing"));

    let install_output = fixture.run(&["shims", "install"]);
    assert_success(&install_output);

    let installed_output = fixture.run(&["shims", "list"]);
    assert_success(&installed_output);

    let installed_stdout = stdout(&installed_output);

    assert!(installed_stdout.contains("ARC shims"));
    assert!(installed_stdout.contains("opencode"));
    assert!(installed_stdout.contains("bash"));
    assert!(installed_stdout.contains("sh"));
    assert!(installed_stdout.contains("installed"));
}

#[test]
fn internal_shell_shim_routes_simple_shell_command_through_arc() {
    let output = run_arc(&["__arc-shim", "shell", "sh", "-c", "echo hola"]);

    assert_success(&output);

    let stdout = stdout(&output);

    assert!(stdout.contains("hola"));
    assert!(!stdout.contains("ARC v"));
}

#[test]
fn internal_shell_shim_rejects_unsupported_shell_syntax() {
    let output = run_arc(&["__arc-shim", "shell", "sh", "-c", "echo hola && echo chau"]);

    assert_eq!(output.status.code(), Some(126));

    let stderr = stderr(&output);

    assert!(stderr.contains("ARC blocked unsupported shell command"));
    assert!(stderr.contains("unsupported shell operator '&'"));
}
