// ─── < Modules > ────────────────────────────────────────────────────

mod error;
mod init;
mod loader;
mod model;
mod paths;
mod rules;
mod validation;

// ─── < Public Exports > ─────────────────────────────────────────────

pub use self::error::ConfigError;
pub use self::init::{ConfigInitResult, PolicyInitResult, init_default_config};
pub use self::loader::{load_from_default_locations, load_runtime_config};
pub use self::model::{
    ActionsConfig, AgentSourceConfig, AgentsConfig, AuditConfig, Config, ConsoleCommandRule, ConsoleConfig, ExecutionConfig,
    ExecutionEnvironmentVariable, HttpConfig, PolicyConfig, RegoPolicyConfig, ResourcesConfig,
};
pub use self::paths::{
    default_user_agent_registry_path, default_user_data_dir, default_user_policies_dir, default_user_policy_path,
    runtime_config_source_path,
};
pub use self::rules::{ConsoleCommandPolicy, ConsoleSubcommandPolicy, DefaultPolicyAction};
pub use self::validation::{ConfigValidationError, ConfigValidationIssue, validate};
