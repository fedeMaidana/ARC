// ─── < Imports > ────────────────────────────────────────────────────

use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use arc::config::{ConsoleCommandRule, ConsoleConfig, ExecutionConfig, ExecutionEnvironmentVariable};
use arc::decision::{Decision, DecisionReason};
use arc::executor::{self, ExecutionReport};
use arc::request::{Request, RequestMode};

// ─── < Tests > ──────────────────────────────────────────────────────

#[test]
fn execute_captures_command_output() {
    let request = Request::new(RequestMode::Execute, "run".to_string(), vec!["echo".to_string(), "hola".to_string()]);

    let decision = Decision::allow(DecisionReason::ActionAllowed);
    let execution_config = execution_config(10, 100_000);
    let console_config = console_config();

    let report = executor::execute(&request, &decision, &execution_config, &console_config);

    match report {
        ExecutionReport::CommandFinished(report) => {
            assert!(report.success);
            assert_eq!(report.exit_code, 0);
            assert_eq!(report.stdout.trim(), "hola");
            assert!(!report.stdout_truncated);
            assert!(!report.stderr_truncated);
        }
        _ => panic!("expected command finished"),
    }
}

#[test]
fn execute_truncates_large_stdout() {
    let request = Request::new(RequestMode::Execute, "run".to_string(), vec!["printf".to_string(), "1234567890".to_string()]);

    let decision = Decision::allow(DecisionReason::ActionAllowed);
    let execution_config = execution_config(10, 4);
    let console_config = console_config();

    let report = executor::execute(&request, &decision, &execution_config, &console_config);

    match report {
        ExecutionReport::CommandFinished(report) => {
            assert!(report.success);
            assert_eq!(report.stdout, "1234");
            assert!(report.stdout_truncated);
        }
        _ => panic!("expected command finished"),
    }
}

#[test]
fn execute_times_out_long_running_command() {
    let request = Request::new(RequestMode::Execute, "run".to_string(), vec!["sleep".to_string(), "2".to_string()]);

    let decision = Decision::allow(DecisionReason::ActionAllowed);
    let execution_config = execution_config(1, 100_000);
    let console_config = console_config();

    let report = executor::execute(&request, &decision, &execution_config, &console_config);

    match report {
        ExecutionReport::CommandTimedOut(report) => {
            assert_eq!(report.command_line, "sleep 2");
            assert_eq!(report.timeout_seconds, 1);
        }
        _ => panic!("expected command timeout"),
    }
}

#[test]
fn execute_uses_configured_working_directory() {
    let working_directory = unique_temp_dir("arc-executor-working-directory");

    fs::create_dir_all(&working_directory).expect("working directory should be created");

    let working_directory = fs::canonicalize(&working_directory).expect("working directory should be canonicalized");

    let request = Request::new(RequestMode::Execute, "run".to_string(), vec!["pwd".to_string()]);

    let decision = Decision::allow(DecisionReason::ActionAllowed);
    let mut execution_config = execution_config(10, 100_000);
    let console_config = console_config();

    execution_config.working_directory = Some(working_directory.to_string_lossy().to_string());

    let report = executor::execute(&request, &decision, &execution_config, &console_config);

    let _ = fs::remove_dir_all(&working_directory);

    match report {
        ExecutionReport::CommandFinished(report) => {
            assert!(report.success);
            assert_eq!(report.stdout.trim(), working_directory.to_string_lossy());
        }
        _ => panic!("expected command finished"),
    }
}

#[test]
fn execute_injects_configured_environment_variables() {
    let request =
        Request::new(RequestMode::Execute, "run".to_string(), vec!["sh".to_string(), "-c".to_string(), "printf \"$ARC_EXECUTOR_TEST_VALUE\"".to_string()]);

    let decision = Decision::allow(DecisionReason::ActionAllowed);
    let mut execution_config = execution_config(10, 100_000);
    let console_config = console_config();

    execution_config.environment.push(ExecutionEnvironmentVariable {
        name: "ARC_EXECUTOR_TEST_VALUE".to_string(),
        value: "configured".to_string(),
    });

    let report = executor::execute(&request, &decision, &execution_config, &console_config);

    match report {
        ExecutionReport::CommandFinished(report) => {
            assert!(report.success);
            assert_eq!(report.stdout, "configured");
        }
        _ => panic!("expected command finished"),
    }
}

#[test]
fn execute_reports_command_not_found_before_spawning() {
    let request = Request::new(RequestMode::Execute, "run".to_string(), vec!["__arc_command_that_should_not_exist__".to_string()]);

    let decision = Decision::allow(DecisionReason::ActionAllowed);
    let execution_config = execution_config(10, 100_000);
    let console_config = console_config();

    let report = executor::execute(&request, &decision, &execution_config, &console_config);

    match report {
        ExecutionReport::CommandFailed(error) => {
            assert_eq!(error.command_line, "__arc_command_that_should_not_exist__");
            assert!(error.details.contains("command not found in PATH"));
        }
        _ => panic!("expected command failed"),
    }
}

