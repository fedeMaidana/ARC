// ─── < Imports > ────────────────────────────────────────────────────

use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use arc::audit::{self, AuditEvent, AuditExecution};
use arc::config::AuditConfig;
use arc::decision::{Decision, DecisionReason};
use arc::executor::{CommandExecutionError, CommandExecutionReport, ExecutionReport};
use arc::request::{Request, RequestMode};

// ─── < Tests > ──────────────────────────────────────────────────────

#[test]
fn audit_event_contains_decision_data() {
    let request = Request::new(RequestMode::Check, "run".to_string(), vec!["ls".to_string(), "-la".to_string()]);

    let decision = Decision::allow(DecisionReason::ActionAllowed);
    let execution_report = ExecutionReport::CheckMode { allowed: true };

    let event = AuditEvent::from_parts("test", &request, &decision, &execution_report);

    assert_eq!(event.mode, "check");
    assert_eq!(event.action, "run");
    assert_eq!(event.resource, Some("ls -la".to_string()));
    assert_eq!(event.decision, "allow");
    assert_eq!(event.reason, "request matches an allowed policy");
    assert_eq!(event.risk, "low");
    assert!(!event.executed);
    assert_eq!(event.exit_code, 0);
    assert_eq!(event.source, "test");
}

#[test]
fn audit_event_redacts_sensitive_resource_values() {
    let request = Request::new(RequestMode::Check, "run".to_string(), vec!["echo".to_string(), "api_key=super-secret-value".to_string()]);

    let decision = Decision::allow(DecisionReason::ActionAllowed);
    let execution_report = ExecutionReport::CheckMode { allowed: true };

    let event = AuditEvent::from_parts("test", &request, &decision, &execution_report);

    assert_eq!(event.resource, Some("echo api_key=[redacted]".to_string()));
}

#[test]
fn audit_event_redacts_sensitive_command_line_values() {
    let request = Request::new(RequestMode::Execute, "run".to_string(), vec!["echo".to_string()]);

    let decision = Decision::allow(DecisionReason::ActionAllowed);
    let execution_report = ExecutionReport::CommandFinished(CommandExecutionReport {
        command_line: "echo token=super-secret-value".to_string(),
        status: "exit status: 0".to_string(),
        success: true,
        exit_code: 0,
        stdout: "ok".to_string(),
        stderr: String::new(),
        stdout_truncated: false,
        stderr_truncated: false,
    });

    let event = AuditEvent::from_parts("test", &request, &decision, &execution_report);

    match event.execution {
        AuditExecution::CommandFinished { command_line, .. } => {
            assert_eq!(command_line, "echo token=[redacted]");
        }
        _ => panic!("expected command finished audit execution"),
    }
}

#[test]
fn audit_event_redacts_sensitive_error_details() {
    let request = Request::new(RequestMode::Execute, "run".to_string(), vec!["custom".to_string()]);

    let decision = Decision::allow(DecisionReason::ActionAllowed);
    let execution_report = ExecutionReport::CommandFailed(CommandExecutionError {
        command_line: "custom --password=hunter2".to_string(),
        details: "failed with authorization=BearerSecret".to_string(),
    });

    let event = AuditEvent::from_parts("test", &request, &decision, &execution_report);

    match event.execution {
        AuditExecution::CommandFailed { command_line, details } => {
            assert_eq!(command_line, "custom --password=[redacted]");
            assert_eq!(details, "failed with authorization=[redacted]");
        }
        _ => panic!("expected command failed audit execution"),
    }
}

#[test]
fn audit_event_truncates_large_fields() {
    let request = Request::new(RequestMode::Check, "run".to_string(), vec!["echo".to_string(), "x".repeat(2_000)]);

    let decision = Decision::allow(DecisionReason::ActionAllowed);
    let execution_report = ExecutionReport::CheckMode { allowed: true };

    let event = AuditEvent::from_parts("test", &request, &decision, &execution_report);

    let resource = event.resource.expect("resource should be present");

    assert!(resource.chars().count() <= 512);
    assert!(resource.ends_with("…[truncated]"));
}

#[test]
fn record_event_writes_json_line() {
    let audit_path = temp_audit_path("record_event_writes_json_line");

    let config = AuditConfig {
        enabled: true,
        path: audit_path.to_string_lossy().to_string(),
    };

    let request = Request::new(RequestMode::Check, "run".to_string(), vec!["ls".to_string()]);

    let decision = Decision::allow(DecisionReason::ActionAllowed);
    let execution_report = ExecutionReport::CheckMode { allowed: true };
    let event = AuditEvent::from_parts("test", &request, &decision, &execution_report);

    audit::record_event(&config, &event).expect("audit event should be written");

    let content = fs::read_to_string(&audit_path).expect("audit log should exist");
    let lines: Vec<&str> = content.lines().collect();

    assert_eq!(lines.len(), 1);

    let json: serde_json::Value = serde_json::from_str(lines[0]).expect("audit line should be valid json");

    assert_eq!(json["mode"], "check");
    assert_eq!(json["action"], "run");
    assert_eq!(json["decision"], "allow");
    assert_eq!(json["reason"], "request matches an allowed policy");
    assert_eq!(json["risk"], "low");
    assert_eq!(json["source"], "test");

    let _ = fs::remove_file(audit_path);
}

#[cfg(unix)]
#[test]
fn record_event_restricts_audit_log_permissions() {
    use std::os::unix::fs::PermissionsExt;

    let audit_path = temp_audit_path("record_event_restricts_audit_log_permissions");

    let config = AuditConfig {
        enabled: true,
        path: audit_path.to_string_lossy().to_string(),
    };

    let request = Request::new(RequestMode::Check, "run".to_string(), vec!["ls".to_string()]);
    let decision = Decision::allow(DecisionReason::ActionAllowed);
    let execution_report = ExecutionReport::CheckMode { allowed: true };
    let event = AuditEvent::from_parts("test", &request, &decision, &execution_report);

    audit::record_event(&config, &event).expect("audit event should be written");

    let permissions = fs::metadata(&audit_path)
        .expect("audit log metadata should be readable")
        .permissions()
        .mode()
        & 0o777;

    assert_eq!(permissions, 0o600);

    let _ = fs::remove_file(audit_path);
}

#[test]
fn disabled_audit_does_not_create_log_file() {
    let audit_path = temp_audit_path("disabled_audit_does_not_create_log_file");

    let config = AuditConfig {
        enabled: false,
        path: audit_path.to_string_lossy().to_string(),
    };

    let request = Request::new(RequestMode::Check, "run".to_string(), vec!["ls".to_string()]);

    let decision = Decision::allow(DecisionReason::ActionAllowed);
    let execution_report = ExecutionReport::CheckMode { allowed: true };
    let event = AuditEvent::from_parts("test", &request, &decision, &execution_report);

    audit::record_event(&config, &event).expect("disabled audit should not fail");

    assert!(!audit_path.exists());
}

// ─── < Helpers > ────────────────────────────────────────────────────

fn temp_audit_path(test_name: &str) -> PathBuf {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos();

    std::env::temp_dir().join(format!("arc-{test_name}-{timestamp}.audit.log"))
}
