// ─── < Imports > ────────────────────────────────────────────────────

use std::collections::HashSet;
use std::env;
use std::fs;
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
    KnownAgent {
        id: "gemini-cli",
        display_name: "Gemini CLI",
        command_names: &["gemini", "gemini-cli"],
    },
    KnownAgent {
        id: "aider",
        display_name: "Aider",
        command_names: &["aider"],
    },
    KnownAgent {
        id: "goose",
        display_name: "Goose",
        command_names: &["goose"],
    },
    KnownAgent {
        id: "qwen-code",
        display_name: "Qwen Code",
        command_names: &["qwen", "qwen-code"],
    },
    KnownAgent {
        id: "plandex",
        display_name: "Plandex",
        command_names: &["plandex", "pdx"],
    },
    KnownAgent {
        id: "crush",
        display_name: "Crush",
        command_names: &["crush"],
    },
    KnownAgent {
        id: "open-interpreter",
        display_name: "Open Interpreter",
        command_names: &["interpreter", "open-interpreter"],
    },
];

const CANDIDATE_NAME_HINTS: &[&str] = &[
    "agent",
    "assistant",
    "aider",
    "amp",
    "augment",
    "claude",
    "cline",
    "codex",
    "copilot",
    "cursor",
    "devin",
    "gemini",
    "goose",
    "interpreter",
    "junie",
    "llm",
    "mcp",
    "openhands",
    "openai",
    "opencode",
    "plandex",
    "qwen",
    "roo",
    "windsurf",
];

const IGNORED_CANDIDATE_COMMANDS: &[&str] = &[
    "git", "cargo", "rustc", "rustup", "node", "npm", "npx", "pnpm", "bun", "python", "python3", "pip", "pip3", "sh", "bash", "zsh",
    "fish", "ls", "cat", "echo", "pwd", "rg", "grep", "sed", "awk", "curl", "wget", "ssh", "scp",
];

