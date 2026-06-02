// ─── < Imports > ────────────────────────────────────────────────────

use std::fs;
use std::path::PathBuf;

use super::{ConfigError, default_user_policy_path};

// ─── < Constants > ──────────────────────────────────────────────────

const DEFAULT_REGO_POLICY_CONTENT: &str = r#"package arc

import rego.v1

default decision := {
	"status": "deny",
	"reason_code": "action_not_configured",
	"risk": "high",
}

decision := {
	"status": "allow",
	"reason_code": "action_allowed",
	"risk": "low",
} if {
	input.request.action == "run"
	input.request.command.name == "echo"
}

decision := {
	"status": "allow",
	"reason_code": "action_allowed",
	"risk": "low",
} if {
	input.request.action == "run"
	input.request.command.name == "git"
	input.request.command.args[0] == "status"
}

decision := {
	"status": "ask",
	"reason_code": "console_subcommand_requires_approval",
	"risk": "low",
} if {
	input.request.action == "run"
	input.request.command.name == "git"
	input.request.command.args[0] == "commit"
}

decision := {
	"status": "deny",
	"reason_code": "console_subcommand_blocked",
	"risk": "critical",
} if {
	input.request.action == "run"
	input.request.command.name == "git"
	input.request.command.args[0] == "push"
}

decision := {
	"status": "deny",
	"reason_code": "console_command_blocked",
	"risk": "critical",
} if {
	input.request.action == "run"
	input.request.command.name == "rm"
}

decision := {
	"status": "deny",
	"reason_code": "console_argument_blocked",
	"risk": "critical",
} if {
	input.request.action == "run"
	input.request.command.parts[_] == "-rf"
}

decision := {
	"status": "allow",
	"reason_code": "action_allowed",
	"risk": "low",
} if {
	input.request.action == "http_get"
	startswith(input.request.resource, "https://")
}

decision := {
	"status": "deny",
	"reason_code": "invalid_http_url",
	"risk": "medium",
} if {
	input.request.action == "http_get"
	not startswith(input.request.resource, "https://")
}
"#;

// ─── < Enums > ──────────────────────────────────────────────────────

pub enum ConfigInitResult {
    Created(PathBuf),
    AlreadyExists(PathBuf),
}

// ─── < Public Functions > ───────────────────────────────────────────

pub fn init_default_config() -> Result<ConfigInitResult, ConfigError> {
    let path = default_user_policy_path()?;

    if path.exists() {
        return Ok(ConfigInitResult::AlreadyExists(path));
    }

    let Some(parent_dir) = path.parent() else {
        return Err(ConfigError::MissingParent {
            path: path.display().to_string(),
        });
    };

    fs::create_dir_all(parent_dir).map_err(|source| ConfigError::CreateDir {
        path: parent_dir.display().to_string(),
        source,
    })?;

    fs::write(&path, DEFAULT_REGO_POLICY_CONTENT).map_err(|source| ConfigError::Write {
        path: path.display().to_string(),
        source,
    })?;

    Ok(ConfigInitResult::Created(path))
}
