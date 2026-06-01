// ─── < Imports > ────────────────────────────────────────────────────

use std::env;
use std::path::{Path, PathBuf};

// ─── < Public Functions > ───────────────────────────────────────────

pub(super) fn resolve_command_path(command_name: &str) -> Result<PathBuf, String> {
    if command_name.trim().is_empty() {
        return Err("command name cannot be empty".to_string());
    }

    if command_contains_path_separator(command_name) {
        return Ok(PathBuf::from(command_name));
    }

    for search_path in search_path_entries() {
        let candidate = search_path.join(command_name);

        if is_executable_file(&candidate) {
            return Ok(candidate);
        }
    }

    Err(format!("command not found in PATH: {command_name}"))
}

// ─── < Private Functions > ──────────────────────────────────────────

fn command_contains_path_separator(command_name: &str) -> bool {
    command_name.contains('/') || command_name.contains('\\')
}

fn search_path_entries() -> Vec<PathBuf> {
    env::var_os("PATH")
        .map(|paths| env::split_paths(&paths).collect())
        .unwrap_or_default()
}

#[cfg(unix)]
fn is_executable_file(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;

    let Ok(metadata) = path.metadata() else {
        return false;
    };

    metadata.is_file() && metadata.permissions().mode() & 0o111 != 0
}

#[cfg(not(unix))]
fn is_executable_file(path: &Path) -> bool {
    path.is_file()
}
