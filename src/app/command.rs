// ─── < Imports > ────────────────────────────────────────────────────

use anyhow::{Context, Result};

use crate::agent;
use crate::application::{self, ApprovalMode};
use crate::cli::{AgentEnvRequest, AgentScanRequest, CliCommand, ShellShimRequest};
use crate::config::{self, AgentSourceConfig};
use crate::doctor;
use crate::json_api;
use crate::output;
use crate::request::Request;
use crate::shims;
use crate::tui;

use super::json;

// ─── < Public Functions > ───────────────────────────────────────────

pub fn handle(command: CliCommand) -> Result<i32> {
    match command {
        CliCommand::Init => handle_init_command(),
        CliCommand::Doctor => handle_doctor_command(),
        CliCommand::SettingsPath => handle_settings_path_command(),
        CliCommand::SettingsCheck => handle_settings_check_command(),
        CliCommand::SettingsShow => handle_settings_show_command(),
        CliCommand::SettingsHelp => handle_settings_help_command(),
        CliCommand::AgentsList => handle_agents_list_command(),
        CliCommand::AgentsScan(request) => handle_agents_scan_command(request),
        CliCommand::AgentsSync => handle_agents_sync_command(),
        CliCommand::AgentsEnv(request) => handle_agents_env_command(request),
        CliCommand::AgentsHelp => handle_agents_help_command(),
        CliCommand::ShimsInstall => handle_shims_install_command(),
        CliCommand::ShimsList => handle_shims_list_command(),
        CliCommand::ShimsPath => handle_shims_path_command(),
        CliCommand::ShimsActivate => handle_shims_activate_command(),
        CliCommand::ShimsHelp => handle_shims_help_command(),
        CliCommand::InternalShellShim(request) => handle_internal_shell_shim_command(request),
        CliCommand::DecideJson => handle_decide_json_command(),
        CliCommand::Tui => handle_tui_command(),
        CliCommand::PolicyRequest(request) => handle_policy_request(request),
        CliCommand::Help => handle_help_command(),
    }
}

// ─── < Command Handlers > ───────────────────────────────────────────

fn handle_init_command() -> Result<i32> {
    let result = config::init_default_config().context("could not initialize ARC")?;

    output::print_init_result(&result);

    Ok(0)
}

fn handle_doctor_command() -> Result<i32> {
    let report = doctor::run_doctor().context("could not run ARC doctor")?;
    let exit_code = if report.has_failures() { 1 } else { 0 };

    output::print_doctor_report(&report);

    Ok(exit_code)
}

fn handle_settings_path_command() -> Result<i32> {
    let source = config::runtime_config_source_path();

    output::print_settings_source_path(&source);

    Ok(0)
}

fn handle_settings_check_command() -> Result<i32> {
    match config::load_from_default_locations() {
        Ok((_loaded_config, source)) => {
            output::print_settings_check_success(&source);
            Ok(0)
        }
        Err(error) => {
            output::print_settings_check_error(&error);
            Ok(2)
        }
    }
}

fn handle_settings_show_command() -> Result<i32> {
    let (loaded_config, source) = config::load_from_default_locations().context("could not load ARC runtime config")?;

    output::print_settings(&loaded_config, &source);

    Ok(0)
}

fn handle_settings_help_command() -> Result<i32> {
    output::print_settings_usage();

    Ok(2)
}

fn handle_agents_list_command() -> Result<i32> {
    let (loaded_config, _source) = config::load_from_default_locations().context("could not load ARC runtime config")?;

    output::print_agents(&loaded_config);

    Ok(0)
}

fn handle_agents_scan_command(request: AgentScanRequest) -> Result<i32> {
    let scan = if request.include_missing_known_agents {
        agent::scan_known_agents()
    } else {
        agent::scan_installed_agents()
    };

    output::print_agent_scan_results(&scan);

    Ok(0)
}

fn handle_agents_sync_command() -> Result<i32> {
    let registry_path = config::default_user_agent_registry_path().context("could not resolve ARC agent registry path")?;
    let report = agent::sync_agent_registry(&registry_path).context("could not sync ARC agent registry")?;

    output::print_agent_sync_report(&report);

    Ok(0)
}

