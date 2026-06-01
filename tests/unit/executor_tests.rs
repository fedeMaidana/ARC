// ─── < Imports > ────────────────────────────────────────────────────

use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use arc::config::{ExecutionConfig, ExecutionEnvironmentVariable};
use arc::decision::{Decision, DecisionReason};
use arc::executor::{self, ExecutionReport};
use arc::request::{Request, RequestMode};

// ─── < Tests > ──────────────────────────────────────────────────────

#[test]
fn execute_captures_command_output() {
    let request = Request::new(RequestMode::Execute, "run".to_string(), vec!["echo".to_string(), "hola".to_string()]);

    let decision = Decision::allow(DecisionReason::ActionAllowed);
    let execution_config = execution_config(10, 100_000);

    let report = executor::execute(&request, &decision, &execution_config);

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

    let report = executor::execute(&request, &decision, &execution_config);

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

    let report = executor::execute(&request, &decision, &execution_config);

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

    execution_config.working_directory = Some(working_directory.to_string_lossy().to_string());

    let report = executor::execute(&request, &decision, &execution_config);

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
    let request = Request::new(
        RequestMode::Execute,
        "run".to_string(),
        vec![
            "sh".to_string(),
            "-c".to_string(),
            "printf \"$ARC_EXECUTOR_TEST_VALUE\"".to_string(),
        ],
    );

    let decision = Decision::allow(DecisionReason::ActionAllowed);
    let mut execution_config = execution_config(10, 100_000);

    execution_config.environment.push(ExecutionEnvironmentVariable {
        name: "ARC_EXECUTOR_TEST_VALUE".to_string(),
        value: "configured".to_string(),
    });

    let report = executor::execute(&request, &decision, &execution_config);

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

    let report = executor::execute(&request, &decision, &execution_config);

    match report {
        ExecutionReport::CommandFailed(error) => {
            assert_eq!(error.command_line, "__arc_command_that_should_not_exist__");
            assert!(error.details.contains("command not found in PATH"));
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

fn unique_temp_dir(name: &str) -> std::path::PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after unix epoch")
        .as_nanos();

    std::env::temp_dir().join(format!("{name}-{}-{timestamp}", std::process::id()))
}
