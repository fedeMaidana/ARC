// ─── < Imports > ────────────────────────────────────────────────────

use thiserror::Error;

use crate::request::{Request, RequestMode};

// ─── < Constants > ──────────────────────────────────────────────────

const DEFAULT_AGENT_KIND: &str = "local_agent";
const SUPPORTED_AGENT_KINDS: &[&str] = &["local_cli", "local_agent", "custom"];
const RESERVED_AGENT_SOURCE_IDS: &[&str] = &["cli", "json_api"];

// ─── < Errors > ─────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum CliError {
    #[error("missing action after 'check'")]
    MissingActionAfterCheck,

    #[error("missing runtime settings command")]
    MissingRuntimeSettingsCommand,

    #[error("unknown runtime settings command '{command}'")]
    UnknownRuntimeSettingsCommand { command: String },

    #[error("missing agent command")]
    MissingAgentCommand,

    #[error("unknown agent command '{command}'")]
    UnknownAgentCommand { command: String },

    #[error("missing shims command")]
    MissingShimsCommand,

    #[error("unknown shims command '{command}'")]
    UnknownShimsCommand { command: String },

    #[error("missing internal shim command")]
    MissingInternalShimCommand,

    #[error("unknown internal shim command '{command}'")]
    UnknownInternalShimCommand { command: String },

    #[error("missing shell name for internal shell shim")]
    MissingInternalShellShimName,

    #[error("missing agent source id")]
    MissingAgentSourceId,

    #[error("missing value for agent option '{option}'")]
    MissingAgentOptionValue { option: String },

    #[error("unknown agent option '{option}'")]
    UnknownAgentOption { option: String },

    #[error("invalid agent source id '{source_id}'; use lowercase letters, numbers, '.', '_' or '-'")]
    InvalidAgentSourceId { source_id: String },

    #[error("agent source id '{source_id}' is reserved by ARC")]
    ReservedAgentSourceId { source_id: String },

    #[error("invalid agent kind '{kind}'; expected one of: local_cli, local_agent, custom")]
    InvalidAgentKind { kind: String },

    #[error("invalid {field}; values cannot contain '|' or ';'")]
    InvalidAgentEnvironmentField { field: &'static str },

    #[error("missing decide command format")]
    MissingDecideFormat,

    #[error("unknown decide option '{option}'")]
    UnknownDecideOption { option: String },
}

