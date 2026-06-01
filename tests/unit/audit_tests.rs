// ─── < Imports > ────────────────────────────────────────────────────

use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use arc::audit::{self, AuditEvent};
use arc::config::AuditConfig;
use arc::decision::{Decision, DecisionReason};
use arc::executor::ExecutionReport;
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
    assert_eq!(event.reason, "action is allowed");
    assert_eq!(event.risk, "low");
    assert!(!event.executed);
    assert_eq!(event.exit_code, 0);
    assert_eq!(event.source, "test");
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
    assert_eq!(json["reason"], "action is allowed");
    assert_eq!(json["risk"], "low");
    assert_eq!(json["source"], "test");

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
