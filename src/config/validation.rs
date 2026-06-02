// ─── < Imports > ────────────────────────────────────────────────────

use std::collections::HashSet;
use std::fmt;
use std::path::Path;

use ipnet::IpNet;

use super::model::{Config, ConsoleCommandRule, ConsoleConfig};

// ─── < Constants > ──────────────────────────────────────────────────

const SUPPORTED_CONFIG_VERSION: u32 = 1;
const POLICY_ACTIONS: &[&str] = &["deny", "ask", "allow"];
const COMMAND_MODES: &[&str] = &["allow", "ask", "deny"];
const HTTP_SCHEMES: &[&str] = &["http", "https"];

// ─── < Structs > ────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigValidationIssue {
    field: String,
    message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigValidationError {
    issues: Vec<ConfigValidationIssue>,
}

#[derive(Debug, Default)]
struct ConfigValidator {
    issues: Vec<ConfigValidationIssue>,
}

// ─── < Public Functions > ───────────────────────────────────────────

pub fn validate(config: &Config) -> Result<(), ConfigValidationError> {
    let mut validator = ConfigValidator::default();

    validate_config_version(&mut validator, config);
    validate_policy(&mut validator, config);
    validate_console(&mut validator, &config.console);
    validate_http(&mut validator, config);

    validator.finish()
}

// ─── < Implementations > ────────────────────────────────────────────

impl ConfigValidationIssue {
    fn new(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
        }
    }

    pub fn field(&self) -> &str {
        &self.field
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for ConfigValidationIssue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.field, self.message)
    }
}

impl ConfigValidationError {
    fn new(issues: Vec<ConfigValidationIssue>) -> Self {
        Self { issues }
    }

    pub fn issues(&self) -> &[ConfigValidationIssue] {
        &self.issues
    }
}

impl fmt::Display for ConfigValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(formatter, "config validation failed:")?;

        for issue in &self.issues {
            writeln!(formatter, "  {issue}")?;
        }

        Ok(())
    }
}

impl std::error::Error for ConfigValidationError {}

impl ConfigValidator {
    fn issue(&mut self, field: impl Into<String>, message: impl Into<String>) {
        self.issues.push(ConfigValidationIssue::new(field, message));
    }

    fn finish(self) -> Result<(), ConfigValidationError> {
        if self.issues.is_empty() {
            Ok(())
        } else {
            Err(ConfigValidationError::new(self.issues))
        }
    }
}

// ─── < Validation Functions > ───────────────────────────────────────

fn validate_config_version(validator: &mut ConfigValidator, config: &Config) {
    if config.config_version != SUPPORTED_CONFIG_VERSION {
        validator.issue(
            "config_version",
            format!("unsupported config version {}; supported version: {}", config.config_version, SUPPORTED_CONFIG_VERSION),
        );
    }
}

fn validate_policy(validator: &mut ConfigValidator, config: &Config) {
    validate_supported_value(validator, "policy.default_action", &config.policy.default_action, POLICY_ACTIONS);
}

fn validate_console(validator: &mut ConfigValidator, console: &ConsoleConfig) {
    validate_supported_value(validator, "console.default_command_policy", &console.default_command_policy, POLICY_ACTIONS);

    validate_command_rules(validator, console);
    validate_command_allowed_paths(validator, console);
    validate_path_resolution_rules(validator, console);
    validate_blocked_arguments(validator, console);
}

fn validate_http(validator: &mut ConfigValidator, config: &Config) {
    for (index, scheme) in config.http.allowed_schemes.iter().enumerate() {
        validate_supported_value(validator, format!("http.allowed_schemes[{index}]"), scheme, HTTP_SCHEMES);
    }

    for (index, cidr) in config.http.blocked_cidrs.iter().enumerate() {
        if cidr.parse::<IpNet>().is_err() {
            validator.issue(format!("http.blocked_cidrs[{index}]"), format!("invalid CIDR \"{cidr}\""));
        }
    }

    for (index, host) in config.http.blocked_hosts.iter().enumerate() {
        if looks_like_url_or_path(host) {
            validator.issue(format!("http.blocked_hosts[{index}]"), format!("must be a hostname, not a URL/path: \"{host}\""));
        }
    }
}

fn validate_command_rules(validator: &mut ConfigValidator, console: &ConsoleConfig) {
    let mut seen_commands = HashSet::new();

    for rule in &console.command_rules {
        if !seen_commands.insert(rule.name.as_str()) {
            validator.issue(command_field(rule, ""), format!("duplicate command \"{}\"", rule.name));
        }

        validate_supported_value(validator, command_field(rule, ".mode"), &rule.mode, COMMAND_MODES);
    }
}

fn validate_command_allowed_paths(validator: &mut ConfigValidator, console: &ConsoleConfig) {
    for rule in &console.command_rules {
        for (index, allowed_path) in rule.allowed_paths.iter().enumerate() {
            if !Path::new(allowed_path).is_absolute() {
                validator.issue(
                    format!("{}.allowed_paths[{index}]", command_field(rule, "")),
                    format!("path must be absolute: \"{allowed_path}\""),
                );
            }
        }
    }
}

fn validate_path_resolution_rules(validator: &mut ConfigValidator, console: &ConsoleConfig) {
    if console.allow_path_resolution {
        return;
    }

    if is_permissive_mode(&console.default_command_policy) {
        validator.issue(
            "console.default_command_policy",
            "cannot be \"allow\" or \"ask\" when console.allow_path_resolution=false; use [[console.commands]] with allowed_paths",
        );
    }

    if !console.allowed_commands.is_empty() {
        validator.issue(
            "console.allowed_commands",
            "cannot be used when console.allow_path_resolution=false; use [[console.commands]] with allowed_paths",
        );
    }

    if !console.ask_commands.is_empty() {
        validator.issue(
            "console.ask_commands",
            "cannot be used when console.allow_path_resolution=false; use [[console.commands]] with allowed_paths",
        );
    }

    for rule in &console.command_rules {
        if is_permissive_mode(&rule.mode) && rule.allowed_paths.is_empty() {
            validator.issue(
                command_field(rule, ".allowed_paths"),
                "must not be empty when console.allow_path_resolution=false and command mode is allow/ask",
            );
        }
    }
}

fn validate_blocked_arguments(validator: &mut ConfigValidator, console: &ConsoleConfig) {
    if has_permissive_console_commands(console) && console.blocked_arguments.is_empty() {
        validator.issue("console.blocked_arguments", "must not be empty when commands can be allowed or asked");
    }
}

// ─── < Private Functions > ──────────────────────────────────────────

fn validate_supported_value(validator: &mut ConfigValidator, field: impl Into<String>, value: &str, supported_values: &[&str]) {
    if supported_values.contains(&value) {
        return;
    }

    validator.issue(field, format!("unsupported value \"{value}\"; expected one of: {}", supported_values.join(", ")));
}

fn has_permissive_console_commands(console: &ConsoleConfig) -> bool {
    is_permissive_mode(&console.default_command_policy)
        || !console.allowed_commands.is_empty()
        || !console.ask_commands.is_empty()
        || console.command_rules.iter().any(|rule| is_permissive_mode(&rule.mode))
}

fn is_permissive_mode(mode: &str) -> bool {
    matches!(mode, "allow" | "ask")
}

fn command_field(rule: &ConsoleCommandRule, suffix: &str) -> String {
    format!("console.commands[{}]{suffix}", rule.name)
}

fn looks_like_url_or_path(value: &str) -> bool {
    value.contains("://") || value.contains('/')
}
