// ─── < Imports > ────────────────────────────────────────────────────

use anyhow::{Context, Result};

use crate::cli::CliCommand;
use crate::config;
use crate::executor;
use crate::json_api;
use crate::output;
use crate::policy;
use crate::request::Request;
use crate::tui;

use super::approval;
use super::audit;
use super::json;

// ─── < Public Functions > ───────────────────────────────────────────

pub fn handle(command: CliCommand) -> Result<i32> {
    match command {
        CliCommand::Init => handle_init_command(),
        CliCommand::SettingsPath => handle_settings_path_command(),
        CliCommand::SettingsCheck => handle_settings_check_command(),
        CliCommand::SettingsShow => handle_settings_show_command(),
        CliCommand::SettingsHelp => handle_settings_help_command(),
        CliCommand::DecideJson => handle_decide_json_command(),
        CliCommand::Tui => handle_tui_command(),
        CliCommand::PolicyRequest(request) => handle_policy_request(request),
        CliCommand::Help => handle_help_command(),
    }
}

// ─── < Command Handlers > ───────────────────────────────────────────

fn handle_init_command() -> Result<i32> {
    let result = config::init_default_config().context("could not initialize ARC Rego policy")?;

    output::print_policy_init_result(&result);

    Ok(0)
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

fn handle_decide_json_command() -> Result<i32> {
    let request = match json::read_request_from_stdin() {
        Ok(request) => request,
        Err(error) => {
            json::print_read_error(&error);

            return Ok(2);
        }
    };

    let (loaded_config, _source) = config::load_from_default_locations().context("could not load ARC runtime config")?;
    let source = audit::resolve_source("json_api", &loaded_config.agents)?;

    audit::prepare(&loaded_config.audit)?;

    let decision = policy::decide(&request, &loaded_config);
    let execution_report = executor::execute(&request, &decision, &loaded_config.execution, &loaded_config.console);

    audit::record(&source, &loaded_config.audit, &request, &decision, &execution_report)?;

    let response = json_api::decision_response_from_parts(&request, &decision, &execution_report);

    json::print_response(&response)?;

    Ok(execution_report.exit_code())
}

fn handle_tui_command() -> Result<i32> {
    tui::run().context("could not start ARC TUI")?;

    Ok(0)
}

fn handle_policy_request(request: Request) -> Result<i32> {
    let (loaded_config, _source) = config::load_from_default_locations().context("could not load ARC runtime config")?;
    let source = audit::resolve_source("cli", &loaded_config.agents)?;

    audit::prepare(&loaded_config.audit)?;

    let decision = policy::decide(&request, &loaded_config);

    output::print_decision(&request, &decision);

    let execution_report = if decision.should_ask() && !request.is_check_mode() {
        approval::ask_and_maybe_execute(&request, &loaded_config.execution, &loaded_config.console)?
    } else {
        executor::execute(&request, &decision, &loaded_config.execution, &loaded_config.console)
    };

    audit::record(&source, &loaded_config.audit, &request, &decision, &execution_report)?;

    output::print_execution_report(&execution_report);

    Ok(execution_report.exit_code())
}

fn handle_help_command() -> Result<i32> {
    output::print_usage();

    Ok(2)
}