// ─── < Structs > ────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentEnvRequest {
    pub id: String,
    pub display_name: String,
    pub kind: String,
    pub enabled: bool,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AgentScanRequest {
    pub include_missing_known_agents: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellShimRequest {
    pub shell: String,
    pub args: Vec<String>,
}

// ─── < Enums > ──────────────────────────────────────────────────────

pub enum CliCommand {
    Init,
    Doctor,
    SettingsPath,
    SettingsCheck,
    SettingsShow,
    SettingsHelp,
    AgentsList,
    AgentsScan(AgentScanRequest),
    AgentsSync,
    AgentsEnv(AgentEnvRequest),
    AgentsHelp,
    ShimsInstall,
    ShimsList,
    ShimsPath,
    ShimsActivate,
    ShimsHelp,
    InternalShellShim(ShellShimRequest),
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
            "doctor" => Ok(Self::Doctor),
            "settings" | "config" => Self::parse_runtime_settings_command(args),
            "agents" => Self::parse_agents_command(args),
            "shims" => Self::parse_shims_command(args),
            "__arc-shim" => Self::parse_internal_shim_command(args),
            "decide" => Self::parse_decide_command(args),
            "monitor" | "tui" => Ok(Self::Tui),
            _ => Self::parse_policy_request(args),
        }
    }

    fn parse_runtime_settings_command(args: &[String]) -> Result<Self, CliError> {
        if args.len() < 3 {
            return Err(CliError::MissingRuntimeSettingsCommand);
        }

        match args[2].as_str() {
            "path" => Ok(Self::SettingsPath),
            "check" => Ok(Self::SettingsCheck),
            "show" => Ok(Self::SettingsShow),
            "help" | "-h" | "--help" => Ok(Self::SettingsHelp),
            command => Err(CliError::UnknownRuntimeSettingsCommand {
                command: command.to_string(),
            }),
        }
    }

    fn parse_agents_command(args: &[String]) -> Result<Self, CliError> {
        if args.len() < 3 {
            return Err(CliError::MissingAgentCommand);
        }

        match args[2].as_str() {
            "list" => Ok(Self::AgentsList),
            "scan" => Self::parse_agents_scan_command(args),
            "sync" => Ok(Self::AgentsSync),
            "env" => Self::parse_agents_env_command(args),
            "help" | "-h" | "--help" => Ok(Self::AgentsHelp),
            command => Err(CliError::UnknownAgentCommand {
                command: command.to_string(),
            }),
        }
    }

    fn parse_shims_command(args: &[String]) -> Result<Self, CliError> {
        if args.len() < 3 {
            return Err(CliError::MissingShimsCommand);
        }

        match args[2].as_str() {
            "install" => Ok(Self::ShimsInstall),
            "list" => Ok(Self::ShimsList),
            "path" => Ok(Self::ShimsPath),
            "activate" => Ok(Self::ShimsActivate),
            "help" | "-h" | "--help" => Ok(Self::ShimsHelp),
            command => Err(CliError::UnknownShimsCommand {
                command: command.to_string(),
            }),
        }
    }

    fn parse_internal_shim_command(args: &[String]) -> Result<Self, CliError> {
        if args.len() < 3 {
            return Err(CliError::MissingInternalShimCommand);
        }

        match args[2].as_str() {
            "shell" => Self::parse_internal_shell_shim_command(args),
            command => Err(CliError::UnknownInternalShimCommand {
                command: command.to_string(),
            }),
        }
    }

    fn parse_internal_shell_shim_command(args: &[String]) -> Result<Self, CliError> {
        if args.len() < 4 {
            return Err(CliError::MissingInternalShellShimName);
        }

        Ok(Self::InternalShellShim(ShellShimRequest {
            shell: args[3].clone(),
            args: args[4..].to_vec(),
        }))
    }

    fn parse_agents_scan_command(args: &[String]) -> Result<Self, CliError> {
        let mut include_missing_known_agents = false;
        let mut index = 3;

        while index < args.len() {
            match args[index].as_str() {
                "--known" => {
                    include_missing_known_agents = true;
                    index += 1;
                }
                "help" | "-h" | "--help" => return Ok(Self::AgentsHelp),
                option => {
                    return Err(CliError::UnknownAgentOption {
                        option: option.to_string(),
                    });
                }
            }
        }

        Ok(Self::AgentsScan(AgentScanRequest {
            include_missing_known_agents,
        }))
    }

    fn parse_agents_env_command(args: &[String]) -> Result<Self, CliError> {
        if args.len() < 4 {
            return Err(CliError::MissingAgentSourceId);
        }

        let id = args[3].trim().to_string();

        validate_agent_source_id(&id)?;

        let mut request = AgentEnvRequest {
            display_name: display_name_from_source_id(&id),
            id,
            kind: DEFAULT_AGENT_KIND.to_string(),
            enabled: true,
            description: None,
        };

        let mut index = 4;

        while index < args.len() {
            match args[index].as_str() {
                "--name" => {
                    let value = required_agent_option_value(args, index, "--name")?;
                    validate_environment_field("agent source display name", &value)?;

                    request.display_name = value;
                    index += 2;
                }
                "--kind" => {
                    let value = required_agent_option_value(args, index, "--kind")?;
                    validate_agent_kind(&value)?;

                    request.kind = value;
                    index += 2;
                }
                "--description" => {
                    let value = required_agent_option_value(args, index, "--description")?;
                    validate_environment_field("agent source description", &value)?;

                    request.description = Some(value);
                    index += 2;
                }
                "--disabled" => {
                    request.enabled = false;
                    index += 1;
                }
                "--enabled" => {
                    request.enabled = true;
                    index += 1;
                }
                option => {
                    return Err(CliError::UnknownAgentOption {
                        option: option.to_string(),
                    });
                }
            }
        }

        Ok(Self::AgentsEnv(request))
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

fn required_agent_option_value(args: &[String], option_index: usize, option: &str) -> Result<String, CliError> {
    let value = args
        .get(option_index + 1)
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| CliError::MissingAgentOptionValue {
            option: option.to_string(),
        })?;

    Ok(value)
}

fn validate_agent_source_id(source_id: &str) -> Result<(), CliError> {
    if RESERVED_AGENT_SOURCE_IDS.contains(&source_id) {
        return Err(CliError::ReservedAgentSourceId {
            source_id: source_id.to_string(),
        });
    }

    if !is_valid_agent_source_id(source_id) {
        return Err(CliError::InvalidAgentSourceId {
            source_id: source_id.to_string(),
        });
    }

    Ok(())
}

fn validate_agent_kind(kind: &str) -> Result<(), CliError> {
    if SUPPORTED_AGENT_KINDS.contains(&kind) {
        return Ok(());
    }

    Err(CliError::InvalidAgentKind { kind: kind.to_string() })
}

fn validate_environment_field(field: &'static str, value: &str) -> Result<(), CliError> {
    if value.contains('|') || value.contains(';') {
        return Err(CliError::InvalidAgentEnvironmentField { field });
    }

    Ok(())
}

fn is_valid_agent_source_id(value: &str) -> bool {
    !value.is_empty()
        && value
            .chars()
            .all(|character| character.is_ascii_lowercase() || character.is_ascii_digit() || matches!(character, '-' | '_' | '.'))
}

fn display_name_from_source_id(source_id: &str) -> String {
    let words: Vec<String> = source_id
        .split(['-', '_', '.'])
        .filter(|part| !part.is_empty())
        .map(capitalize_ascii_word)
        .collect();

    if words.is_empty() { source_id.to_string() } else { words.join(" ") }
}

fn capitalize_ascii_word(word: &str) -> String {
    let mut chars = word.chars();

    let Some(first) = chars.next() else {
        return String::new();
    };

    let mut capitalized = String::new();

    capitalized.extend(first.to_uppercase());
    capitalized.push_str(chars.as_str());

    capitalized
}
