// ─── < Imports > ────────────────────────────────────────────────────

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::Value;

// ─── < Tests: CLI Help And Errors > ─────────────────────────────────

#[test]
fn help_command_prints_usage() {
    let output = run_arc(&["help"]);

    assert_eq!(output.status.code(), Some(2));

    let stdout = stdout(&output);

    assert!(stdout.contains("ARC"));
    assert!(stdout.contains("Usage"));
    assert!(stdout.contains("arc run <command> [args...]"));
}

#[test]
fn config_help_command_prints_config_usage() {
    let output = run_arc(&["config", "help"]);

    assert_eq!(output.status.code(), Some(2));

    let stdout = stdout(&output);

    assert!(stdout.contains("Config usage"));
    assert!(stdout.contains("arc config path"));
    assert!(stdout.contains("arc config check"));
    assert!(stdout.contains("arc config show"));
}

#[test]
fn unknown_config_command_prints_cli_error() {
    let output = run_arc(&["config", "nope"]);

    assert_eq!(output.status.code(), Some(2));

    let stdout = stdout(&output);

    assert!(stdout.contains("CLI error"));
    assert!(stdout.contains("unknown config command 'nope'"));
    assert!(stdout.contains("Usage"));
}

#[test]
fn check_without_action_prints_cli_error() {
    let output = run_arc(&["check"]);

    assert_eq!(output.status.code(), Some(2));

    let stdout = stdout(&output);

    assert!(stdout.contains("CLI error"));
    assert!(stdout.contains("missing action after 'check'"));
    assert!(stdout.contains("Usage"));
}

// ─── < Tests: Config Commands > ─────────────────────────────────────

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

// ─── < Tests: Human CLI Policy Flow > ───────────────────────────────

#[test]
fn check_allowed_command_returns_success_without_running_command() {
    let fixture = TestFixture::new("check-allowed-command");
    let output = fixture.run(&["check", "run", "echo", "hello"]);

    assert_success(&output);

    let stdout = stdout(&output);

    assert!(stdout.contains("Decision"));
    assert!(stdout.contains("allow"));
    assert!(stdout.contains("Check mode"));
    assert!(stdout.contains("Execution skipped"));
}

#[test]
fn run_allowed_command_executes_and_prints_output() {
    let fixture = TestFixture::new("run-allowed-command");
    let output = fixture.run(&["run", "echo", "hello"]);

    assert_success(&output);

    let stdout = stdout(&output);

    assert!(stdout.contains("Decision"));
    assert!(stdout.contains("allow"));
    assert!(stdout.contains("Execution"));
    assert!(stdout.contains("Output"));
    assert!(stdout.contains("hello"));
}

#[test]
fn check_blocked_command_returns_non_zero_exit_code() {
    let fixture = TestFixture::new("check-blocked-command");
    let output = fixture.run(&["check", "run", "rm", "-rf", "/"]);

    assert_eq!(output.status.code(), Some(1));

    let stdout = stdout(&output);

    assert!(stdout.contains("Decision"));
    assert!(stdout.contains("deny"));
    assert!(stdout.contains("command is explicitly blocked by console policy"));
    assert!(stdout.contains("Check mode"));
}

#[test]
fn check_blocked_command_subcommand_returns_non_zero_exit_code() {
    let fixture = TestFixture::new("check-blocked-subcommand");
    let output = fixture.run(&["check", "run", "git", "push"]);

    assert_eq!(output.status.code(), Some(1));

    let stdout = stdout(&output);

    assert!(stdout.contains("Decision"));
    assert!(stdout.contains("deny"));
    assert!(stdout.contains("subcommand is explicitly blocked by command policy"));
}

#[test]
fn check_ask_command_subcommand_returns_success_but_marks_ask() {
    let fixture = TestFixture::new("check-ask-subcommand");
    let output = fixture.run(&["check", "run", "git", "commit"]);

    assert_success(&output);

    let stdout = stdout(&output);

    assert!(stdout.contains("Decision"));
    assert!(stdout.contains("ask"));
    assert!(stdout.contains("subcommand requires manual approval"));
    assert!(stdout.contains("Check mode"));
}

// ─── < Tests: JSON CLI Flow > ───────────────────────────────────────

