// ─── < Imports > ────────────────────────────────────────────────────

use arc::config::{
    ActionsConfig, AuditConfig, Config, ConsoleCommandRule, ConsoleConfig, ExecutionConfig, HttpConfig, PolicyConfig, ResourcesConfig,
};
use arc::decision::{Decision, DecisionReason, DecisionStatus, RiskLevel};
use arc::request::{Request, RequestMode};

// ─── < Public Helpers > ─────────────────────────────────────────────

pub fn default_config() -> Config {
    Config {
        config_version: 1,
        policy: PolicyConfig {
            default_action: "deny".to_string(),
        },
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
            allowed_schemes: strings(&["http", "https"]),
            block_localhost: true,
            block_private_networks: true,
            block_link_local: true,
            block_metadata_services: true,
            blocked_hosts: strings(&["localhost"]),
            blocked_cidrs: strings(&[
                "0.0.0.0/8",
                "10.0.0.0/8",
                "127.0.0.0/8",
                "169.254.0.0/16",
                "172.16.0.0/12",
                "192.168.0.0/16",
                "::1/128",
                "fc00::/7",
                "fe80::/10",
            ]),
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
            default_command_policy: "deny".to_string(),
            allow_path_resolution: true,
            allowed_commands: strings(&["cargo", "git", "rg", "ls", "pwd", "cat", "echo"]),
            blocked_commands: strings(&["rm", "sudo", "su", "sh", "bash"]),
            blocked_arguments: strings(&["-rf", "--no-preserve-root", "/", "/etc", "/root", "..", "~", "--danger"]),
            ask_commands: Vec::new(),
            command_rules: command_rules(),
        },
        audit: AuditConfig::default(),
        execution: ExecutionConfig::default(),
    }
}

pub fn request(action: &str, command_parts: &[&str]) -> Request {
    Request::new(RequestMode::Execute, action.to_string(), strings(command_parts))
}

pub fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| value.to_string()).collect()
}

pub fn assert_decision(decision: Decision, expected_status: DecisionStatus, expected_reason: DecisionReason, expected_risk: RiskLevel) {
    assert_eq!(decision.status, expected_status);
    assert_eq!(decision.reason, expected_reason);
    assert_eq!(decision.risk, expected_risk);
}

// ─── < Private Helpers > ────────────────────────────────────────────

fn command_rules() -> Vec<ConsoleCommandRule> {
    vec![
        ConsoleCommandRule {
            name: "git".to_string(),
            mode: "allow".to_string(),
            risk: None,
            allowed_paths: Vec::new(),
            allowed_subcommands: strings(&["status", "diff", "log", "show", "branch"]),
            blocked_subcommands: strings(&["push", "credential", "remote"]),
            ask_subcommands: strings(&["add", "commit"]),
            blocked_arguments: strings(&["--upload-pack", "--receive-pack"]),
            ask_arguments: Vec::new(),
        },
        ConsoleCommandRule {
            name: "cargo".to_string(),
            mode: "allow".to_string(),
            risk: None,
            allowed_paths: Vec::new(),
            allowed_subcommands: strings(&["build", "check", "fmt", "test", "clippy", "nextest"]),
            blocked_subcommands: strings(&["publish", "install", "login", "owner"]),
            ask_subcommands: strings(&["run"]),
            blocked_arguments: Vec::new(),
            ask_arguments: strings(&["--release"]),
        },
    ]
}
