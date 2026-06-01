// ─── < Imports > ────────────────────────────────────────────────────

use thiserror::Error;

// ─── < Errors > ─────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum JsonApiError {
    #[error("invalid JSON request")]
    InvalidJson {
        #[source]
        source: serde_json::Error,
    },

    #[error("action is required")]
    MissingAction,

    #[error("run action requires a command array")]
    MissingCommand,

    #[error("command array cannot be empty")]
    EmptyCommand,

    #[error("run action cannot use resource; use command instead")]
    ResourceNotAllowedForRun,

    #[error("command can only be used with run action")]
    CommandOnlyAllowedForRun,
}

// ─── < Implementations > ────────────────────────────────────────────

impl JsonApiError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidJson { .. } => "invalid_json",
            Self::MissingAction => "missing_action",
            Self::MissingCommand => "missing_command",
            Self::EmptyCommand => "empty_command",
            Self::ResourceNotAllowedForRun => "resource_not_allowed_for_run",
            Self::CommandOnlyAllowedForRun => "command_only_allowed_for_run",
        }
    }
}
