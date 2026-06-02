// ─── < Imports > ────────────────────────────────────────────────────

use arc::config::{
    ActionsConfig, AuditConfig, Config, ConfigValidationError, ConsoleCommandRule, ConsoleConfig, ExecutionConfig, HttpConfig,
    PolicyConfig, ResourcesConfig, validate,
};

// ─── < Tests > ──────────────────────────────────────────────────────

#[test]
fn accepts_valid_config() {
    let config = default_config();

    assert!(validate(&config).is_ok());
}

#[test]
fn rejects_unsupported_config_version() {
    let mut config = default_config();

    config.config_version = 2;

    assert_validation_error(&config, &["config_version", "unsupported config version 2"]);
}

#[test]
fn rejects_unsupported_policy_values() {
    let mut config = default_config();

    config.policy.default_action = "permit".to_string();
    config.console.default_command_policy = "maybe".to_string();

    assert_validation_error(
        &config,
        &[
            "policy.default_action: unsupported value \"permit\"",
            "console.default_command_policy: unsupported value \"maybe\"",
        ],
    );
}

#[test]
fn rejects_unsupported_console_command_mode() {
    let mut config = default_config();

    config.console.command_rules[0].mode = "alow".to_string();

    assert_validation_error(
        &config,
        &[
            "console.commands[git].mode",
            "unsupported value \"alow\"",
            "expected one of: allow, ask, deny",
        ],
    );
}

#[test]
fn rejects_duplicate_console_commands() {
    let mut config = default_config();

    config.console.command_rules.push(command_rule("git", "deny", &[]));

    assert_validation_error(&config, &["console.commands[git]", "duplicate command \"git\""]);
}

#[test]
fn rejects_relative_allowed_paths() {
    let mut config = default_config();

    config.console.command_rules[0].allowed_paths = strings(&["relative/path"]);

    assert_validation_error(&config, &["console.commands[git].allowed_paths[0]", "path must be absolute: \"relative/path\""]);
}

#[test]
fn rejects_missing_allowed_paths_when_path_resolution_is_disabled() {
    let mut config = default_config();

    config.console.allow_path_resolution = false;

    assert_validation_error(
        &config,
        &[
            "console.commands[git].allowed_paths",
            "must not be empty when console.allow_path_resolution=false",
        ],
    );
}

#[test]
fn rejects_legacy_command_allowlists_when_path_resolution_is_disabled() {
    let mut config = default_config();

    config.console.allow_path_resolution = false;
    config.console.command_rules[0].allowed_paths = strings(&["/workspace/project"]);
    config.console.allowed_commands = strings(&["ls"]);
    config.console.ask_commands = strings(&["cat"]);

    assert_validation_error(
        &config,
        &[
            "console.allowed_commands",
            "console.ask_commands",
            "cannot be used when console.allow_path_resolution=false",
        ],
    );
}

#[test]
fn rejects_invalid_http_schemes_cidrs_and_url_hosts() {
    let mut config = default_config();

    config.http.allowed_schemes.push("ftp".to_string());
    config.http.blocked_cidrs.push("192.168/33".to_string());
    config.http.blocked_hosts.push("https://localhost".to_string());

    assert_validation_error(
        &config,
        &[
            "http.allowed_schemes[1]: unsupported value \"ftp\"",
            "http.blocked_cidrs[1]: invalid CIDR \"192.168/33\"",
            "http.blocked_hosts[1]: must be a hostname, not a URL/path: \"https://localhost\"",
        ],
    );
}

#[test]
fn rejects_empty_blocked_arguments_when_commands_can_be_allowed() {
    let mut config = default_config();

    config.console.blocked_arguments.clear();

    assert_validation_error(
        &config,
        &[
            "console.blocked_arguments",
            "must not be empty when commands can be allowed or asked",
        ],
    );
}

// ─── < Helpers > ────────────────────────────────────────────────────

fn default_config() -> Config {
    Config {
        config_version: 1,
        policy: PolicyConfig {
            default_action: "deny".to_string(),
        },
        actions: ActionsConfig {
            allowed: strings(&["run", "http_get"]),
            blocked: strings(&["delete_file"]),
            need_resource: strings(&["run", "http_get"]),
            ask: Vec::new(),
        },
        resources: ResourcesConfig {
            protected: strings(&[".env"]),
            blocked_path_prefixes: strings(&["/etc/"]),
        },
        http: HttpConfig {
            allowed_schemes: strings(&["https"]),
            block_localhost: true,
            block_private_networks: true,
            block_link_local: true,
            block_metadata_services: true,
            blocked_hosts: strings(&["localhost"]),
            blocked_cidrs: strings(&["127.0.0.0/8"]),
            blocked_targets: Vec::new(),
        },
        console: ConsoleConfig {
            default_command_policy: "deny".to_string(),
            allow_path_resolution: true,
            allowed_commands: Vec::new(),
            blocked_commands: strings(&["rm"]),
            blocked_arguments: strings(&["-rf"]),
            ask_commands: Vec::new(),
            command_rules: vec![command_rule("git", "allow", &[])],
        },
        audit: AuditConfig::default(),
        execution: ExecutionConfig::default(),
    }
}

fn command_rule(name: &str, mode: &str, allowed_paths: &[&str]) -> ConsoleCommandRule {
    ConsoleCommandRule {
        name: name.to_string(),
        mode: mode.to_string(),
        risk: None,
        allowed_paths: strings(allowed_paths),
        allowed_subcommands: Vec::new(),
        blocked_subcommands: Vec::new(),
        ask_subcommands: Vec::new(),
        blocked_arguments: Vec::new(),
        ask_arguments: Vec::new(),
    }
}

fn strings(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| value.to_string()).collect()
}

fn assert_validation_error(config: &Config, expected_parts: &[&str]) {
    let error = validate(config).expect_err("config should be rejected");
    let details = validation_details(&error);

    for expected_part in expected_parts {
        assert!(details.contains(expected_part), "expected validation error to contain {expected_part:?}\n\nactual:\n{details}");
    }
}

fn validation_details(error: &ConfigValidationError) -> String {
    error.issues().iter().map(ToString::to_string).collect::<Vec<_>>().join("\n")
}
