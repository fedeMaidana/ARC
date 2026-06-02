// ─── < Imports > ────────────────────────────────────────────────────

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::Value;

// ─── < Public Helpers: Commands > ───────────────────────────────────

pub fn run_arc(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_arc"))
        .args(args)
        .output()
        .expect("failed to execute arc binary")
}

pub struct TestFixture {
    root_dir: PathBuf,
    pub config_path: PathBuf,
}

impl TestFixture {
    pub fn new(name: &str) -> Self {
        let root_dir = unique_temp_dir(name);

        fs::create_dir_all(&root_dir).expect("test fixture directory should be created");

        let config_path = root_dir.join("arc.toml");

        fs::write(&config_path, test_config_content(&root_dir)).expect("test config should be written");

        Self { root_dir, config_path }
    }

    pub fn with_config_content(name: &str, config_content: String) -> Self {
        let root_dir = unique_temp_dir(name);

        fs::create_dir_all(&root_dir).expect("test fixture directory should be created");

        let config_path = root_dir.join("arc.toml");

        fs::write(&config_path, config_content).expect("test config should be written");

        Self { root_dir, config_path }
    }

    pub fn run(&self, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_arc"))
            .args(args)
            .env("ARC_CONFIG", &self.config_path)
            .current_dir(&self.root_dir)
            .output()
            .expect("failed to execute arc binary")
    }

    pub fn run_with_stdin(&self, args: &[&str], stdin: &str) -> Output {
        let mut child = Command::new(env!("CARGO_BIN_EXE_arc"))
            .args(args)
            .env("ARC_CONFIG", &self.config_path)
            .current_dir(&self.root_dir)
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

// ─── < Public Helpers: Fixtures > ───────────────────────────────────

pub fn invalid_config_content() -> String {
    r#"config_version = 1

[policy]
  default_action = "deny"

[actions]
  allowed = ["run"]
  blocked = []
  need_resource = ["run"]
  ask = []

[resources]
  protected = []
  blocked_path_prefixes = []

[http]
  allowed_schemes = ["https"]
  blocked_hosts = []
  blocked_cidrs = ["192.168/33"]

[console]
  default_command_policy = "deny"
  allow_path_resolution = true
  blocked_arguments = ["-rf"]

  [[console.commands]]
    name = "git"
    mode = "alow"
    allowed_paths = []

[audit]
  enabled = false
  path = "audit.log"

[execution]
  timeout_seconds = 10
  max_output_bytes = 100000
  inherit_environment = false
  environment = []
"#
    .to_string()
}

// ─── < Private Helpers > ────────────────────────────────────────────

fn unique_temp_dir(name: &str) -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after unix epoch")
        .as_nanos();

    std::env::temp_dir().join(format!("arc-e2e-{name}-{}-{timestamp}", std::process::id()))
}

fn test_config_content(root_dir: &Path) -> String {
    format!(
        r#"[actions]
  allowed = [
    "list_files",
    "read_file",
    "http_get",
    "run"
  ]

  blocked = [
    "delete_file",
    "write_file",
    "run_shell"
  ]

  need_resource = [
    "read_file",
    "http_get",
    "run"
  ]

  ask = []

[resources]
  protected = [
    ".env",
    "id_rsa",
    "secrets.txt"
  ]

  blocked_path_prefixes = [
    "/etc/",
    "/root/",
    "../"
  ]

[http]
  blocked_targets = [
    "http://localhost",
    "https://localhost",
    "http://127.0.0.1",
    "https://127.0.0.1",
    "http://0.0.0.0",
    "https://0.0.0.0",
    "http://[::1]",
    "https://[::1]"
  ]

[console]
  allowed_commands = [
    "cargo",
    "git",
    "rg",
    "ls",
    "pwd",
    "cat",
    "echo",
    "whoami",
    "date"
  ]

  blocked_commands = [
    "rm",
    "sudo",
    "su",
    "sh",
    "bash",
    "zsh",
    "fish",
    "chmod",
    "chown",
    "curl",
    "wget",
    "ssh",
    "scp",
    "nc",
    "ncat",
    "dd",
    "mkfs",
    "mount",
    "umount",
    "systemctl",
    "kill",
    "pkill"
  ]

  blocked_arguments = [
    "-rf",
    "--no-preserve-root",
    "/",
    "/etc",
    "/root",
    "..",
    "~"
  ]

  ask_commands = []

  [[console.command_rules]]
    name = "git"
    allowed_subcommands = [
      "status",
      "diff",
      "log",
      "show",
      "branch"
    ]
    ask_subcommands = [
      "add",
      "commit"
    ]
    blocked_subcommands = [
      "push",
      "credential",
      "remote"
    ]
    blocked_arguments = [
      "--upload-pack",
      "--receive-pack"
    ]
    ask_arguments = []

  [[console.command_rules]]
    name = "cargo"
    allowed_subcommands = [
      "build",
      "check",
      "fmt",
      "test",
      "clippy",
      "nextest"
    ]
    ask_subcommands = [
      "run"
    ]
    blocked_subcommands = [
      "publish",
      "install",
      "login",
      "owner"
    ]
    blocked_arguments = []
    ask_arguments = [
      "--release"
    ]

[audit]
  enabled = false
  path = "{}/audit.log"

[execution]
  timeout_seconds = 10
  max_output_bytes = 100000
  inherit_environment = false
  working_directory = "."
  environment = []
"#,
        root_dir.display()
    )
}
