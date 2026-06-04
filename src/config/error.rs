// ─── < Imports > ────────────────────────────────────────────────────

use std::env;

use thiserror::Error;

use super::validation::ConfigValidationError;

// ─── < Errors > ─────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("invalid environment variable {name}={value:?}; expected {expected}")]
    InvalidEnvironmentValue {
        name: String,
        value: String,
        expected: &'static str,
    },

    #[error("runtime config validation failed")]
    Validation {
        #[source]
        source: ConfigValidationError,
    },

    #[error("failed to find HOME environment variable")]
    MissingHome {
        #[source]
        source: env::VarError,
    },

    #[error("failed to resolve config directory for '{path}'")]
    MissingParent { path: String },

    #[error("failed to create config directory '{path}'")]
    CreateDir {
        path: String,

        #[source]
        source: std::io::Error,
    },

    #[error("failed to read file '{path}'")]
    Read {
        path: String,

        #[source]
        source: std::io::Error,
    },

    #[error("failed to write file '{path}'")]
    Write {
        path: String,

        #[source]
        source: std::io::Error,
    },

    #[error("invalid JSON file '{path}'")]
    Json {
        path: String,

        #[source]
        source: serde_json::Error,
    },
}
