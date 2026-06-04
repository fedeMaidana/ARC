// ─── < Imports > ────────────────────────────────────────────────────

use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::Value;

// ─── < Public Helpers: Commands > ───────────────────────────────────

pub fn run_arc(args: &[&str]) -> Output {
    let registry_path = env::temp_dir().join(format!("arc-e2e-run-{}.agents.json", std::process::id()));
    let mut command = Command::new(env!("CARGO_BIN_EXE_arc"));

    command
        .args(args)
        .env("ARC_AUDIT_ENABLED", "false")
        .env("ARC_AGENT_REGISTRY_PATH", registry_path)
        .env_remove("ARC_AGENT_SOURCES")
        .env_remove("ARC_SOURCE");

    command.output().expect("failed to execute arc binary")
}

pub struct TestFixture {
    root_dir: PathBuf,
    env_overrides: Vec<(String, String)>,
    path_prepend: Vec<PathBuf>,
    use_system_path: bool,
}

impl TestFixture {
    pub fn new(name: &str) -> Self {
        let root_dir = unique_temp_dir(name);

        fs::create_dir_all(&root_dir).expect("test fixture directory should be created");

        Self {
            root_dir,
            env_overrides: Vec::new(),
            path_prepend: Vec::new(),
            use_system_path: true,
        }
    }

    pub fn with_env(name: &str, key: &str, value: &str) -> Self {
        let mut fixture = Self::new(name);

        fixture.set_env(key, value);

        fixture
    }

    pub fn registry_path(&self) -> PathBuf {
        self.root_dir.join("agents.json")
    }

    pub fn set_env(&mut self, key: &str, value: impl Into<String>) {
        self.env_overrides.push((key.to_string(), value.into()));
    }

    pub fn without_system_path(mut self) -> Self {
        self.use_system_path = false;
        self
    }

    pub fn create_path_command(&mut self, name: &str) -> PathBuf {
        let bin_dir = self.root_dir.join("bin");

        fs::create_dir_all(&bin_dir).expect("test bin directory should be created");

        if !self.path_prepend.iter().any(|path| path == &bin_dir) {
            self.path_prepend.push(bin_dir.clone());
        }

        let command_path = bin_dir.join(name);

        fs::write(&command_path, "#!/usr/bin/env sh\nexit 0\n").expect("test command should be written");
        make_executable(&command_path);

        command_path
    }

    pub fn run(&self, args: &[&str]) -> Output {
        let mut command = self.base_command(args);

        command.output().expect("failed to execute arc binary")
    }

    pub fn run_with_stdin(&self, args: &[&str], stdin: &str) -> Output {
        let mut child = self
            .base_command(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("failed to execute arc binary");

        child
            .stdin
            .as_mut()
            .expect("stdin should be available")
            .write_all(stdin.as_bytes())
            .expect("stdin should be written");

        child.wait_with_output().expect("arc output should be captured")
    }

    fn base_command(&self, args: &[&str]) -> Command {
        let mut command = Command::new(env!("CARGO_BIN_EXE_arc"));

        command
            .args(args)
            .env("ARC_AUDIT_ENABLED", "false")
            .env("ARC_POLICY_ENGINE", "native")
            .env("ARC_EXECUTION_WORKING_DIRECTORY", ".")
            .env("ARC_AGENT_REGISTRY_PATH", self.registry_path())
            .env_remove("ARC_AGENT_SOURCES")
            .env_remove("ARC_SOURCE")
            .current_dir(&self.root_dir);

        if let Some(path) = test_path(&self.path_prepend, self.use_system_path) {
            command.env("PATH", path);
        }

        for (key, value) in &self.env_overrides {
            command.env(key, value);
        }

        command
    }
}

impl Drop for TestFixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root_dir);
    }
}

// ─── < Public Helpers: Assertions > ─────────────────────────────────

pub fn assert_success(output: &Output) {
    assert_eq!(output.status.code(), Some(0), "stdout:\n{}\nstderr:\n{}", stdout(output), stderr(output));
}

pub fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).to_string()
}

pub fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).to_string()
}

pub fn json_stdout(output: &Output) -> Value {
    let stdout = stdout(output);

    serde_json::from_str(&stdout).unwrap_or_else(|error| {
        panic!("stdout should be valid JSON: {error}\nstdout:\n{stdout}\nstderr:\n{}", stderr(output));
    })
}

// ─── < Private Helpers > ────────────────────────────────────────────

fn unique_temp_dir(name: &str) -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after unix epoch")
        .as_nanos();

    env::temp_dir().join(format!("arc-e2e-{name}-{}-{timestamp}", std::process::id()))
}

fn test_path(path_prepend: &[PathBuf], use_system_path: bool) -> Option<std::ffi::OsString> {
    if path_prepend.is_empty() && use_system_path {
        return None;
    }

    let mut paths = path_prepend.to_vec();

    if use_system_path {
        let existing_path = env::var_os("PATH").unwrap_or_default();

        paths.extend(env::split_paths(&existing_path));
    }

    env::join_paths(paths).ok()
}

#[cfg(unix)]
fn make_executable(path: &Path) {
    use std::os::unix::fs::PermissionsExt;

    let mut permissions = fs::metadata(path).expect("test command metadata should be readable").permissions();

    permissions.set_mode(0o755);

    fs::set_permissions(path, permissions).expect("test command permissions should be updated");
}

#[cfg(not(unix))]
fn make_executable(_path: &Path) {}
