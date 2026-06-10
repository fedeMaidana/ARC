// ─── < Imports > ────────────────────────────────────────────────────

use std::env;
use std::path::{Path, PathBuf};

use crate::agent::load_agent_registry;
use crate::config::{
    ConfigError, default_user_agent_registry_path, default_user_launcher_dir, default_user_policy_path, default_user_runtime_shims_dir,
};
use crate::shims;

// ─── < Structs > ────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoctorReport {
    checks: Vec<DoctorCheck>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoctorCheck {
    name: String,
    status: DoctorCheckStatus,
    message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DoctorCheckStatus {
    Pass,
    Warn,
    Fail,
}

// ─── < Public Functions > ───────────────────────────────────────────

pub fn run_doctor() -> Result<DoctorReport, ConfigError> {
    let policy_path = default_user_policy_path()?;
    let registry_path = default_user_agent_registry_path()?;
    let launcher_dir = default_user_launcher_dir()?;
    let runtime_shims_dir = default_user_runtime_shims_dir()?;

    let registry = load_agent_registry(&registry_path)?;
    let shim_report = shims::list_arc_shims(&registry_path, &launcher_dir, &runtime_shims_dir)?;

    let checks = vec![
        check_policy_file(&policy_path),
        check_agent_registry_file(&registry_path),
        check_registered_agents(registry.agents().len()),
        check_registered_agent_real_paths(&shim_report, &launcher_dir, &runtime_shims_dir),
        check_launcher_shims(&shim_report),
        check_runtime_shims(&shim_report),
        check_launcher_dir_in_path(&launcher_dir),
        check_agent_commands_resolve_to_launchers(&shim_report),
    ];

    Ok(DoctorReport { checks })
}

// ─── < Implementations > ────────────────────────────────────────────

impl DoctorReport {
    pub fn checks(&self) -> &[DoctorCheck] {
        &self.checks
    }

    pub fn has_failures(&self) -> bool {
        self.checks.iter().any(|check| check.status == DoctorCheckStatus::Fail)
    }

    pub fn has_warnings(&self) -> bool {
        self.checks.iter().any(|check| check.status == DoctorCheckStatus::Warn)
    }
}

impl DoctorCheck {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn status(&self) -> DoctorCheckStatus {
        self.status
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

// ─── < Private Functions: Checks > ──────────────────────────────────

fn check_policy_file(policy_path: &Path) -> DoctorCheck {
    if policy_path.is_file() {
        return check_pass("policy", format!("default policy exists at {}", policy_path.display()));
    }

    check_warn("policy", format!("default policy was not found at {}; run `arc init`", policy_path.display()))
}

fn check_agent_registry_file(registry_path: &Path) -> DoctorCheck {
    if registry_path.is_file() {
        return check_pass("agent registry", format!("agent registry exists at {}", registry_path.display()));
    }

    check_warn("agent registry", format!("agent registry was not found at {}; run `arc init`", registry_path.display()))
}

fn check_registered_agents(agent_count: usize) -> DoctorCheck {
    if agent_count > 0 {
        return check_pass("registered agents", format!("{agent_count} agent(s) registered"));
    }

    check_warn("registered agents", "no agents registered; run `arc init` or `arc agents sync`")
}

fn check_registered_agent_real_paths(report: &shims::ShimListReport, launcher_dir: &Path, runtime_shims_dir: &Path) -> DoctorCheck {
    if report.launchers().is_empty() {
        return check_warn("registered agent paths", "no registered agents to check");
    }

    let broken_paths = report
        .launchers()
        .iter()
        .filter(|launcher| is_arc_managed_path(Path::new(launcher.real_path()), launcher_dir, runtime_shims_dir))
        .map(|launcher| format!("{} points to ARC-managed path {}", launcher.command(), launcher.real_path()))
        .collect::<Vec<_>>();

    if broken_paths.is_empty() {
        return check_pass("registered agent paths", "registered agents point to real binaries");
    }

    check_fail(
        "registered agent paths",
        format!("{}; run `arc agents sync` after removing ARC launcher/runtime directories from PATH", broken_paths.join("; ")),
    )
}

fn check_launcher_shims(report: &shims::ShimListReport) -> DoctorCheck {
    if report.launchers().is_empty() {
        return check_warn("launcher shims", "no launcher shims expected because no agents are registered");
    }

    let missing = report.launchers().iter().filter(|launcher| !launcher.installed()).count();

    if missing == 0 {
        return check_pass("launcher shims", format!("all {} launcher shim(s) are installed", report.launchers().len()));
    }

    check_fail("launcher shims", format!("{missing} launcher shim(s) missing; run `arc shims install`"))
}

fn check_runtime_shims(report: &shims::ShimListReport) -> DoctorCheck {
    let missing = report.runtime_shims().iter().filter(|shim| !shim.installed()).count();

    if missing == 0 {
        return check_pass("runtime shims", format!("all {} runtime shim(s) are installed", report.runtime_shims().len()));
    }

    check_fail("runtime shims", format!("{missing} runtime shim(s) missing; run `arc shims install`"))
}

fn check_launcher_dir_in_path(launcher_dir: &Path) -> DoctorCheck {
    if path_contains_directory(launcher_dir) {
        return check_pass("launcher PATH", format!("launcher directory is in PATH: {}", launcher_dir.display()));
    }

    check_fail("launcher PATH", format!("launcher directory is not in PATH; add: export PATH=\"{}:$PATH\"", launcher_dir.display()))
}

fn check_agent_commands_resolve_to_launchers(report: &shims::ShimListReport) -> DoctorCheck {
    if report.launchers().is_empty() {
        return check_warn("agent command resolution", "no registered launchers to check");
    }

    let mut broken = Vec::new();

    for launcher in report.launchers() {
        let resolved = first_command_in_path(launcher.command());

        if resolved.as_deref() != Some(launcher.shim_path()) {
            broken.push(format!("{} should resolve to {}", launcher.command(), launcher.shim_path().display()));
        }
    }

    if broken.is_empty() {
        return check_pass("agent command resolution", "registered agent commands resolve to ARC launchers");
    }

    check_fail("agent command resolution", broken.join("; "))
}

// ─── < Private Functions: Helpers > ─────────────────────────────────

fn check_pass(name: &str, message: impl Into<String>) -> DoctorCheck {
    DoctorCheck {
        name: name.to_string(),
        status: DoctorCheckStatus::Pass,
        message: message.into(),
    }
}

fn check_warn(name: &str, message: impl Into<String>) -> DoctorCheck {
    DoctorCheck {
        name: name.to_string(),
        status: DoctorCheckStatus::Warn,
        message: message.into(),
    }
}

fn check_fail(name: &str, message: impl Into<String>) -> DoctorCheck {
    DoctorCheck {
        name: name.to_string(),
        status: DoctorCheckStatus::Fail,
        message: message.into(),
    }
}

fn is_arc_managed_path(path: &Path, launcher_dir: &Path, runtime_shims_dir: &Path) -> bool {
    path.starts_with(launcher_dir) || path.starts_with(runtime_shims_dir)
}

fn path_contains_directory(directory: &Path) -> bool {
    let Some(path) = env::var_os("PATH") else {
        return false;
    };

    env::split_paths(&path).any(|path_dir| path_dir == directory)
}

fn first_command_in_path(command: &str) -> Option<PathBuf> {
    let path = env::var_os("PATH")?;

    env::split_paths(&path)
        .map(|directory| directory.join(command))
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
