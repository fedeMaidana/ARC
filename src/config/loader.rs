// ─── < Imports > ────────────────────────────────────────────────────

use std::env;
use std::path::PathBuf;

use super::validation::validate;
use super::{
    ActionsConfig, AgentSourceConfig, AgentsConfig, AuditConfig, Config, ConfigError, ConsoleCommandRule, ConsoleConfig, ExecutionConfig,
    HttpConfig, PolicyConfig, RegoPolicyConfig, ResourcesConfig, runtime_config_source_path,
};

// ─── < Public Functions > ───────────────────────────────────────────

pub fn load_runtime_config() -> Result<Config, ConfigError> {
    let config = Config {
        config_version: 1,
        policy: runtime_policy_config()?,
        agents: runtime_agents_config()?,
        actions: runtime_actions_config(),
        resources: runtime_resources_config(),
        http: runtime_http_config(),
        console: runtime_console_config(),
        audit: runtime_audit_config()?,
        execution: runtime_execution_config()?,
    };

    validate(&config).map_err(|source| ConfigError::Validation { source })?;

    Ok(config)
}

pub fn load_from_default_locations() -> Result<(Config, PathBuf), ConfigError> {
    let config = load_runtime_config()?;

    Ok((config, runtime_config_source_path()))
}

// ─── < Runtime Config Builders > ────────────────────────────────────

fn runtime_policy_config() -> Result<PolicyConfig, ConfigError> {
    Ok(PolicyConfig {
        engine: env_string("ARC_POLICY_ENGINE", "native"),
        default_action: env_string("ARC_POLICY_DEFAULT_ACTION", "deny"),
        rego: RegoPolicyConfig {
            policy_path: env_string("ARC_REGO_POLICY_PATH", "~/.config/arc/policies.d"),
            entrypoint: env_string("ARC_REGO_ENTRYPOINT", "data.arc.decision"),
            timeout_seconds: env_u64("ARC_REGO_TIMEOUT_SECONDS", 2)?,
        },
    })
}

fn runtime_agents_config() -> Result<AgentsConfig, ConfigError> {
    let mut sources = vec![
        agent_source("cli", "ARC CLI", true, "local_cli", Some("Built-in human command-line usage")),
        agent_source("json_api", "ARC JSON API", true, "local_cli", Some("Built-in machine-readable JSON interface")),
        agent_source("opencode", "OpenCode", true, "local_agent", Some("OpenCode custom bash tool routed through ARC")),
    ];

    sources.extend(agent_sources_from_environment()?);

    Ok(AgentsConfig {
        allow_unknown_sources: env_bool("ARC_AGENTS_ALLOW_UNKNOWN_SOURCES", true)?,
        sources,
    })
}

fn runtime_actions_config() -> ActionsConfig {
    ActionsConfig {
        allowed: strings(&["list_files", "read_file", "http_get", "run"]),
        blocked: strings(&["delete_file", "write_file", "run_shell"]),
        need_resource: strings(&["read_file", "http_get", "run"]),
        ask: Vec::new(),
    }
}

fn runtime_resources_config() -> ResourcesConfig {
    ResourcesConfig {
        protected: strings(&[".env", "id_rsa", "id_ed25519", "secrets.txt", "credentials.json", "Cargo.lock"]),
        blocked_path_prefixes: strings(&["/etc/", "/root/", "/var/run/", "/proc/", "/sys/", "/dev/", "../"]),
    }
}

fn runtime_http_config() -> HttpConfig {
    HttpConfig {
        allowed_schemes: strings(&["https"]),
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
        blocked_targets: Vec::new(),
    }
}

fn runtime_console_config() -> ConsoleConfig {
    ConsoleConfig {
        default_command_policy: "deny".to_string(),
        allow_path_resolution: true,
        allowed_commands: Vec::new(),
        blocked_commands: strings(&[
            "rm",
            "sudo",
            "su",
            "sh",
            "bash",
            "zsh",
            "fish",
            "chmod",
            "chown",
            "curl",
            "wget",
            "ssh",
            "scp",
            "nc",
            "ncat",
            "dd",
            "mkfs",
            "mount",
            "umount",
            "systemctl",
            "kill",
            "pkill",
        ]),
        blocked_arguments: strings(&["-rf", "--no-preserve-root", "/", "/etc", "/root", "..", "~"]),
        ask_commands: Vec::new(),
        command_rules: runtime_command_rules(),
    }
}

fn runtime_audit_config() -> Result<AuditConfig, ConfigError> {
    Ok(AuditConfig {
        enabled: env_bool("ARC_AUDIT_ENABLED", true)?,
        path: env_string("ARC_AUDIT_PATH", "~/.local/share/arc/audit.log"),
    })
}

fn runtime_execution_config() -> Result<ExecutionConfig, ConfigError> {
    Ok(ExecutionConfig {
        timeout_seconds: env_u64("ARC_EXECUTION_TIMEOUT_SECONDS", 10)?,
        max_output_bytes: env_usize("ARC_EXECUTION_MAX_OUTPUT_BYTES", 100_000)?,
        inherit_environment: env_bool("ARC_EXECUTION_INHERIT_ENVIRONMENT", false)?,
        working_directory: env_optional_string("ARC_EXECUTION_WORKING_DIRECTORY"),
        environment: Vec::new(),
    })
}

