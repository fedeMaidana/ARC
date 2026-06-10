// ─── < Imports > ────────────────────────────────────────────────────

use std::env;
use std::path::PathBuf;

use super::AuditError;

// ─── < Public Functions > ───────────────────────────────────────────

pub fn resolve_audit_path(path: &str) -> Result<PathBuf, AuditError> {
    if path == "~" {
        return home_dir();
    }

    if let Some(relative_path) = path.strip_prefix("~/") {
        return Ok(home_dir()?.join(relative_path));
    }

    Ok(PathBuf::from(path))
}

// ─── < Private Functions > ──────────────────────────────────────────

fn home_dir() -> Result<PathBuf, AuditError> {
    let home = env::var("HOME").map_err(|source| AuditError::MissingHome { source })?;

    Ok(PathBuf::from(home))
}
