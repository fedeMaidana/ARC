// ─── < Imports > ────────────────────────────────────────────────────

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::agent::{AgentRegistration, load_agent_registry};
use crate::config::ConfigError;

// ─── < Constants > ──────────────────────────────────────────────────

const RUNTIME_SHELL_SHIMS: &[&str] = &["bash", "sh"];
const UNSUPPORTED_SHELL_OPERATORS: &[char] = &['|', ';', '&', '<', '>', '`', '$', '(', ')', '\n', '\r'];

const ARC_PROFILE_BLOCK_START: &str = "# >>> ARC shims >>>";
const ARC_PROFILE_BLOCK_END: &str = "# <<< ARC shims <<<";

// ─── < Structs > ────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShimInstallReport {
    launcher_dir: PathBuf,
    runtime_shims_dir: PathBuf,
    launchers: Vec<LauncherShim>,
    runtime_shims: Vec<RuntimeShim>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShimListReport {
    launcher_dir: PathBuf,
    runtime_shims_dir: PathBuf,
    launchers: Vec<LauncherShimStatus>,
    runtime_shims: Vec<RuntimeShimStatus>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellActivationReport {
    profile_path: PathBuf,
    launcher_dir: PathBuf,
    status: ShellActivationStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LauncherShim {
    agent_id: String,
    command: String,
    real_path: String,
    shim_path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LauncherShimStatus {
    agent_id: String,
    command: String,
    real_path: String,
    shim_path: PathBuf,
    installed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeShim {
    command: String,
    shim_path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeShimStatus {
    command: String,
    shim_path: PathBuf,
    installed: bool,
}

// ─── < Enums > ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellActivationStatus {
    Created,
    Updated,
    Unchanged,
}

// ─── < Public Functions > ───────────────────────────────────────────

pub fn install_arc_shims(registry_path: &Path, launcher_dir: &Path, runtime_shims_dir: &Path) -> Result<ShimInstallReport, ConfigError> {
    let registry = load_agent_registry(registry_path)?;
    let arc_bin = current_arc_binary();

    fs::create_dir_all(launcher_dir).map_err(|source| ConfigError::CreateDir {
        path: launcher_dir.display().to_string(),
        source,
    })?;

    fs::create_dir_all(runtime_shims_dir).map_err(|source| ConfigError::CreateDir {
        path: runtime_shims_dir.display().to_string(),
        source,
    })?;

    let mut launchers = Vec::new();

    for agent in registry.agents().iter().filter(|agent| agent.enabled()) {
        let shim_path = launcher_dir.join(agent.command());
        let content = launcher_script(agent, runtime_shims_dir, &arc_bin);

        fs::write(&shim_path, content).map_err(|source| ConfigError::Write {
            path: shim_path.display().to_string(),
            source,
        })?;

        make_executable(&shim_path)?;

        launchers.push(LauncherShim {
            agent_id: agent.id().to_string(),
            command: agent.command().to_string(),
            real_path: agent.path().to_string(),
            shim_path,
        });
    }

    let mut runtime_shims = Vec::new();

    for command in RUNTIME_SHELL_SHIMS {
        let shim_path = runtime_shims_dir.join(command);
        let content = runtime_shell_shim_script(command);

        fs::write(&shim_path, content).map_err(|source| ConfigError::Write {
            path: shim_path.display().to_string(),
            source,
        })?;

        make_executable(&shim_path)?;

        runtime_shims.push(RuntimeShim {
            command: (*command).to_string(),
            shim_path,
        });
    }

    launchers.sort_by(|left, right| left.command.cmp(&right.command));
    runtime_shims.sort_by(|left, right| left.command.cmp(&right.command));

    Ok(ShimInstallReport {
        launcher_dir: launcher_dir.to_path_buf(),
        runtime_shims_dir: runtime_shims_dir.to_path_buf(),
        launchers,
        runtime_shims,
    })
}

pub fn list_arc_shims(registry_path: &Path, launcher_dir: &Path, runtime_shims_dir: &Path) -> Result<ShimListReport, ConfigError> {
    let registry = load_agent_registry(registry_path)?;

    let mut launchers = registry
        .agents()
        .iter()
        .filter(|agent| agent.enabled())
        .map(|agent| {
            let shim_path = launcher_dir.join(agent.command());

            LauncherShimStatus {
                agent_id: agent.id().to_string(),
                command: agent.command().to_string(),
                real_path: agent.path().to_string(),
                installed: shim_path.is_file(),
                shim_path,
            }
        })
        .collect::<Vec<_>>();

    let mut runtime_shims = RUNTIME_SHELL_SHIMS
        .iter()
        .map(|command| {
            let shim_path = runtime_shims_dir.join(command);

            RuntimeShimStatus {
                command: (*command).to_string(),
                installed: shim_path.is_file(),
                shim_path,
            }
        })
        .collect::<Vec<_>>();

    launchers.sort_by(|left, right| left.command.cmp(&right.command));
    runtime_shims.sort_by(|left, right| left.command.cmp(&right.command));

    Ok(ShimListReport {
        launcher_dir: launcher_dir.to_path_buf(),
        runtime_shims_dir: runtime_shims_dir.to_path_buf(),
        launchers,
        runtime_shims,
    })
}

pub fn activate_shell_profile(launcher_dir: &Path) -> Result<ShellActivationReport, ConfigError> {
    let profile_path = shell_profile_path()?;

    if let Some(parent_dir) = profile_path.parent().filter(|path| !path.as_os_str().is_empty()) {
        fs::create_dir_all(parent_dir).map_err(|source| ConfigError::CreateDir {
            path: parent_dir.display().to_string(),
            source,
        })?;
    }

    let existing_content = if profile_path.exists() {
        fs::read_to_string(&profile_path).map_err(|source| ConfigError::Read {
            path: profile_path.display().to_string(),
            source,
        })?
    } else {
        String::new()
    };

    let block = shell_profile_block(launcher_dir);
    let (new_content, status) = update_shell_profile_content(&existing_content, &block);

    if status != ShellActivationStatus::Unchanged {
        fs::write(&profile_path, new_content).map_err(|source| ConfigError::Write {
            path: profile_path.display().to_string(),
            source,
        })?;
    }

    Ok(ShellActivationReport {
        profile_path,
        launcher_dir: launcher_dir.to_path_buf(),
        status,
    })
}

pub fn execute_shell_runtime_shim(shell: &str, args: &[String]) -> i32 {
    let Some(command_index) = shell_command_string_index(args) else {
        return exec_real_shell(shell, args);
    };

    if command_index >= args.len() {
        eprintln!("ARC shell shim error: missing shell command after -c");
        return 2;
    }

    let command = args[command_index].trim();

    if command.is_empty() {
        eprintln!("ARC shell shim blocked execution: empty shell command");
        return 2;
    }

    let command_parts = match split_simple_shell_command(command) {
        Ok(parts) => parts,
        Err(error) => {
            eprintln!("ARC blocked unsupported shell command.");
            eprintln!("Reason: {error}");
            eprintln!("Command: {command}");
            return 126;
        }
    };

    run_arc_command(&command_parts)
}

// ─── < Implementations > ────────────────────────────────────────────

impl ShimInstallReport {
    pub fn launcher_dir(&self) -> &Path {
        &self.launcher_dir
    }

    pub fn runtime_shims_dir(&self) -> &Path {
        &self.runtime_shims_dir
    }

    pub fn launchers(&self) -> &[LauncherShim] {
        &self.launchers
    }

    pub fn runtime_shims(&self) -> &[RuntimeShim] {
        &self.runtime_shims
    }
}

impl ShimListReport {
    pub fn launcher_dir(&self) -> &Path {
        &self.launcher_dir
    }

    pub fn runtime_shims_dir(&self) -> &Path {
        &self.runtime_shims_dir
    }

    pub fn launchers(&self) -> &[LauncherShimStatus] {
        &self.launchers
    }

    pub fn runtime_shims(&self) -> &[RuntimeShimStatus] {
        &self.runtime_shims
    }
}

impl ShellActivationReport {
    pub fn profile_path(&self) -> &Path {
        &self.profile_path
    }

    pub fn launcher_dir(&self) -> &Path {
        &self.launcher_dir
    }

    pub fn status(&self) -> ShellActivationStatus {
        self.status
    }
}

impl LauncherShim {
    pub fn agent_id(&self) -> &str {
        &self.agent_id
    }

    pub fn command(&self) -> &str {
        &self.command
    }

    pub fn real_path(&self) -> &str {
        &self.real_path
    }

    pub fn shim_path(&self) -> &Path {
        &self.shim_path
    }
}

impl LauncherShimStatus {
    pub fn agent_id(&self) -> &str {
        &self.agent_id
    }

    pub fn command(&self) -> &str {
        &self.command
    }

    pub fn real_path(&self) -> &str {
        &self.real_path
    }

    pub fn shim_path(&self) -> &Path {
        &self.shim_path
    }

    pub fn installed(&self) -> bool {
        self.installed
    }
}

impl RuntimeShim {
    pub fn command(&self) -> &str {
        &self.command
    }

    pub fn shim_path(&self) -> &Path {
        &self.shim_path
    }
}

impl RuntimeShimStatus {
    pub fn command(&self) -> &str {
        &self.command
    }

    pub fn shim_path(&self) -> &Path {
        &self.shim_path
    }

    pub fn installed(&self) -> bool {
        self.installed
    }
}

// ─── < Private Functions: Scripts > ─────────────────────────────────

fn launcher_script(agent: &AgentRegistration, runtime_shims_dir: &Path, arc_bin: &Path) -> String {
    let agent_id = shell_single_quote(agent.id());
    let command = shell_single_quote(agent.command());
    let real_path = shell_single_quote(agent.path());
    let runtime_shims_dir = shell_single_quote(&runtime_shims_dir.display().to_string());
    let arc_bin = shell_single_quote(&arc_bin.display().to_string());

    format!(
        r#"#!/usr/bin/env sh
# Generated by ARC. Do not edit by hand.

ARC_AGENT_ID={agent_id}
ARC_AGENT_COMMAND={command}
ARC_AGENT_REAL_PATH={real_path}
ARC_RUNTIME_SHIMS_DIR={runtime_shims_dir}

export ARC_SOURCE="$ARC_AGENT_ID"
export ARC_AGENT_COMMAND
export ARC_AGENT_REAL_PATH
export ARC_RUNTIME_SHIMS_DIR
export ARC_BIN={arc_bin}

if [ ! -x "$ARC_AGENT_REAL_PATH" ]; then
  echo "ARC launcher error: real agent binary not found or not executable: $ARC_AGENT_REAL_PATH" >&2
  exit 127
fi

ARC_ORIGINAL_PATH="${{ARC_ORIGINAL_PATH:-$PATH}}"
export ARC_ORIGINAL_PATH

PATH="$ARC_RUNTIME_SHIMS_DIR:$ARC_ORIGINAL_PATH"
export PATH

exec "$ARC_AGENT_REAL_PATH" "$@"
"#
    )
}

fn runtime_shell_shim_script(command: &str) -> String {
    format!(
        r#"#!/usr/bin/env sh
# Generated by ARC. Do not edit by hand.

ARC_BIN="${{ARC_BIN:-arc}}"
export ARC_NO_BANNER=1

exec "$ARC_BIN" __arc-shim shell {command} "$@"
"#
    )
}

fn shell_profile_path() -> Result<PathBuf, ConfigError> {
    if let Some(path) = env::var_os("ARC_SHELL_PROFILE_PATH")
        .map(PathBuf::from)
        .filter(|path| !path.as_os_str().is_empty())
    {
        return Ok(path);
    }

    let home = env::var("HOME").map_err(|source| ConfigError::MissingHome { source })?;
    let shell = env::var("SHELL").unwrap_or_default();

    if shell.ends_with("zsh") {
        return Ok(PathBuf::from(home).join(".zshrc"));
    }

    if shell.ends_with("bash") {
        return Ok(PathBuf::from(home).join(".bashrc"));
    }

    Ok(PathBuf::from(home).join(".profile"))
}

fn shell_profile_block(launcher_dir: &Path) -> String {
    format!(
        "{ARC_PROFILE_BLOCK_START}\n# Added by ARC. Routes registered agent commands through ARC launchers.\nexport PATH={}:\"$PATH\"\n{ARC_PROFILE_BLOCK_END}\n",
        shell_single_quote(&launcher_dir.display().to_string())
    )
}

fn update_shell_profile_content(content: &str, block: &str) -> (String, ShellActivationStatus) {
    if content.trim().is_empty() {
        return (block.to_string(), ShellActivationStatus::Created);
    }

    let Some(start_index) = content.find(ARC_PROFILE_BLOCK_START) else {
        return (append_shell_profile_block(content, block), ShellActivationStatus::Updated);
    };

    let Some(relative_end_index) = content[start_index..].find(ARC_PROFILE_BLOCK_END) else {
        return (append_shell_profile_block(content, block), ShellActivationStatus::Updated);
    };

    let block_end_index = start_index + relative_end_index + ARC_PROFILE_BLOCK_END.len();
    let replacement_end_index = consume_trailing_line_ending(content, block_end_index);

    let mut new_content = String::new();

    new_content.push_str(&content[..start_index]);
    new_content.push_str(block);
    new_content.push_str(&content[replacement_end_index..]);

    if new_content == content {
        (content.to_string(), ShellActivationStatus::Unchanged)
    } else {
        (new_content, ShellActivationStatus::Updated)
    }
}

fn consume_trailing_line_ending(content: &str, index: usize) -> usize {
    if content[index..].starts_with("\r\n") {
        return index + 2;
    }

    if content[index..].starts_with('\n') {
        return index + 1;
    }

    index
}

fn append_shell_profile_block(content: &str, block: &str) -> String {
    let mut new_content = content.to_string();

    if !new_content.ends_with('\n') {
        new_content.push('\n');
    }

    new_content.push('\n');
    new_content.push_str(block);

    new_content
}

fn shell_single_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}

fn current_arc_binary() -> PathBuf {
    env::current_exe().unwrap_or_else(|_| PathBuf::from("arc"))
}

// ─── < Private Functions: Runtime Shell Handling > ──────────────────

fn shell_command_string_index(args: &[String]) -> Option<usize> {
    for (index, arg) in args.iter().enumerate() {
        if arg == "-c" {
            return Some(index + 1);
        }

        if is_short_shell_option_with_c(arg) {
            return Some(index + 1);
        }
    }

    None
}

fn is_short_shell_option_with_c(value: &str) -> bool {
    value.starts_with('-') && !value.starts_with("--") && value.len() > 2 && value.chars().skip(1).any(|character| character == 'c')
}

fn split_simple_shell_command(command: &str) -> Result<Vec<String>, String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut quote: Option<char> = None;
    let mut escaping = false;

    for character in command.chars() {
        if escaping {
            current.push(character);
            escaping = false;
            continue;
        }

        if character == '\\' {
            escaping = true;
            continue;
        }

        if is_unsupported_shell_operator(character) {
            return Err(format!("unsupported shell operator '{character}'"));
        }

        if let Some(active_quote) = quote {
            if character == active_quote {
                quote = None;
            } else {
                current.push(character);
            }

            continue;
        }

        if character == '\'' || character == '"' {
            quote = Some(character);
            continue;
        }

        if character.is_whitespace() {
            if !current.is_empty() {
                parts.push(current);
                current = String::new();
            }

            continue;
        }

        current.push(character);
    }

    if escaping {
        current.push('\\');
    }

    if let Some(active_quote) = quote {
        return Err(format!("unterminated {active_quote} quote"));
    }

    if !current.is_empty() {
        parts.push(current);
    }

    if parts.is_empty() {
        return Err("empty command".to_string());
    }

    Ok(parts)
}

fn is_unsupported_shell_operator(character: char) -> bool {
    UNSUPPORTED_SHELL_OPERATORS.contains(&character)
}

fn run_arc_command(command_parts: &[String]) -> i32 {
    let arc_bin = env::var_os("ARC_BIN").unwrap_or_else(|| current_arc_binary().into_os_string());

    match Command::new(arc_bin)
        .arg("run")
        .args(command_parts)
        .env("ARC_NO_BANNER", "1")
        .status()
    {
        Ok(status) => status.code().unwrap_or(1),
        Err(error) => {
            eprintln!("ARC shell shim error: could not run ARC: {error}");
            127
        }
    }
}

fn exec_real_shell(shell: &str, args: &[String]) -> i32 {
    let Some(real_shell) = resolve_real_shell(shell) else {
        eprintln!("ARC shell shim error: could not find real shell binary for '{shell}'");
        return 127;
    };

    match Command::new(real_shell).args(args).status() {
        Ok(status) => status.code().unwrap_or(1),
        Err(error) => {
            eprintln!("ARC shell shim error: could not run real shell '{shell}': {error}");
            127
        }
    }
}

fn resolve_real_shell(shell: &str) -> Option<PathBuf> {
    let runtime_shims_dir = env::var_os("ARC_RUNTIME_SHIMS_DIR").map(PathBuf::from);
    let path = env::var_os("ARC_ORIGINAL_PATH").or_else(|| env::var_os("PATH"))?;

    for directory in env::split_paths(&path) {
        let candidate = directory.join(shell);

        if runtime_shims_dir
            .as_ref()
            .is_some_and(|runtime_shims_dir| candidate.starts_with(runtime_shims_dir))
        {
            continue;
        }

        if is_executable_file(&candidate) {
            return Some(candidate);
        }
    }

    fallback_shell_paths(shell).into_iter().find(|path| is_executable_file(path))
}

fn fallback_shell_paths(shell: &str) -> Vec<PathBuf> {
    match shell {
        "bash" => vec![PathBuf::from("/usr/bin/bash"), PathBuf::from("/bin/bash")],
        "sh" => vec![PathBuf::from("/usr/bin/sh"), PathBuf::from("/bin/sh")],
        _ => Vec::new(),
    }
}

fn is_executable_file(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }

    has_execute_permission(path)
}

// ─── < Private Functions: Permissions > ─────────────────────────────

#[cfg(unix)]
fn make_executable(path: &Path) -> Result<(), ConfigError> {
    use std::os::unix::fs::PermissionsExt;

    let mut permissions = fs::metadata(path)
        .map_err(|source| ConfigError::Read {
            path: path.display().to_string(),
            source,
        })?
        .permissions();

    permissions.set_mode(0o755);

    fs::set_permissions(path, permissions).map_err(|source| ConfigError::Write {
        path: path.display().to_string(),
        source,
    })
}

#[cfg(not(unix))]
fn make_executable(_path: &Path) -> Result<(), ConfigError> {
    Ok(())
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
