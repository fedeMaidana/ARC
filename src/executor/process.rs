// ─── < Imports > ────────────────────────────────────────────────────

use std::process::{Child, ExitStatus};
use std::time::Duration;

use wait_timeout::ChildExt;

// ─── < Enums > ──────────────────────────────────────────────────────

pub(super) enum CommandWaitResult {
    Finished(ExitStatus),
    TimedOut,
}

// ─── < Public Functions > ───────────────────────────────────────────

pub(super) fn wait_for_child(child: &mut Child, timeout_seconds: u64) -> Result<CommandWaitResult, std::io::Error> {
    if timeout_seconds == 0 {
        let status = child.wait()?;

        return Ok(CommandWaitResult::Finished(status));
    }

    let timeout = Duration::from_secs(timeout_seconds);

    match child.wait_timeout(timeout)? {
        Some(status) => Ok(CommandWaitResult::Finished(status)),
        None => Ok(CommandWaitResult::TimedOut),
    }
}
