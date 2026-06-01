// ─── < Imports > ────────────────────────────────────────────────────

use arc::request::{Request, RequestMode};

// ─── < Tests > ──────────────────────────────────────────────────────

#[test]
fn request_builds_resource_from_command_parts() {
    let request = Request::new(RequestMode::Execute, "run".to_string(), vec!["echo".to_string(), "hello".to_string()]);

    assert_eq!(request.action, "run");
    assert_eq!(request.resource, "echo hello");
    assert!(request.has_resource());
}

#[test]
fn request_returns_command_name() {
    let request = Request::new(RequestMode::Execute, "run".to_string(), vec!["echo".to_string(), "hello".to_string()]);

    assert_eq!(request.command_name(), Some("echo"));
}

#[test]
fn request_returns_command_args() {
    let request = Request::new(RequestMode::Execute, "run".to_string(), vec!["echo".to_string(), "hello".to_string(), "world".to_string()]);

    assert_eq!(request.command_args(), &["hello".to_string(), "world".to_string()]);
}

#[test]
fn request_detects_check_mode() {
    let request = Request::new(RequestMode::Check, "run".to_string(), vec!["ls".to_string()]);

    assert!(request.is_check_mode());
}
