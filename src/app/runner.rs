// ─── < Imports > ────────────────────────────────────────────────────

use std::env;

use anyhow::Result;

use crate::cli::CliCommand;
use crate::output;

use super::command;
use super::json;

// ─── < Enums > ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OutputMode {
    Human,
    Json,
    Tui,
}

// ─── < Public Functions > ───────────────────────────────────────────

pub fn run() -> i32 {
    let args: Vec<String> = env::args().collect();

    run_with_args(&args)
}

pub fn run_with_args(args: &[String]) -> i32 {
    let output_mode = OutputMode::from_args(args);

    if output_mode.should_print_banner() {
        output::print_banner();
    }

    match run_inner(args) {
        Ok(exit_code) => exit_code,
        Err(error) => {
            output_mode.print_error(&error);

            2
        }
    }
}

// ─── < Implementations > ────────────────────────────────────────────

impl OutputMode {
    fn from_args(args: &[String]) -> Self {
        if is_json_decide_command(args) {
            Self::Json
        } else if is_tui_command(args) {
            Self::Tui
        } else {
            Self::Human
        }
    }

    fn should_print_banner(self) -> bool {
        matches!(self, Self::Human)
    }

    fn print_error(self, error: &anyhow::Error) {
        match self {
            Self::Human | Self::Tui => output::print_app_error(error),
            Self::Json => json::print_error(error),
        }
    }
}

// ─── < Private Functions > ──────────────────────────────────────────

fn run_inner(args: &[String]) -> Result<i32> {
    let cli_command = match CliCommand::from_args(args) {
        Ok(command) => command,
        Err(error) => {
            output::print_cli_error(&error);
            output::print_usage();

            return Ok(2);
        }
    };

    command::handle(cli_command)
}

fn is_json_decide_command(args: &[String]) -> bool {
    args.len() >= 3 && args[1] == "decide" && args[2] == "--json"
}

fn is_tui_command(args: &[String]) -> bool {
    args.len() >= 2 && args[1] == "tui"
}
