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
pub use self::init::{ConfigInitResult, init_default_config};
pub use self::loader::load_from_default_locations;
pub use self::model::{
    ActionsConfig, AgentSourceConfig, AgentsConfig, AuditConfig, Config, ConsoleCommandRule, ConsoleConfig, ExecutionConfig,
    ExecutionEnvironmentVariable, HttpConfig, PolicyConfig, ResourcesConfig,
};
pub use self::paths::{default_user_config_path, resolve_config_path};
pub use self::rules::{ConsoleCommandPolicy, ConsoleSubcommandPolicy, DefaultPolicyAction};
pub use self::validation::{ConfigValidationError, ConfigValidationIssue, validate};