fn handle_agents_env_command(request: AgentEnvRequest) -> Result<i32> {
    let source = AgentSourceConfig {
        id: request.id,
        display_name: request.display_name,
        enabled: request.enabled,
        kind: request.kind,
        description: request.description,
    };

    output::print_agent_env_exports(&source);

    Ok(0)
}

fn handle_agents_help_command() -> Result<i32> {
    output::print_agents_usage();

    Ok(2)
}

fn handle_shims_install_command() -> Result<i32> {
    let registry_path = config::default_user_agent_registry_path().context("could not resolve ARC agent registry path")?;
    let launcher_dir = config::default_user_launcher_dir().context("could not resolve ARC launcher directory")?;
    let runtime_shims_dir = config::default_user_runtime_shims_dir().context("could not resolve ARC runtime shims directory")?;
    let report = shims::install_arc_shims(&registry_path, &launcher_dir, &runtime_shims_dir).context("could not install ARC shims")?;

    output::print_shims_install_report(&report);

    Ok(0)
}

fn handle_shims_list_command() -> Result<i32> {
    let registry_path = config::default_user_agent_registry_path().context("could not resolve ARC agent registry path")?;
    let launcher_dir = config::default_user_launcher_dir().context("could not resolve ARC launcher directory")?;
    let runtime_shims_dir = config::default_user_runtime_shims_dir().context("could not resolve ARC runtime shims directory")?;
    let report = shims::list_arc_shims(&registry_path, &launcher_dir, &runtime_shims_dir).context("could not list ARC shims")?;

    output::print_shims_list_report(&report);

    Ok(0)
}

fn handle_shims_path_command() -> Result<i32> {
    let launcher_dir = config::default_user_launcher_dir().context("could not resolve ARC launcher directory")?;
    let runtime_shims_dir = config::default_user_runtime_shims_dir().context("could not resolve ARC runtime shims directory")?;

    output::print_shims_path(&launcher_dir, &runtime_shims_dir);

    Ok(0)
}

fn handle_shims_activate_command() -> Result<i32> {
    let launcher_dir = config::default_user_launcher_dir().context("could not resolve ARC launcher directory")?;
    let report = shims::activate_shell_profile(&launcher_dir).context("could not activate ARC launcher shims in shell profile")?;

    output::print_shims_activation_report(&report);

    Ok(0)
}

fn handle_shims_help_command() -> Result<i32> {
    output::print_shims_usage();

    Ok(2)
}

fn handle_internal_shell_shim_command(request: ShellShimRequest) -> Result<i32> {
    Ok(shims::execute_shell_runtime_shim(&request.shell, &request.args))
}

fn handle_decide_json_command() -> Result<i32> {
    let request = match json::read_request_from_stdin() {
        Ok(request) => request,
        Err(error) => {
            json::print_read_error(&error);

            return Ok(2);
        }
    };

    let (loaded_config, _source) = config::load_from_default_locations().context("could not load ARC runtime config")?;

    let report = application::review_action(&request, &loaded_config, "json_api", ApprovalMode::NonInteractive)?;

    let response = json_api::decision_response_from_parts(&request, report.decision(), report.execution_report());

    json::print_response(&response)?;

    Ok(report.execution_report().exit_code())
}

fn handle_tui_command() -> Result<i32> {
    tui::run().context("could not start ARC TUI")?;

    Ok(0)
}

fn handle_policy_request(request: Request) -> Result<i32> {
    let (loaded_config, _source) = config::load_from_default_locations().context("could not load ARC runtime config")?;

    let review = application::prepare_action_review(&request, &loaded_config, "cli")?;

    output::print_decision(&request, review.decision());

    let report = application::complete_action_review(&request, &loaded_config, &review, ApprovalMode::Interactive)?;

    output::print_execution_report(report.execution_report());

    Ok(report.execution_report().exit_code())
}

fn handle_help_command() -> Result<i32> {
    output::print_usage();

    Ok(2)
}
