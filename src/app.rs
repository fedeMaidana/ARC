// ─── < Imports > ────────────────────────────────────────────────────

use anyhow::{Context, Result};
use std::env;
use std::io::{self, Read};

use crate::ask::{self, AskAnswer};
use crate::audit::{self, AuditEvent};
use crate::cli::CliCommand;
use crate::config;
use crate::executor::{self, ExecutionReport};
use crate::json_api;
use crate::output;
use crate::policy;
use crate::request::Request;

// ─── < Public Functions > ───────────────────────────────────────────

pub fn run() -> i32 {
    let args: Vec<String> = env::args().collect();

    run_with_args(&args)
}

pub fn run_with_args(args: &[String]) -> i32 {
    let is_json_command = is_json_decide_command(args);

    if !is_json_command {
        output::print_banner();
    }

    match run_inner(args) {
        Ok(exit_code) => exit_code,
        Err(error) => {
            if is_json_command {
                print_json_error(&error);
            } else {
                output::print_app_error(&error);
            }

            2
        }
    }
}

// ─── < App Flow > ───────────────────────────────────────────────────

fn run_inner(args: &[String]) -> Result<i32> {
    let command = match CliCommand::from_args(args) {
        Ok(command) => command,
        Err(error) => {
            output::print_cli_error(&error);
            output::print_usage();
            return Ok(2);
        }
    };

    match command {
        CliCommand::Init => handle_init_command(),
        CliCommand::ConfigPath => handle_config_path_command(),
        CliCommand::ConfigShow => handle_config_show_command(),
        CliCommand::ConfigHelp => {
            output::print_config_usage();
            Ok(2)
        }
        CliCommand::DecideJson => handle_decide_json_command(),
        CliCommand::PolicyRequest(request) => handle_policy_request(request),
        CliCommand::Help => {
            output::print_usage();
            Ok(2)
        }
    }
}

// ─── < Command Handlers > ───────────────────────────────────────────

fn handle_init_command() -> Result<i32> {
    let result = config::init_default_config().context("could not initialize ARC config")?;

    output::print_config_init_result(&result);

    Ok(0)
}

fn handle_config_path_command() -> Result<i32> {
    if let Some(path) = config::resolve_config_path() {
        output::print_config_path(&path);
        return Ok(0);
    }

    let default_path = config::default_user_config_path().context("could not resolve default config path")?;

    output::print_config_path_missing(&default_path);

    Ok(1)
}

fn handle_config_show_command() -> Result<i32> {
    let (loaded_config, path) = config::load_from_default_locations().context("could not load ARC config")?;

    output::print_config(&loaded_config, &path);

    Ok(0)
}

fn handle_decide_json_command() -> Result<i32> {
    let mut input = String::new();

    io::stdin()
        .read_to_string(&mut input)
        .context("could not read JSON request from stdin")?;

    let request = json_api::request_from_json(&input).context("could not parse JSON request")?;

    let (loaded_config, _path) = config::load_from_default_locations().context("could not load ARC config")?;

    audit::ensure_audit_log_is_writable(&loaded_config.audit).context("could not prepare audit log")?;

    let decision = policy::decide(&request, &loaded_config);
    let execution_report = executor::execute(&request, &decision, &loaded_config.execution);

    let audit_source = audit_source_or("json_api");
    let audit_event = AuditEvent::from_parts(audit_source, &request, &decision, &execution_report);

    audit::record_event(&loaded_config.audit, &audit_event).context("could not write audit log")?;

    let response = json_api::decision_response_from_parts(&request, &decision, &execution_report);

    print_json_response(&response)?;

    Ok(execution_report.exit_code())
}

fn handle_policy_request(request: Request) -> Result<i32> {
    let (loaded_config, _path) = config::load_from_default_locations().context("could not load ARC config")?;

    audit::ensure_audit_log_is_writable(&loaded_config.audit).context("could not prepare audit log")?;

    let decision = policy::decide(&request, &loaded_config);
    output::print_decision(&request, &decision);

    let execution_report = if decision.should_ask() && !request.is_check_mode() {
        ask_and_maybe_execute(&request, &loaded_config.execution)?
    } else {
        executor::execute(&request, &decision, &loaded_config.execution)
    };

    let audit_source = audit_source_or("cli");
    let audit_event = AuditEvent::from_parts(audit_source, &request, &decision, &execution_report);

    audit::record_event(&loaded_config.audit, &audit_event).context("could not write audit log")?;

    output::print_execution_report(&execution_report);

    Ok(execution_report.exit_code())
}

// ─── < Private Functions > ──────────────────────────────────────────

fn ask_and_maybe_execute(request: &Request, execution_config: &config::ExecutionConfig) -> Result<ExecutionReport> {
    let prompt = ask_prompt(request);

    let answer = ask::ask_yes_no(&prompt).context("could not ask for request approval")?;

    match answer {
        AskAnswer::Yes => Ok(executor::execute_approved(request, execution_config)),
        AskAnswer::No => Ok(ExecutionReport::AskDeclined),
    }
}

fn ask_prompt(request: &Request) -> String {
    if request.has_resource() {
        return format!("ARC wants to execute `{}`", request.resource);
    }

    format!("ARC wants to perform `{}`", request.action)
}

fn is_json_decide_command(args: &[String]) -> bool {
    args.len() >= 3 && args[1] == "decide" && args[2] == "--json"
}

fn print_json_response(response: &json_api::JsonDecisionResponse) -> Result<()> {
    let serialized = serde_json::to_string(response).context("could not serialize JSON response")?;

    println!("{serialized}");

    Ok(())
}

fn print_json_error(error: &impl std::fmt::Display) {
    let response = json_api::error_response(error);

    match serde_json::to_string(&response) {
        Ok(serialized) => println!("{serialized}"),
        Err(_) => println!(r#"{{"ok":false,"error":"could not serialize JSON error"}}"#),
    }
}

fn audit_source_or(default_source: &str) -> String {
    env::var("ARC_SOURCE")
        .ok()
        .map(|source| source.trim().to_string())
        .filter(|source| !source.is_empty())
        .unwrap_or_else(|| default_source.to_string())
}
