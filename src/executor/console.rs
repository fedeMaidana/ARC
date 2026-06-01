// ─── < Imports > ────────────────────────────────────────────────────

use std::process::{Command, Stdio};

use crate::config::{ConsoleConfig, ExecutionConfig};
use crate::request::Request;

use super::command::resolve_command_path;
use super::environment::apply_execution_environment;
use super::model::{CommandExecutionError, CommandExecutionReport, CommandTimeoutReport, ExecutionReport};
use super::output::{capture_output, join_output};
use super::process::{CommandWaitResult, wait_for_child};

// ─── < Public Functions > ───────────────────────────────────────────

pub fn run(request: &Request, execution_config: &ExecutionConfig, console_config: &ConsoleConfig) -> ExecutionReport {
    let Some(command_name) = request.command_name() else {
        return ExecutionReport::MissingCommand;
    };

    let command_line = request.resource.clone();

    let command_path = match resolve_command_path(command_name, console_config) {
        Ok(command_path) => command_path,
        Err(details) => {
            return ExecutionReport::CommandFailed(CommandExecutionError { command_line, details });
        }
    };

    let mut command = Command::new(command_path);

    command.args(request.command_args()).stdout(Stdio::piped()).stderr(Stdio::piped());

    apply_execution_environment(&mut command, execution_config);

    let child_result = command.spawn();

    let mut child = match child_result {
        Ok(child) => child,
        Err(error) => {
            return ExecutionReport::CommandFailed(CommandExecutionError {
                command_line,
                details: error.to_string(),
            });
        }
    };

    let stdout_reader = capture_output(child.stdout.take(), execution_config.max_output_bytes);
    let stderr_reader = capture_output(child.stderr.take(), execution_config.max_output_bytes);

    match wait_for_child(&mut child, execution_config.timeout_seconds) {
        Ok(CommandWaitResult::Finished(status)) => {
            let stdout = join_output(stdout_reader);
            let stderr = join_output(stderr_reader);

            ExecutionReport::CommandFinished(CommandExecutionReport {
                command_line,
                status: status.to_string(),
                success: status.success(),
                exit_code: status.code().unwrap_or(1),
                stdout: stdout.content,
                stderr: stderr.content,
                stdout_truncated: stdout.truncated,
                stderr_truncated: stderr.truncated,
            })
        }
        Ok(CommandWaitResult::TimedOut) => {
            let _ = child.kill();
            let _ = child.wait();

            let stdout = join_output(stdout_reader);
            let stderr = join_output(stderr_reader);

            ExecutionReport::CommandTimedOut(CommandTimeoutReport {
                command_line,
                timeout_seconds: execution_config.timeout_seconds,
                stdout: stdout.content,
                stderr: stderr.content,
                stdout_truncated: stdout.truncated,
                stderr_truncated: stderr.truncated,
            })
        }
        Err(error) => ExecutionReport::CommandFailed(CommandExecutionError {
            command_line,
            details: error.to_string(),
        }),
    }
}