#[test]
fn decide_json_allowed_command_returns_machine_readable_response() {
    let fixture = TestFixture::new("decide-json-allowed-command");

    let output = fixture.run_with_stdin(&["decide", "--json"], r#"{"action":"run","command":["echo","hello"]}"#);

    assert_success(&output);

    let response = json_stdout(&output);

    assert_eq!(response["ok"], true);
    assert_eq!(response["api_version"], "1");
    assert_eq!(response["kind"], "decision");

    assert_eq!(response["request"]["mode"], "check");
    assert_eq!(response["request"]["action"], "run");
    assert_eq!(response["request"]["resource"], "echo hello");

    assert_eq!(response["decision"]["status"], "allow");
    assert_eq!(response["decision"]["reason"], "request matches an allowed policy");
    assert_eq!(response["decision"]["reason_code"], "action_allowed");

    assert_eq!(response["execution"]["kind"], "check_mode");
    assert_eq!(response["execution"]["allowed"], true);
    assert_eq!(response["execution"]["executed"], false);
    assert_eq!(response["execution"]["exit_code"], 0);
}

#[test]
fn decide_json_blocked_command_returns_non_zero_machine_readable_response() {
    let fixture = TestFixture::new("decide-json-blocked-command");

    let output = fixture.run_with_stdin(&["decide", "--json"], r#"{"action":"run","command":["rm","-rf","/"]}"#);

    assert_eq!(output.status.code(), Some(1));

    let response = json_stdout(&output);

    assert_eq!(response["ok"], true);
    assert_eq!(response["api_version"], "1");
    assert_eq!(response["kind"], "decision");

    assert_eq!(response["decision"]["status"], "deny");
    assert_eq!(response["decision"]["reason"], "command is explicitly blocked by console policy");
    assert_eq!(response["decision"]["reason_code"], "console_command_blocked");

    assert_eq!(response["execution"]["kind"], "check_mode");
    assert_eq!(response["execution"]["allowed"], false);
    assert_eq!(response["execution"]["executed"], false);
    assert_eq!(response["execution"]["exit_code"], 1);
}

#[test]
fn decide_json_invalid_request_returns_json_error() {
    let fixture = TestFixture::new("decide-json-invalid-request");

    let output = fixture.run_with_stdin(&["decide", "--json"], r#"{"action":"run"}"#);

    assert_eq!(output.status.code(), Some(2));

    let response = json_stdout(&output);

    assert_eq!(response["ok"], false);
    assert_eq!(response["api_version"], "1");
    assert_eq!(response["kind"], "error");
    assert_eq!(response["error_code"], "missing_command");
    assert_eq!(response["error"], "run action requires a command array");
}

// ─── < Helpers: Commands > ──────────────────────────────────────────

fn run_arc(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_arc"))
        .args(args)
        .output()
        .expect("failed to execute arc binary")
}

struct TestFixture {
    root_dir: PathBuf,
    config_path: PathBuf,
}

impl TestFixture {
    fn new(name: &str) -> Self {
        let root_dir = unique_temp_dir(name);

        fs::create_dir_all(&root_dir).expect("test fixture directory should be created");

        let config_path = root_dir.join("arc.toml");

        fs::write(&config_path, test_config_content(&root_dir)).expect("test config should be written");

        Self { root_dir, config_path }
    }

    fn with_config_content(name: &str, config_content: String) -> Self {
        let root_dir = unique_temp_dir(name);

        fs::create_dir_all(&root_dir).expect("test fixture directory should be created");

        let config_path = root_dir.join("arc.toml");

        fs::write(&config_path, config_content).expect("test config should be written");

        Self { root_dir, config_path }
    }

    fn run(&self, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_arc"))
            .args(args)
            .env("ARC_CONFIG", &self.config_path)
            .current_dir(&self.root_dir)
            .output()
            .expect("failed to execute arc binary")
    }

    fn run_with_stdin(&self, args: &[&str], stdin: &str) -> Output {
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

// ─── < Helpers: Assertions > ────────────────────────────────────────

fn assert_success(output: &Output) {
    assert_eq!(output.status.code(), Some(0), "stdout:\n{}\nstderr:\n{}", stdout(output), stderr(output));
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).to_string()
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).to_string()
}

fn json_stdout(output: &Output) -> Value {
    let stdout = stdout(output);

    serde_json::from_str(&stdout).unwrap_or_else(|error| {
        panic!("stdout should be valid JSON: {error}\nstdout:\n{stdout}\nstderr:\n{}", stderr(output));
    })
}

// ─── < Helpers: Fixtures > ──────────────────────────────────────────

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

fn invalid_config_content() -> String {
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
