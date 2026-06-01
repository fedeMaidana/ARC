// ─── < Imports > ────────────────────────────────────────────────────

use arc::config::ExecutionConfig;
use arc::decision::{Decision, DecisionReason};
use arc::executor::{self, ExecutionReport};
use arc::request::{Request, RequestMode};

// ─── < Tests > ──────────────────────────────────────────────────────

#[test]
fn execute_captures_command_output() {
    let request = Request::new(RequestMode::Execute, "run".to_string(), vec!["echo".to_string(), "hola".to_string()]);

    let decision = Decision::allow(DecisionReason::ActionAllowed);
    let execution_config = ExecutionConfig {
        timeout_seconds: 10,
        max_output_bytes: 100_000,
    };

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
    let execution_config = ExecutionConfig {
        timeout_seconds: 10,
        max_output_bytes: 4,
    };

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
    let request = Request::new(RequestMode::Execute, "run".to_string(), vec!["sh".to_string(), "-c".to_string(), "sleep 2".to_string()]);

    let decision = Decision::allow(DecisionReason::ActionAllowed);
    let execution_config = ExecutionConfig {
        timeout_seconds: 1,
        max_output_bytes: 100_000,
    };

    let report = executor::execute(&request, &decision, &execution_config);

    match report {
        ExecutionReport::CommandTimedOut(report) => {
            assert_eq!(report.command_line, "sh -c sleep 2");
            assert_eq!(report.timeout_seconds, 1);
        }
        _ => panic!("expected command timeout"),
    }
}
