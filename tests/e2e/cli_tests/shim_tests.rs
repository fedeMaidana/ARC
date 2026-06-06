// ─── < Imports > ────────────────────────────────────────────────────

use std::fs;

use super::common::{TestFixture, assert_success, stdout};

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
fn shims_install_creates_launcher_for_registered_agent() {
    let mut fixture = TestFixture::new("shims-install").without_system_path();

    let real_command_path = fixture.create_path_command("opencode");

    let sync_output = fixture.run(&["agents", "sync"]);
    assert_success(&sync_output);

    let install_output = fixture.run(&["shims", "install"]);
    assert_success(&install_output);

    let stdout = stdout(&install_output);
    let launcher_path = fixture.launcher_dir().join("opencode");

    assert!(stdout.contains("ARC launcher shims installed"));
    assert!(stdout.contains("opencode"));
    assert!(stdout.contains(&launcher_path.display().to_string()));
    assert!(stdout.contains(&real_command_path.display().to_string()));

    let launcher_content = fs::read_to_string(&launcher_path).expect("launcher shim should exist");

    assert!(launcher_content.contains("ARC_AGENT_ID='opencode'"));
    assert!(launcher_content.contains("ARC_AGENT_COMMAND='opencode'"));
    assert!(launcher_content.contains(&format!("ARC_AGENT_REAL_PATH='{}'", real_command_path.display())));
    assert!(launcher_content.contains("export ARC_SOURCE=\"$ARC_AGENT_ID\""));
    assert!(launcher_content.contains("exec \"$ARC_AGENT_REAL_PATH\" \"$@\""));
}

#[test]
fn shims_list_reports_missing_and_installed_launchers() {
    let mut fixture = TestFixture::new("shims-list").without_system_path();

    fixture.create_path_command("opencode");

    let sync_output = fixture.run(&["agents", "sync"]);
    assert_success(&sync_output);

    let missing_output = fixture.run(&["shims", "list"]);
    assert_success(&missing_output);

    let missing_stdout = stdout(&missing_output);

    assert!(missing_stdout.contains("ARC launcher shims"));
    assert!(missing_stdout.contains("opencode"));
    assert!(missing_stdout.contains("missing"));

    let install_output = fixture.run(&["shims", "install"]);
    assert_success(&install_output);

    let installed_output = fixture.run(&["shims", "list"]);
    assert_success(&installed_output);

    let installed_stdout = stdout(&installed_output);

    assert!(installed_stdout.contains("ARC launcher shims"));
    assert!(installed_stdout.contains("opencode"));
    assert!(installed_stdout.contains("installed"));
}
