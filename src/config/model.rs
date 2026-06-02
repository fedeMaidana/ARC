// ─── < Imports > ────────────────────────────────────────────────────

use serde::Deserialize;

// ─── < Structs > ────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_config_version")]
    pub config_version: u32,

    #[serde(default)]
    pub policy: PolicyConfig,

    #[serde(default)]
    pub agents: AgentsConfig,

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
pub struct PolicyConfig {
    #[serde(default = "default_policy_engine")]
    pub engine: String,

    #[serde(default = "default_policy_action")]
    pub default_action: String,
}

#[derive(Debug, Deserialize)]
pub struct AgentsConfig {
    #[serde(default = "default_allow_unknown_sources")]
    pub allow_unknown_sources: bool,

    #[serde(default)]
    pub sources: Vec<AgentSourceConfig>,
}

#[derive(Debug, Deserialize)]
pub struct AgentSourceConfig {
    pub id: String,

    pub display_name: String,

    #[serde(default = "default_agent_source_enabled")]
    pub enabled: bool,

    #[serde(default = "default_agent_kind")]
    pub kind: String,

    #[serde(default)]
    pub description: Option<String>,
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
    #[serde(default, alias = "protected_names")]
    pub protected: Vec<String>,

    pub blocked_path_prefixes: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct HttpConfig {
    #[serde(default = "default_allowed_http_schemes")]
    pub allowed_schemes: Vec<String>,

    #[serde(default = "default_true")]
    pub block_localhost: bool,

    #[serde(default = "default_true")]
    pub block_private_networks: bool,

    #[serde(default = "default_true")]
    pub block_link_local: bool,

    #[serde(default = "default_true")]
    pub block_metadata_services: bool,

    #[serde(default)]
    pub blocked_hosts: Vec<String>,

    #[serde(default)]
    pub blocked_cidrs: Vec<String>,

    #[serde(default)]
    pub blocked_targets: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ConsoleConfig {
    #[serde(default = "default_command_policy")]
    pub default_command_policy: String,

    #[serde(default = "default_allow_path_resolution")]
    pub allow_path_resolution: bool,

    #[serde(default)]
    pub allowed_commands: Vec<String>,

    #[serde(default)]
    pub blocked_commands: Vec<String>,

    #[serde(default)]
    pub blocked_arguments: Vec<String>,

    #[serde(default)]
    pub ask_commands: Vec<String>,

    #[serde(default, alias = "commands")]
    pub command_rules: Vec<ConsoleCommandRule>,
}

#[derive(Debug, Deserialize)]
pub struct ConsoleCommandRule {
    pub name: String,

    #[serde(default = "default_command_rule_mode")]
    pub mode: String,

    #[serde(default)]
    pub risk: Option<String>,

    #[serde(default)]
    pub allowed_paths: Vec<String>,

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

impl Default for PolicyConfig {
    fn default() -> Self {
        Self {
            engine: default_policy_engine(),
            default_action: default_policy_action(),
        }
    }
}

impl Default for AgentsConfig {
    fn default() -> Self {
        Self {
            allow_unknown_sources: default_allow_unknown_sources(),
            sources: Vec::new(),
        }
    }
}

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

fn default_config_version() -> u32 {
    1
}

fn default_policy_engine() -> String {
    "native".to_string()
}

fn default_policy_action() -> String {
    "deny".to_string()
}

fn default_allow_unknown_sources() -> bool {
    true
}

fn default_agent_source_enabled() -> bool {
    true
}

fn default_agent_kind() -> String {
    "custom".to_string()
}

fn default_allowed_http_schemes() -> Vec<String> {
    vec!["http".to_string(), "https".to_string()]
}

fn default_true() -> bool {
    true
}

fn default_command_policy() -> String {
    "deny".to_string()
}

fn default_allow_path_resolution() -> bool {
    true
}

fn default_command_rule_mode() -> String {
    "allow".to_string()
}

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
