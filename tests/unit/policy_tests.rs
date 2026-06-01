// ─── < Imports > ────────────────────────────────────────────────────

use arc::config::{ActionsConfig, AuditConfig, Config, ConsoleConfig, ExecutionConfig, HttpConfig, ResourcesConfig};
use arc::decision::{Decision, DecisionReason, DecisionStatus, RiskLevel};
use arc::policy;
use arc::request::{Request, RequestMode};

// ─── < Tests: Action Rules > ────────────────────────────────────────

#[test]
fn allows_configured_action_without_resource() {
    let config = default_config();
    let request = request("list_files", &[]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Allow, DecisionReason::ActionAllowed, RiskLevel::Low);
}

#[test]
fn denies_blocked_action() {
    let config = default_config();
    let request = request("delete_file", &["README.md"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::ActionBlocked, RiskLevel::Critical);
}

#[test]
fn denies_action_that_requires_missing_resource() {
    let config = default_config();
    let request = request("read_file", &[]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::ResourceRequired, RiskLevel::Medium);
}

#[test]
fn asks_for_configured_action_approval() {
    let mut config = default_config();

    config.actions.allowed.push("publish".to_string());
    config.actions.ask.push("publish".to_string());

    let request = request("publish", &["release"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Ask, DecisionReason::ActionRequiresApproval, RiskLevel::Low);
}

#[test]
fn denies_unconfigured_action() {
    let config = default_config();
    let request = request("teleport", &[]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::ActionNotConfigured, RiskLevel::High);
}

// ─── < Tests: Console Rules > ───────────────────────────────────────

#[test]
fn allows_configured_console_command() {
    let config = default_config();
    let request = request("run", &["ls", "-la"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Allow, DecisionReason::ActionAllowed, RiskLevel::Low);
}

#[test]
fn denies_run_without_command_when_action_does_not_require_resource_first() {
    let mut config = default_config();

    config.actions.need_resource.clear();

    let request = request("run", &[]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::ConsoleCommandRequired, RiskLevel::Medium);
}

#[test]
fn denies_blocked_console_command() {
    let config = default_config();
    let request = request("run", &["rm", "-rf", "/"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::ConsoleCommandBlocked, RiskLevel::Critical);
}

#[test]
fn denies_console_command_not_in_allowlist() {
    let config = default_config();
    let request = request("run", &["python", "script.py"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::ConsoleCommandNotAllowed, RiskLevel::High);
}

#[test]
fn asks_for_console_command_approval() {
    let mut config = default_config();

    config.console.ask_commands.push("cat".to_string());

    let request = request("run", &["cat", "README.md"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Ask, DecisionReason::ConsoleCommandRequiresApproval, RiskLevel::Medium);
}

#[test]
fn denies_console_argument_that_is_explicitly_blocked() {
    let config = default_config();
    let request = request("run", &["cat", "/"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::ConsoleArgumentBlocked, RiskLevel::Critical);
}

#[test]
fn denies_console_argument_that_targets_protected_resource() {
    let config = default_config();
    let request = request("run", &["cat", ".env"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::ConsoleArgumentBlocked, RiskLevel::Critical);
}

#[test]
fn denies_console_argument_that_targets_protected_resource_after_normalization() {
    let config = default_config();
    let request = request("run", &["cat", "config/../.env"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::ConsoleArgumentBlocked, RiskLevel::Critical);
}

#[test]
fn denies_console_argument_that_targets_blocked_path_after_normalization() {
    let config = default_config();
    let request = request("run", &["cat", "/tmp/../etc/passwd"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::ConsoleArgumentBlocked, RiskLevel::Critical);
}

// ─── < Tests: Resource Rules > ──────────────────────────────────────

#[test]
fn denies_protected_resource() {
    let config = default_config();
    let request = request("read_file", &[".env"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::ResourceProtected, RiskLevel::Critical);
}

#[test]
fn denies_protected_resource_nested_in_folder() {
    let config = default_config();
    let request = request("read_file", &["config/.env"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::ResourceProtected, RiskLevel::Critical);
}

#[test]
fn denies_protected_resource_with_current_dir_segment() {
    let config = default_config();
    let request = request("read_file", &["./.env"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::ResourceProtected, RiskLevel::Critical);
}

#[test]
fn denies_protected_resource_after_parent_segment_normalization() {
    let config = default_config();
    let request = request("read_file", &["config/../.env"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::ResourceProtected, RiskLevel::Critical);
}

#[test]
fn denies_blocked_path_prefix() {
    let config = default_config();
    let request = request("read_file", &["/etc/passwd"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::PathBlocked, RiskLevel::Critical);
}

#[test]
fn denies_blocked_path_after_parent_segment_normalization() {
    let config = default_config();
    let request = request("read_file", &["/tmp/../etc/passwd"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::PathBlocked, RiskLevel::Critical);
}

#[test]
fn denies_parent_directory_traversal_after_normalization() {
    let config = default_config();
    let request = request("read_file", &["config/../../secrets.txt"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::ResourceProtected, RiskLevel::Critical);
}

// ─── < Tests: HTTP Rules > ──────────────────────────────────────────

#[test]
fn denies_invalid_http_url() {
    let config = default_config();
    let request = request("http_get", &["example.com"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::InvalidHttpUrl, RiskLevel::Medium);
}

#[test]
fn denies_unsupported_http_scheme() {
    let config = default_config();
    let request = request("http_get", &["file:///etc/passwd"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::InvalidHttpUrl, RiskLevel::Medium);
}

#[test]
fn denies_blocked_http_target() {
    let config = default_config();
    let request = request("http_get", &["http://localhost:3000"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::HttpTargetBlocked, RiskLevel::High);
}

#[test]
fn denies_blocked_ipv6_loopback_http_target() {
    let config = default_config();
    let request = request("http_get", &["http://[::1]:3000"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::HttpTargetBlocked, RiskLevel::High);
}

#[test]
fn denies_blocked_unspecified_ipv4_http_target() {
    let config = default_config();
    let request = request("http_get", &["http://0.0.0.0:3000"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Deny, DecisionReason::HttpTargetBlocked, RiskLevel::High);
}

#[test]
fn allows_domain_that_only_starts_with_blocked_host_text() {
    let config = default_config();
    let request = request("http_get", &["http://localhost.evil.com"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Allow, DecisionReason::ActionAllowed, RiskLevel::Medium);
}

#[test]
fn allows_external_http_url_with_medium_risk() {
    let config = default_config();
    let request = request("http_get", &["https://example.com"]);

    let decision = policy::decide(&request, &config);

    assert_decision(decision, DecisionStatus::Allow, DecisionReason::ActionAllowed, RiskLevel::Medium);
}

// ─── < Helpers > ────────────────────────────────────────────────────

fn default_config() -> Config {
    Config {
        actions: ActionsConfig {
            allowed: strings(&["list_files", "read_file", "http_get", "run"]),
            blocked: strings(&["delete_file", "write_file", "run_shell"]),
            need_resource: strings(&["read_file", "http_get", "run"]),
            ask: Vec::new(),
        },
        resources: ResourcesConfig {
            protected: strings(&[".env", "id_rsa", "secrets.txt"]),
            blocked_path_prefixes: strings(&["/etc/", "/root/", "../"]),
        },
        http: HttpConfig {
            blocked_targets: strings(&[
                "http://localhost",
                "https://localhost",
                "http://127.0.0.1",
                "https://127.0.0.1",
                "http://0.0.0.0",
                "https://0.0.0.0",
                "http://[::1]",
                "https://[::1]",
            ]),
        },
        console: ConsoleConfig {
            allowed_commands: strings(&["cargo", "git", "rg", "ls", "pwd", "cat", "echo"]),
            blocked_commands: strings(&["rm", "sudo", "su", "sh", "bash"]),
            blocked_arguments: strings(&["-rf", "--no-preserve-root", "/", "/etc", "/root", "..", "~"]),
            ask_commands: Vec::new(),
        },
        audit: AuditConfig::default(),
        execution: ExecutionConfig::default(),
    }
}

fn request(action: &str, command_parts: &[&str]) -> Request {
    Request::new(RequestMode::Execute, action.to_string(), strings(command_parts))
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| value.to_string()).collect()
}

fn assert_decision(decision: Decision, expected_status: DecisionStatus, expected_reason: DecisionReason, expected_risk: RiskLevel) {
    assert_eq!(decision.status, expected_status);
    assert_eq!(decision.reason, expected_reason);
    assert_eq!(decision.risk, expected_risk);
}
