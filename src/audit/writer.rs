// ─── < Imports > ────────────────────────────────────────────────────

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

use crate::config::AuditConfig;

use super::path::resolve_audit_path;
use super::{AuditError, AuditEvent};

// ─── < Public Functions > ───────────────────────────────────────────

pub fn ensure_audit_log_is_writable(config: &AuditConfig) -> Result<(), AuditError> {
    if !config.enabled {
        return Ok(());
    }

    let path = resolve_audit_path(&config.path)?;

    ensure_parent_dir_exists(&path)?;

    OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map_err(|source| AuditError::Open {
            path: path.display().to_string(),
            source,
        })?;

    Ok(())
}

pub fn record_event(config: &AuditConfig, event: &AuditEvent) -> Result<(), AuditError> {
    if !config.enabled {
        return Ok(());
    }

    let path = resolve_audit_path(&config.path)?;

    ensure_parent_dir_exists(&path)?;

    let serialized_event = serde_json::to_string(event).map_err(|source| AuditError::Serialize { source })?;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .map_err(|source| AuditError::Open {
            path: path.display().to_string(),
            source,
        })?;

    writeln!(file, "{serialized_event}").map_err(|source| AuditError::Write {
        path: path.display().to_string(),
        source,
    })?;

    Ok(())
}

// ─── < Private Functions > ──────────────────────────────────────────

fn ensure_parent_dir_exists(path: &Path) -> Result<(), AuditError> {
    let Some(parent_dir) = path.parent() else {
        return Ok(());
    };

    fs::create_dir_all(parent_dir).map_err(|source| AuditError::CreateDir {
        path: parent_dir.display().to_string(),
        source,
    })
}
