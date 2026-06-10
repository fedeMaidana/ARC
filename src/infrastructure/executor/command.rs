// ─── < Imports > ────────────────────────────────────────────────────

use std::env;
use std::path::{Path, PathBuf};

use crate::config::{ConsoleCommandRule, ConsoleConfig};

// ─── < Public Functions > ───────────────────────────────────────────

pub(super) fn resolve_command_path(command_name: &str, console_config: &ConsoleConfig) -> Result<PathBuf, String> {
    if command_name.trim().is_empty() {
        return Err("command name cannot be empty".to_string());
    }

    if let Some(command_rule) = console_config.command_rule(command_name)
        && !command_rule.allowed_paths.is_empty()
    {
        return resolve_allowed_command_path(command_name, command_rule);
    }

    if command_contains_path_separator(command_name) {
        return Err(format!("command path is not allowed unless it is configured with allowed_paths: {command_name}"));
    }

    if !console_config.allow_path_resolution {
        return Err(format!("PATH resolution is disabled and no allowed path was configured for command: {command_name}"));
    }

    resolve_from_path(command_name)
}

// ─── < Private Functions > ──────────────────────────────────────────

fn resolve_allowed_command_path(command_name: &str, command_rule: &ConsoleCommandRule) -> Result<PathBuf, String> {
    for allowed_path in &command_rule.allowed_paths {
        let candidate = PathBuf::from(allowed_path);

        if !candidate.is_absolute() {
            return Err(format!("allowed path for command '{command_name}' must be absolute: {allowed_path}"));
        }

        if !path_file_name_matches_command(&candidate, command_name) {
            return Err(format!("allowed path for command '{command_name}' points to a different binary: {allowed_path}"));
        }

        if is_executable_file(&candidate) {
            return Ok(candidate);
        }
    }

    Err(format!("no executable allowed path was found for command: {command_name}"))
}

fn resolve_from_path(command_name: &str) -> Result<PathBuf, String> {
    for search_path in search_path_entries() {
        let candidate = search_path.join(command_name);

        if is_executable_file(&candidate) {
            return Ok(candidate);
        }
    }

    Err(format!("command not found in PATH: {command_name}"))
}

fn path_file_name_matches_command(path: &Path, command_name: &str) -> bool {
    path.file_name()
        .and_then(|file_name| file_name.to_str())
        .is_some_and(|file_name| file_name == command_name)
}

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
