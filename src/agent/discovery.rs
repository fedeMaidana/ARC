// ─── < Imports > ────────────────────────────────────────────────────

use std::env;
use std::path::{Path, PathBuf};

// ─── < Constants > ──────────────────────────────────────────────────

const KNOWN_AGENTS: &[KnownAgent] = &[
    KnownAgent {
        id: "opencode",
        display_name: "OpenCode",
        command_names: &["opencode"],
    },
    KnownAgent {
        id: "claude-code",
        display_name: "Claude Code",
        command_names: &["claude", "claude-code"],
    },
    KnownAgent {
        id: "codex",
        display_name: "Codex",
        command_names: &["codex"],
    },
];

// ─── < Structs > ────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentDiscovery {
    id: String,
    display_name: String,
    command_names: Vec<String>,
    detected_command: Option<String>,
    path: Option<PathBuf>,
}

#[derive(Debug)]
struct KnownAgent {
    id: &'static str,
    display_name: &'static str,
    command_names: &'static [&'static str],
}

// ─── < Public Functions > ───────────────────────────────────────────

pub fn scan_known_agents() -> Vec<AgentDiscovery> {
    KNOWN_AGENTS.iter().map(discover_known_agent).collect()
}

// ─── < Implementations > ────────────────────────────────────────────

impl AgentDiscovery {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    pub fn command_names(&self) -> &[String] {
        &self.command_names
    }

    pub fn detected_command(&self) -> Option<&str> {
        self.detected_command.as_deref()
    }

    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    pub fn is_detected(&self) -> bool {
        self.path.is_some()
    }
}

// ─── < Private Functions > ──────────────────────────────────────────

fn discover_known_agent(agent: &KnownAgent) -> AgentDiscovery {
    let detected = agent
        .command_names
        .iter()
        .find_map(|command_name| find_command_in_path(command_name).map(|path| ((*command_name).to_string(), path)));

    let (detected_command, path) = match detected {
        Some((command_name, path)) => (Some(command_name), Some(path)),
        None => (None, None),
    };

    AgentDiscovery {
        id: agent.id.to_string(),
        display_name: agent.display_name.to_string(),
        command_names: agent.command_names.iter().map(|command| (*command).to_string()).collect(),
        detected_command,
        path,
    }
}

fn find_command_in_path(command_name: &str) -> Option<PathBuf> {
    let path = env::var_os("PATH")?;

    env::split_paths(&path)
        .map(|directory| directory.join(command_name))
        .find(|candidate| is_executable_file(candidate))
}

fn is_executable_file(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }

    has_execute_permission(path)
}

#[cfg(unix)]
fn has_execute_permission(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;

    path.metadata()
        .map(|metadata| metadata.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

#[cfg(not(unix))]
fn has_execute_permission(_path: &Path) -> bool {
    true
}
