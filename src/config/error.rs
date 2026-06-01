// ─── < Imports > ────────────────────────────────────────────────────

use std::env;

use thiserror::Error;

// ─── < Errors > ─────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("failed to read config file '{path}'")]
    Read {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("failed to parse config file '{path}'")]
    Parse {
        path: String,
        #[source]
        source: toml::de::Error,
    },

    #[error("config file not found.\n\nRun:\n  arc init\n\nOr provide one manually:\n  ARC_CONFIG=/path/to/arc.toml arc run ls -la")]
    NotFound,

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

    #[error("failed to write config file '{path}'")]
    Write {
        path: String,
        #[source]
        source: std::io::Error,
    },
}