#[test]
fn execute_rejects_path_resolution_when_disabled() {
    let request = Request::new(RequestMode::Execute, "run".to_string(), vec!["echo".to_string(), "hola".to_string()]);

    let decision = Decision::allow(DecisionReason::ActionAllowed);
    let execution_config = execution_config(10, 100_000);
    let mut console_config = console_config();

    console_config.allow_path_resolution = false;

    let report = executor::execute(&request, &decision, &execution_config, &console_config);

    match report {
        ExecutionReport::CommandFailed(error) => {
            assert_eq!(error.command_line, "echo hola");
            assert!(error.details.contains("PATH resolution is disabled"));
        }
        _ => panic!("expected command failed"),
    }
}

#[test]
fn execute_uses_configured_allowed_command_path() {
    let Some(echo_path) = find_executable_in_path("echo") else {
        return;
    };

    let request = Request::new(RequestMode::Execute, "run".to_string(), vec!["echo".to_string(), "hola".to_string()]);

    let decision = Decision::allow(DecisionReason::ActionAllowed);
    let execution_config = execution_config(10, 100_000);
    let mut console_config = console_config();

    console_config.allow_path_resolution = false;
    console_config.command_rules.push(ConsoleCommandRule {
        name: "echo".to_string(),
        mode: "allow".to_string(),
        risk: None,
        allowed_paths: vec![echo_path.to_string_lossy().to_string()],
        allowed_subcommands: Vec::new(),
        blocked_subcommands: Vec::new(),
        ask_subcommands: Vec::new(),
        blocked_arguments: Vec::new(),
        ask_arguments: Vec::new(),
    });

    let report = executor::execute(&request, &decision, &execution_config, &console_config);

    match report {
        ExecutionReport::CommandFinished(report) => {
            assert!(report.success);
            assert_eq!(report.stdout.trim(), "hola");
        }
        _ => panic!("expected command finished"),
    }
}

#[test]
fn execute_rejects_allowed_path_with_wrong_binary_name() {
    let Some(echo_path) = find_executable_in_path("echo") else {
        return;
    };

    let request = Request::new(RequestMode::Execute, "run".to_string(), vec!["git".to_string(), "status".to_string()]);

    let decision = Decision::allow(DecisionReason::ActionAllowed);
    let execution_config = execution_config(10, 100_000);
    let mut console_config = console_config();

    console_config.allow_path_resolution = false;
    console_config.command_rules.push(ConsoleCommandRule {
        name: "git".to_string(),
        mode: "allow".to_string(),
        risk: None,
        allowed_paths: vec![echo_path.to_string_lossy().to_string()],
        allowed_subcommands: Vec::new(),
        blocked_subcommands: Vec::new(),
        ask_subcommands: Vec::new(),
        blocked_arguments: Vec::new(),
        ask_arguments: Vec::new(),
    });

    let report = executor::execute(&request, &decision, &execution_config, &console_config);

    match report {
        ExecutionReport::CommandFailed(error) => {
            assert_eq!(error.command_line, "git status");
            assert!(error.details.contains("points to a different binary"));
        }
        _ => panic!("expected command failed"),
    }
}

// ─── < Helpers > ────────────────────────────────────────────────────

fn execution_config(timeout_seconds: u64, max_output_bytes: usize) -> ExecutionConfig {
    ExecutionConfig {
        timeout_seconds,
        max_output_bytes,
        inherit_environment: false,
        working_directory: None,
        environment: Vec::new(),
    }
}

fn console_config() -> ConsoleConfig {
    ConsoleConfig {
        default_command_policy: "deny".to_string(),
        allow_path_resolution: true,
        allowed_commands: Vec::new(),
        blocked_commands: Vec::new(),
        blocked_arguments: Vec::new(),
        ask_commands: Vec::new(),
        command_rules: Vec::new(),
    }
}

fn unique_temp_dir(name: &str) -> std::path::PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after unix epoch")
        .as_nanos();

    std::env::temp_dir().join(format!("{name}-{}-{timestamp}", std::process::id()))
}

fn find_executable_in_path(command_name: &str) -> Option<PathBuf> {
    std::env::var_os("PATH")?.as_encoded_bytes().is_empty().then_some(())?;

    for search_path in std::env::split_paths(&std::env::var_os("PATH")?) {
        let candidate = search_path.join(command_name);

        if is_executable_file(&candidate) {
            return Some(candidate);
        }
    }

    None
}

#[cfg(unix)]
fn is_executable_file(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;

    let Ok(metadata) = path.metadata() else {
        return false;
    };

    metadata.is_file() && metadata.permissions().mode() & 0o111 != 0
}

#[cfg(not(unix))]
fn is_executable_file(path: &Path) -> bool {
    path.is_file()
}