// ─── < Runtime Command Rules > ──────────────────────────────────────

fn runtime_command_rules() -> Vec<ConsoleCommandRule> {
    vec![
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
        ConsoleCommandRule {
            name: "git".to_string(),
            mode: "allow".to_string(),
            risk: None,
            allowed_paths: Vec::new(),
            allowed_subcommands: strings(&["status", "diff", "log", "show", "branch"]),
            blocked_subcommands: strings(&["push", "credential", "remote", "config"]),
            ask_subcommands: strings(&["add", "commit"]),
            blocked_arguments: strings(&["--upload-pack", "--receive-pack"]),
            ask_arguments: Vec::new(),
        },
        allow_command("rg"),
        allow_command("ls"),
        allow_command("pwd"),
        allow_command("cat"),
        allow_command("echo"),
        allow_command("whoami"),
        allow_command("date"),
        deny_command("rm"),
        deny_command("sudo"),
        deny_command("su"),
        deny_command("sh"),
        deny_command("bash"),
        deny_command("zsh"),
        deny_command("fish"),
        deny_command("chmod"),
        deny_command("chown"),
        deny_command("curl"),
        deny_command("wget"),
        deny_command("ssh"),
        deny_command("scp"),
        deny_command("nc"),
        deny_command("ncat"),
        deny_command("dd"),
        deny_command("mkfs"),
        deny_command("mount"),
        deny_command("umount"),
        deny_command("systemctl"),
        deny_command("kill"),
        deny_command("pkill"),
    ]
}

// ─── < Environment Parsing > ────────────────────────────────────────

fn env_string(name: &str, default: &str) -> String {
    env::var(name)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| default.to_string())
}

fn env_optional_string(name: &str) -> Option<String> {
    env::var(name)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn env_bool(name: &str, default: bool) -> Result<bool, ConfigError> {
    let Some(value) = env_optional_string(name) else {
        return Ok(default);
    };

    parse_bool(name, &value)
}

fn env_u64(name: &str, default: u64) -> Result<u64, ConfigError> {
    let Some(value) = env_optional_string(name) else {
        return Ok(default);
    };

    value.parse::<u64>().map_err(|_| ConfigError::InvalidEnvironmentValue {
        name: name.to_string(),
        value,
        expected: "an unsigned integer",
    })
}

fn env_usize(name: &str, default: usize) -> Result<usize, ConfigError> {
    let Some(value) = env_optional_string(name) else {
        return Ok(default);
    };

    value.parse::<usize>().map_err(|_| ConfigError::InvalidEnvironmentValue {
        name: name.to_string(),
        value,
        expected: "an unsigned integer",
    })
}

fn parse_bool(name: &str, value: &str) -> Result<bool, ConfigError> {
    match value {
        "true" | "1" | "yes" | "on" => Ok(true),
        "false" | "0" | "no" | "off" => Ok(false),
        _ => Err(ConfigError::InvalidEnvironmentValue {
            name: name.to_string(),
            value: value.to_string(),
            expected: "one of: true, false, 1, 0, yes, no, on, off",
        }),
    }
}

fn agent_sources_from_environment() -> Result<Vec<AgentSourceConfig>, ConfigError> {
    let Some(value) = env_optional_string("ARC_AGENT_SOURCES") else {
        return Ok(Vec::new());
    };

    value
        .split(';')
        .filter(|entry| !entry.trim().is_empty())
        .map(parse_agent_source)
        .collect()
}

fn parse_agent_source(entry: &str) -> Result<AgentSourceConfig, ConfigError> {
    let parts: Vec<&str> = entry.split('|').map(str::trim).collect();

    if parts.len() < 3 || parts.len() > 5 {
        return Err(ConfigError::InvalidEnvironmentValue {
            name: "ARC_AGENT_SOURCES".to_string(),
            value: entry.to_string(),
            expected: "entries like id|display_name|kind|enabled|description separated by ';'",
        });
    }

    let enabled = if let Some(value) = parts.get(3).filter(|value| !value.is_empty()) {
        parse_bool("ARC_AGENT_SOURCES", value)?
    } else {
        true
    };

    Ok(agent_source(parts[0], parts[1], enabled, parts[2], parts.get(4).copied().filter(|description| !description.is_empty())))
}

// ─── < Helpers > ────────────────────────────────────────────────────

fn allow_command(name: &str) -> ConsoleCommandRule {
    command_rule(name, "allow")
}

fn deny_command(name: &str) -> ConsoleCommandRule {
    command_rule(name, "deny")
}

fn command_rule(name: &str, mode: &str) -> ConsoleCommandRule {
    ConsoleCommandRule {
        name: name.to_string(),
        mode: mode.to_string(),
        risk: None,
        allowed_paths: Vec::new(),
        allowed_subcommands: Vec::new(),
        blocked_subcommands: Vec::new(),
        ask_subcommands: Vec::new(),
        blocked_arguments: Vec::new(),
        ask_arguments: Vec::new(),
    }
}

fn agent_source(id: &str, display_name: &str, enabled: bool, kind: &str, description: Option<&str>) -> AgentSourceConfig {
    AgentSourceConfig {
        id: id.to_string(),
        display_name: display_name.to_string(),
        enabled,
        kind: kind.to_string(),
        description: description.map(ToString::to_string),
    }
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| value.to_string()).collect()
}
