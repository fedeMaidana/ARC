// ─── < Imports > ────────────────────────────────────────────────────

use std::env;

use thiserror::Error;

// ─── < Errors > ─────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum AuditError {
    #[error("failed to find HOME environment variable")]
    MissingHome {
        #[source]
        source: env::VarError,
    },

    #[error("failed to create audit directory '{path}'")]
    CreateDir {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("failed to open audit log '{path}'")]
    Open {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("failed to restrict audit log permissions '{path}'")]
    SetPermissions {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("failed to serialize audit event")]
    Serialize {
        #[source]
        source: serde_json::Error,
    },

    #[error("failed to write audit log '{path}'")]
    Write {
        path: String,
        #[source]
        source: std::io::Error,
    },
}