// ─── < Structs > ────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentScan {
    detected_agents: Vec<AgentDiscovery>,
    missing_known_agents: Vec<MissingKnownAgent>,
    candidate_agents: Vec<AgentCandidate>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentDiscovery {
    id: String,
    display_name: String,
    detected_command: String,
    path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MissingKnownAgent {
    id: String,
    display_name: String,
    command_names: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentCandidate {
    command_name: String,
    path: PathBuf,
    reason: String,
}

#[derive(Debug)]
struct KnownAgent {
    id: &'static str,
    display_name: &'static str,
    command_names: &'static [&'static str],
}

// ─── < Public Functions > ───────────────────────────────────────────

pub fn scan_installed_agents() -> AgentScan {
    scan_agents(false)
}

pub fn scan_known_agents() -> AgentScan {
    scan_agents(true)
}

// ─── < Implementations > ────────────────────────────────────────────

impl AgentScan {
    pub fn detected_agents(&self) -> &[AgentDiscovery] {
        &self.detected_agents
    }

    pub fn missing_known_agents(&self) -> &[MissingKnownAgent] {
        &self.missing_known_agents
    }

    pub fn candidate_agents(&self) -> &[AgentCandidate] {
        &self.candidate_agents
    }
}

impl AgentDiscovery {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    pub fn detected_command(&self) -> &str {
        &self.detected_command
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl MissingKnownAgent {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    pub fn command_names(&self) -> &[String] {
        &self.command_names
    }
}

impl AgentCandidate {
    pub fn command_name(&self) -> &str {
        &self.command_name
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}

// ─── < Private Functions > ──────────────────────────────────────────

fn scan_agents(include_missing_known_agents: bool) -> AgentScan {
    let known_command_names = known_command_names();
    let (detected_agents, missing_known_agents) = scan_known_agent_commands(include_missing_known_agents);
    let candidate_agents = scan_candidate_agents(&known_command_names);

    AgentScan {
        detected_agents,
        missing_known_agents,
        candidate_agents,
    }
}

fn scan_known_agent_commands(include_missing_known_agents: bool) -> (Vec<AgentDiscovery>, Vec<MissingKnownAgent>) {
    let mut detected_agents = Vec::new();
    let mut missing_known_agents = Vec::new();

    for agent in KNOWN_AGENTS {
        let detected = agent
            .command_names
            .iter()
            .find_map(|command_name| find_command_in_path(command_name).map(|path| ((*command_name).to_string(), path)));

        if let Some((detected_command, path)) = detected {
            detected_agents.push(AgentDiscovery {
                id: agent.id.to_string(),
                display_name: agent.display_name.to_string(),
                detected_command,
                path,
            });

            continue;
        }

        if include_missing_known_agents {
            missing_known_agents.push(MissingKnownAgent {
                id: agent.id.to_string(),
                display_name: agent.display_name.to_string(),
                command_names: agent.command_names.iter().map(|command_name| (*command_name).to_string()).collect(),
            });
        }
    }

    detected_agents.sort_by(|left, right| left.id.cmp(&right.id));
    missing_known_agents.sort_by(|left, right| left.id.cmp(&right.id));

    (detected_agents, missing_known_agents)
}

fn scan_candidate_agents(known_command_names: &HashSet<String>) -> Vec<AgentCandidate> {
    let mut candidates = Vec::new();
    let mut seen_command_names = HashSet::new();

    for directory in path_directories() {
        let Ok(entries) = fs::read_dir(&directory) else {
            continue;
        };

        for entry in entries.flatten() {
            let command_name = entry.file_name();

            let Some(command_name) = command_name.to_str() else {
                continue;
            };

            if !seen_command_names.insert(command_name.to_string()) {
                continue;
            }

            if known_command_names.contains(command_name) || IGNORED_CANDIDATE_COMMANDS.contains(&command_name) {
                continue;
            }

            let path = entry.path();

            if !is_executable_file(&path) {
                continue;
            }

            let Some(reason) = candidate_reason(command_name) else {
                continue;
            };

            candidates.push(AgentCandidate {
                command_name: command_name.to_string(),
                path,
                reason,
            });
        }
    }

    candidates.sort_by(|left, right| left.command_name.cmp(&right.command_name));

    candidates
}

fn candidate_reason(command_name: &str) -> Option<String> {
    let lower_name = command_name.to_ascii_lowercase();

    if has_token(&lower_name, "ai") {
        return Some("command name has token \"ai\"".to_string());
    }

    for hint in CANDIDATE_NAME_HINTS {
        if lower_name.contains(hint) {
            return Some(format!("command name contains \"{hint}\""));
        }
    }

    None
}

fn has_token(value: &str, token: &str) -> bool {
    value
        .split(|character: char| !character.is_ascii_alphanumeric())
        .any(|part| part == token)
}

fn known_command_names() -> HashSet<String> {
    KNOWN_AGENTS
        .iter()
        .flat_map(|agent| agent.command_names.iter())
        .map(|command_name| (*command_name).to_string())
        .collect()
}

fn find_command_in_path(command_name: &str) -> Option<PathBuf> {
    path_directories()
        .into_iter()
        .map(|directory| directory.join(command_name))
        .find(|candidate| is_executable_file(candidate))
}

fn path_directories() -> Vec<PathBuf> {
    let Some(path) = env::var_os("PATH") else {
        return Vec::new();
    };

    let ignored_directories = arc_managed_directories();
    let mut seen = HashSet::new();
    let mut directories = Vec::new();

    for directory in env::split_paths(&path) {
        if is_arc_managed_directory(&directory, &ignored_directories) {
            continue;
        }

        if seen.insert(directory.clone()) {
            directories.push(directory);
        }
    }

    directories
}

fn arc_managed_directories() -> Vec<PathBuf> {
    let mut directories = Vec::new();

    if let Some(path) = optional_env_path("ARC_LAUNCHER_DIR") {
        directories.push(path);
    }

    if let Some(path) = optional_env_path("ARC_RUNTIME_SHIMS_DIR") {
        directories.push(path);
    }

    if let Ok(home) = env::var("HOME") {
        let arc_data_dir = PathBuf::from(home).join(".local").join("share").join("arc");

        directories.push(arc_data_dir.join("launchers"));
        directories.push(arc_data_dir.join("runtime-shims"));
    }

    dedupe_paths(directories)
}

fn optional_env_path(name: &str) -> Option<PathBuf> {
    env::var(name)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
}

fn dedupe_paths(paths: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut seen = HashSet::new();
    let mut unique_paths = Vec::new();

    for path in paths {
        if seen.insert(path.clone()) {
            unique_paths.push(path);
        }
    }

    unique_paths
}

fn is_arc_managed_directory(directory: &Path, ignored_directories: &[PathBuf]) -> bool {
    ignored_directories
        .iter()
        .any(|ignored_directory| directory == ignored_directory || directory.starts_with(ignored_directory))
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
