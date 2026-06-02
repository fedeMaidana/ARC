// ─── < Imports > ────────────────────────────────────────────────────

use std::fs::{self, File, OpenOptions};
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

    let file = open_audit_log(&path)?;

    restrict_audit_log_permissions(&file, &path)?;

    Ok(())
}

pub fn record_event(config: &AuditConfig, event: &AuditEvent) -> Result<(), AuditError> {
    if !config.enabled {
        return Ok(());
    }

    let path = resolve_audit_path(&config.path)?;

    ensure_parent_dir_exists(&path)?;

    let serialized_event = serde_json::to_string(event).map_err(|source| AuditError::Serialize { source })?;

    let mut file = open_audit_log(&path)?;

    restrict_audit_log_permissions(&file, &path)?;

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

fn open_audit_log(path: &Path) -> Result<File, AuditError> {
    OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|source| AuditError::Open {
            path: path.display().to_string(),
            source,
        })
}

#[cfg(unix)]
fn restrict_audit_log_permissions(file: &File, path: &Path) -> Result<(), AuditError> {
    use std::os::unix::fs::PermissionsExt;

    let mut permissions = file
        .metadata()
        .map_err(|source| AuditError::SetPermissions {
            path: path.display().to_string(),
            source,
        })?
        .permissions();

    permissions.set_mode(0o600);

    file.set_permissions(permissions).map_err(|source| AuditError::SetPermissions {
        path: path.display().to_string(),
        source,
    })
}

#[cfg(not(unix))]
fn restrict_audit_log_permissions(_file: &File, _path: &Path) -> Result<(), AuditError> {
    Ok(())
}
