// ─── < Imports > ────────────────────────────────────────────────────

use std::env;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};
use std::time::Duration;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use wait_timeout::ChildExt;

use crate::decision::{Decision, DecisionReason, DecisionStatus, RiskLevel};
use crate::request::Request;

use super::engine::PolicyEngine;
use super::input::PolicyInput;
use super::output::PolicyDecision;

// ─── < Errors > ─────────────────────────────────────────────────────

#[derive(Debug, Error)]
enum RegoPolicyError {
    #[error("could not serialize Rego input")]
    SerializeInput {
        #[source]
        source: serde_json::Error,
    },

    #[error("could not write Rego input")]
    WriteInput {
        #[source]
        source: std::io::Error,
    },

    #[error("could not start OPA")]
    StartOpa {
        #[source]
        source: std::io::Error,
    },

    #[error("OPA evaluation timed out")]
    TimedOut,

    #[error("OPA evaluation failed")]
    EvaluationFailed,

    #[error("OPA returned invalid JSON")]
    InvalidJson {
        #[source]
        source: serde_json::Error,
    },

    #[error("OPA did not return a decision")]
    MissingDecision,

    #[error("OPA returned an invalid decision")]
    InvalidDecision,
}

// ─── < Structs > ────────────────────────────────────────────────────

#[derive(Debug, Default, Clone, Copy)]
pub struct RegoPolicyEngine;

#[derive(Debug, Serialize)]
struct RegoInput<'a> {
    request: RegoRequest<'a>,
}

#[derive(Debug, Serialize)]
struct RegoRequest<'a> {
    mode: &'static str,
    action: &'a str,
    resource: Option<&'a str>,
    command: Option<RegoCommand<'a>>,
}

#[derive(Debug, Serialize)]
struct RegoCommand<'a> {
    name: &'a str,
    args: &'a [String],
    parts: &'a [String],
}

#[derive(Debug, Deserialize)]
struct OpaEvalOutput {
    #[serde(default)]
    result: Vec<OpaEvalResult>,
}

#[derive(Debug, Deserialize)]
struct OpaEvalResult {
    #[serde(default)]
    expressions: Vec<OpaExpression>,
}

#[derive(Debug, Deserialize)]
struct OpaExpression {
    value: RegoDecisionOutput,
}

#[derive(Debug, Clone, Deserialize)]
struct RegoDecisionOutput {
    status: String,
    reason_code: String,
    risk: String,
}

// ─── < Implementations > ────────────────────────────────────────────

impl RegoPolicyEngine {
    pub fn new() -> Self {
        Self
    }
}

impl PolicyEngine for RegoPolicyEngine {
    fn decide(&self, input: PolicyInput<'_>) -> PolicyDecision {
        match evaluate_rego(input) {
            Ok(decision) => PolicyDecision::new(decision),
            Err(_error) => PolicyDecision::new(Decision::deny_with_risk(DecisionReason::PolicyEngineFailed, RiskLevel::Critical)),
        }
    }
}

impl RegoDecisionOutput {
    fn into_decision(self) -> Result<Decision, RegoPolicyError> {
        let Some(status) = DecisionStatus::from_text(&self.status) else {
            return Err(RegoPolicyError::InvalidDecision);
        };

        let Some(reason) = DecisionReason::from_code(&self.reason_code) else {
            return Err(RegoPolicyError::InvalidDecision);
        };

        let Some(risk) = RiskLevel::from_text(&self.risk) else {
            return Err(RegoPolicyError::InvalidDecision);
        };

        let decision = match status {
            DecisionStatus::Allow => Decision::allow_with_risk(reason, risk),
            DecisionStatus::Deny => Decision::deny_with_risk(reason, risk),
            DecisionStatus::Ask => Decision::ask_with_risk(reason, risk),
        };

        Ok(decision)
    }
}

// ─── < Private Functions > ──────────────────────────────────────────

fn evaluate_rego(input: PolicyInput<'_>) -> Result<Decision, RegoPolicyError> {
    let serialized_input = serialize_rego_input(input.request())?;

    let output = run_opa(input, &serialized_input)?;

    parse_opa_output(&output)
}

fn serialize_rego_input(request: &Request) -> Result<Vec<u8>, RegoPolicyError> {
    let input = RegoInput {
        request: RegoRequest {
            mode: request_mode_text(request),
            action: &request.action,
            resource: request.has_resource().then_some(request.resource.as_str()),
            command: rego_command(request),
        },
    };

    serde_json::to_vec(&input).map_err(|source| RegoPolicyError::SerializeInput { source })
}

fn run_opa(input: PolicyInput<'_>, serialized_input: &[u8]) -> Result<Output, RegoPolicyError> {
    let policy_path = expand_home(&input.config().policy.rego.policy_path);
    let timeout = Duration::from_secs(input.config().policy.rego.timeout_seconds);

    let mut child = Command::new("opa")
        .arg("eval")
        .arg("--format")
        .arg("json")
        .arg("--data")
        .arg(policy_path)
        .arg("--stdin-input")
        .arg(&input.config().policy.rego.entrypoint)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|source| RegoPolicyError::StartOpa { source })?;

    if let Some(mut stdin) = child.stdin.take()
        && let Err(source) = stdin.write_all(serialized_input)
    {
        let _ = child.kill();
        let _ = child.wait_with_output();

        return Err(RegoPolicyError::WriteInput { source });
    }

    match child.wait_timeout(timeout).map_err(|source| RegoPolicyError::StartOpa { source })? {
        Some(_status) => {
            let output = child.wait_with_output().map_err(|source| RegoPolicyError::StartOpa { source })?;

            if output.status.success() {
                Ok(output)
            } else {
                Err(RegoPolicyError::EvaluationFailed)
            }
        }
        None => {
            let _ = child.kill();
            let _ = child.wait_with_output();

            Err(RegoPolicyError::TimedOut)
        }
    }
}

fn parse_opa_output(output: &Output) -> Result<Decision, RegoPolicyError> {
    let opa_output: OpaEvalOutput = serde_json::from_slice(&output.stdout).map_err(|source| RegoPolicyError::InvalidJson { source })?;

    let Some(expression) = opa_output.result.first().and_then(|result| result.expressions.first()) else {
        return Err(RegoPolicyError::MissingDecision);
    };

    expression.value.clone().into_decision()
}

fn rego_command(request: &Request) -> Option<RegoCommand<'_>> {
    let name = request.command_name()?;

    Some(RegoCommand {
        name,
        args: request.command_args(),
        parts: &request.command_parts,
    })
}

fn request_mode_text(request: &Request) -> &'static str {
    if request.is_check_mode() { "check" } else { "execute" }
}

fn expand_home(path: &str) -> PathBuf {
    let Some(relative_path) = path.strip_prefix("~/") else {
        return PathBuf::from(path);
    };

    match env::var("HOME") {
        Ok(home) => PathBuf::from(home).join(relative_path),
        Err(_) => PathBuf::from(path),
    }
}
