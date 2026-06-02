// ─── < Imports > ────────────────────────────────────────────────────

use arc::config::{
    ActionsConfig, AgentsConfig, AuditConfig, Config, ConsoleConfig, ExecutionConfig, HttpConfig, PolicyConfig, ResourcesConfig,
};

// ─── < Public Helpers > ─────────────────────────────────────────────

pub fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|item| item.to_string()).collect()
}

pub fn test_config() -> Config {
    Config {
        config_version: 1,
        policy: PolicyConfig {
            default_action: "deny".to_string(),
        },
        agents: AgentsConfig::default(),
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
            blocked_targets: strings(&["http://localhost", "https://blocked.test"]),
        },
        console: ConsoleConfig {
            default_command_policy: "deny".to_string(),
            allow_path_resolution: true,
            allowed_commands: strings(&["echo", "ls"]),
            blocked_commands: strings(&["rm"]),
            blocked_arguments: strings(&["--danger"]),
            ask_commands: Vec::new(),
            command_rules: Vec::new(),
        },
        audit: AuditConfig {
            enabled: false,
            path: "~/.local/share/arc/audit.log".to_string(),
        },
        execution: ExecutionConfig {
            timeout_seconds: 10,
            max_output_bytes: 100_000,
            inherit_environment: false,
            working_directory: None,
            environment: Vec::new(),
        },
    }
}
