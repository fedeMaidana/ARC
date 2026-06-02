// ─── < Imports > ────────────────────────────────────────────────────

use thiserror::Error;

use crate::request::{Request, RequestMode};

// ─── < Errors > ─────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum CliError {
    #[error("missing action after 'check'")]
    MissingActionAfterCheck,

    #[error("missing config command")]
    MissingConfigCommand,

    #[error("unknown config command '{command}'")]
    UnknownConfigCommand { command: String },

    #[error("missing decide command format")]
    MissingDecideFormat,

    #[error("unknown decide option '{option}'")]
    UnknownDecideOption { option: String },
}

// ─── < Enums > ──────────────────────────────────────────────────────

pub enum CliCommand {
    Init,
    ConfigPath,
    ConfigCheck,
    ConfigShow,
    ConfigHelp,
    DecideJson,
    Tui,
    PolicyRequest(Request),
    Help,
}

// ─── < Implementations > ────────────────────────────────────────────

impl CliCommand {
    pub fn from_args(args: &[String]) -> Result<Self, CliError> {
        if args.len() < 2 {
            return Ok(Self::Help);
        }

        match args[1].as_str() {
            "help" | "-h" | "--help" => Ok(Self::Help),
            "init" => Ok(Self::Init),
            "config" => Self::parse_config_command(args),
            "decide" => Self::parse_decide_command(args),
            "monitor" | "tui" => Ok(Self::Tui),
            _ => Self::parse_policy_request(args),
        }
    }

    fn parse_config_command(args: &[String]) -> Result<Self, CliError> {
        if args.len() < 3 {
            return Err(CliError::MissingConfigCommand);
        }

        match args[2].as_str() {
            "path" => Ok(Self::ConfigPath),
            "check" => Ok(Self::ConfigCheck),
            "show" => Ok(Self::ConfigShow),
            "help" | "-h" | "--help" => Ok(Self::ConfigHelp),
            command => Err(CliError::UnknownConfigCommand {
                command: command.to_string(),
            }),
        }
    }

    fn parse_decide_command(args: &[String]) -> Result<Self, CliError> {
        if args.len() < 3 {
            return Err(CliError::MissingDecideFormat);
        }

        match args[2].as_str() {
            "--json" => Ok(Self::DecideJson),
            option => Err(CliError::UnknownDecideOption {
                option: option.to_string(),
            }),
        }
    }

    fn parse_policy_request(args: &[String]) -> Result<Self, CliError> {
        let request = parse_request(args)?;

        Ok(Self::PolicyRequest(request))
    }
}

// ─── < Private Functions > ──────────────────────────────────────────

fn parse_request(args: &[String]) -> Result<Request, CliError> {
    let mut mode = RequestMode::Execute;
    let mut action_index = 1;

    if args[1] == "check" {
        if args.len() < 3 {
            return Err(CliError::MissingActionAfterCheck);
        }

        mode = RequestMode::Check;
        action_index = 2;
    }

    let command_parts_start = action_index + 1;

    let command_parts = if args.len() > command_parts_start {
        args[command_parts_start..].to_vec()
    } else {
        Vec::new()
    };

    Ok(Request::new(mode, args[action_index].clone(), command_parts))
}
