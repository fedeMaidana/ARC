// ─── < Imports > ────────────────────────────────────────────────────

use arc::config::{ActionsConfig, AuditConfig, Config, ConsoleConfig, ExecutionConfig, HttpConfig, ResourcesConfig};

// ─── < Public Helpers > ─────────────────────────────────────────────

pub fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|item| item.to_string()).collect()
}

pub fn test_config() -> Config {
    Config {
        actions: ActionsConfig {
            allowed: strings(&["run", "http_get"]),
            blocked: strings(&["delete"]),
            need_resource: strings(&["http_get"]),
            ask: Vec::new(),
        },
        resources: ResourcesConfig {
            protected: strings(&["Cargo.toml", ".env"]),
            blocked_path_prefixes: strings(&["/etc", "/root"]),
        },
        http: HttpConfig {
            blocked_targets: strings(&["http://localhost", "https://blocked.test"]),
        },
        console: ConsoleConfig {
            allowed_commands: strings(&["echo", "ls"]),
            blocked_commands: strings(&["rm"]),
            blocked_arguments: strings(&["--danger"]),
            ask_commands: Vec::new(),
        },
        audit: AuditConfig {
            enabled: false,
            path: "~/.local/share/arc/audit.log".to_string(),
        },
        execution: ExecutionConfig {
            timeout_seconds: 10,
            max_output_bytes: 100_000,
        },
    }
}
