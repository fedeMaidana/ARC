// ─── < Imports > ────────────────────────────────────────────────────

use serde::Deserialize;

// ─── < Structs > ────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct Config {
    pub actions: ActionsConfig,
    pub resources: ResourcesConfig,
    pub http: HttpConfig,
    pub console: ConsoleConfig,

    #[serde(default)]
    pub audit: AuditConfig,

    #[serde(default)]
    pub execution: ExecutionConfig,
}

#[derive(Debug, Deserialize)]
pub struct ActionsConfig {
    pub allowed: Vec<String>,
    pub blocked: Vec<String>,
    pub need_resource: Vec<String>,

    #[serde(default)]
    pub ask: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ResourcesConfig {
    pub protected: Vec<String>,
    pub blocked_path_prefixes: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct HttpConfig {
    pub blocked_targets: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ConsoleConfig {
    pub allowed_commands: Vec<String>,
    pub blocked_commands: Vec<String>,
    pub blocked_arguments: Vec<String>,

    #[serde(default)]
    pub ask_commands: Vec<String>,

    #[serde(default)]
    pub command_rules: Vec<ConsoleCommandRule>,
}

#[derive(Debug, Deserialize)]
pub struct ConsoleCommandRule {
    pub name: String,

    #[serde(default)]
    pub allowed_subcommands: Vec<String>,

    #[serde(default)]
    pub blocked_subcommands: Vec<String>,

    #[serde(default)]
    pub ask_subcommands: Vec<String>,

    #[serde(default)]
    pub blocked_arguments: Vec<String>,

    #[serde(default)]
    pub ask_arguments: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct AuditConfig {
    #[serde(default = "default_audit_enabled")]
    pub enabled: bool,

    #[serde(default = "default_audit_path")]
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct ExecutionConfig {
    #[serde(default = "default_timeout_seconds")]
    pub timeout_seconds: u64,

    #[serde(default = "default_max_output_bytes")]
    pub max_output_bytes: usize,

    #[serde(default = "default_inherit_environment")]
    pub inherit_environment: bool,

    #[serde(default)]
    pub working_directory: Option<String>,

    #[serde(default)]
    pub environment: Vec<ExecutionEnvironmentVariable>,
}

#[derive(Debug, Deserialize)]
pub struct ExecutionEnvironmentVariable {
    pub name: String,
    pub value: String,
}

// ─── < Implementations > ────────────────────────────────────────────

impl Default for AuditConfig {
    fn default() -> Self {
        Self {
            enabled: default_audit_enabled(),
            path: default_audit_path(),
        }
    }
}

impl Default for ExecutionConfig {
    fn default() -> Self {
        Self {
            timeout_seconds: default_timeout_seconds(),
            max_output_bytes: default_max_output_bytes(),
            inherit_environment: default_inherit_environment(),
            working_directory: None,
            environment: Vec::new(),
        }
    }
}

// ─── < Private Functions > ──────────────────────────────────────────

fn default_audit_enabled() -> bool {
    true
}

fn default_audit_path() -> String {
    "~/.local/share/arc/audit.log".to_string()
}

fn default_timeout_seconds() -> u64 {
    10
}

fn default_max_output_bytes() -> usize {
    100_000
}

fn default_inherit_environment() -> bool {
    false
}
