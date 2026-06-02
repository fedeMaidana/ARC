// ─── < Imports > ────────────────────────────────────────────────────

use anyhow::{Context, Result};

use crate::ask::{self, AskAnswer};
use crate::config::{ConsoleConfig, ExecutionConfig};
use crate::executor::{self, ExecutionReport};
use crate::request::Request;

// ─── < Public Functions > ───────────────────────────────────────────

pub fn ask_and_maybe_execute(
    request: &Request,
    execution_config: &ExecutionConfig,
    console_config: &ConsoleConfig,
) -> Result<ExecutionReport> {
    let prompt = approval_prompt(request);

    let answer = ask::ask_yes_no(&prompt).context("could not ask for request approval")?;

    match answer {
        AskAnswer::Yes => Ok(executor::execute_approved(request, execution_config, console_config)),
        AskAnswer::No => Ok(ExecutionReport::AskDeclined),
    }
}

// ─── < Private Functions > ──────────────────────────────────────────

fn approval_prompt(request: &Request) -> String {
    if request.has_resource() {
        return format!("ARC wants to execute `{}`", request.resource);
    }

    format!("ARC wants to perform `{}`", request.action)
}
