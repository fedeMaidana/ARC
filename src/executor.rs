// ─── < Imports > ────────────────────────────────────────────────────

use std::io::Read;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

use wait_timeout::ChildExt;

use crate::config::ExecutionConfig;
use crate::decision::Decision;
use crate::request::Request;

// ─── < Structs > ────────────────────────────────────────────────────

pub struct CommandExecutionReport {
    pub command_line: String,
    pub status: String,
    pub success: bool,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub stdout_truncated: bool,
    pub stderr_truncated: bool,
}

pub struct CommandTimeoutReport {
    pub command_line: String,
    pub timeout_seconds: u64,
    pub stdout: String,
    pub stderr: String,
    pub stdout_truncated: bool,
    pub stderr_truncated: bool,
}

pub struct CommandExecutionError {
    pub command_line: String,
    pub details: String,
}

#[derive(Default)]
struct CapturedOutput {
    content: String,
    truncated: bool,
}

// ─── < Enums > ──────────────────────────────────────────────────────

pub enum ExecutionReport {
    CheckMode { allowed: bool },
    SkippedDenied,
    AskRequired,
    AskDeclined,
    NoExecutionNeeded,
    MissingCommand,
    CommandFinished(CommandExecutionReport),
    CommandTimedOut(CommandTimeoutReport),
    CommandFailed(CommandExecutionError),
}

// ─── < Public Functions > ───────────────────────────────────────────

pub fn execute(request: &Request, decision: &Decision, execution_config: &ExecutionConfig) -> ExecutionReport {
    if request.is_check_mode() {
        return ExecutionReport::CheckMode {
            allowed: !decision.is_denied(),
        };
    }

    if decision.is_denied() {
        return ExecutionReport::SkippedDenied;
    }

    if decision.should_ask() {
        return ExecutionReport::AskRequired;
    }

    execute_allowed_request(request, execution_config)
}

pub fn execute_approved(request: &Request, execution_config: &ExecutionConfig) -> ExecutionReport {
    if request.is_check_mode() {
        return ExecutionReport::CheckMode { allowed: true };
    }

    execute_allowed_request(request, execution_config)
}

// ─── < Implementations > ────────────────────────────────────────────

impl ExecutionReport {
    pub fn exit_code(&self) -> i32 {
        match self {
            ExecutionReport::CheckMode { allowed } => {
                if *allowed {
                    0
                } else {
                    1
                }
            }
            ExecutionReport::SkippedDenied => 1,
            ExecutionReport::AskRequired => 1,
            ExecutionReport::AskDeclined => 1,
            ExecutionReport::NoExecutionNeeded => 0,
            ExecutionReport::MissingCommand => 2,
            ExecutionReport::CommandFinished(report) => report.exit_code,
            ExecutionReport::CommandTimedOut(_) => 124,
            ExecutionReport::CommandFailed(_) => 2,
        }
    }
}

// ─── < Private Functions > ──────────────────────────────────────────

fn execute_allowed_request(request: &Request, execution_config: &ExecutionConfig) -> ExecutionReport {
    if request.is_run_command() {
        return run_console_command(request, execution_config);
    }

    ExecutionReport::NoExecutionNeeded
}

fn run_console_command(request: &Request, execution_config: &ExecutionConfig) -> ExecutionReport {
    let Some(command_name) = request.command_name() else {
        return ExecutionReport::MissingCommand;
    };

    let command_line = request.resource.clone();

    let child_result = Command::new(command_name)
        .args(request.command_args())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

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

    let wait_result = wait_for_child(&mut child, execution_config.timeout_seconds);

    match wait_result {
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

enum CommandWaitResult {
    Finished(std::process::ExitStatus),
    TimedOut,
}

fn wait_for_child(child: &mut std::process::Child, timeout_seconds: u64) -> Result<CommandWaitResult, std::io::Error> {
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

fn capture_output<R>(reader: Option<R>, max_output_bytes: usize) -> thread::JoinHandle<CapturedOutput>
where
    R: Read + Send + 'static,
{
    thread::spawn(move || {
        let Some(reader) = reader else {
            return CapturedOutput::default();
        };

        read_capped_output(reader, max_output_bytes)
    })
}

fn read_capped_output(mut reader: impl Read, max_output_bytes: usize) -> CapturedOutput {
    let mut stored = Vec::new();
    let mut buffer = [0_u8; 8192];
    let mut truncated = false;

    loop {
        let bytes_read = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(bytes_read) => bytes_read,
            Err(_) => break,
        };

        if stored.len() < max_output_bytes {
            let remaining_capacity = max_output_bytes - stored.len();
            let bytes_to_store = bytes_read.min(remaining_capacity);

            stored.extend_from_slice(&buffer[..bytes_to_store]);

            if bytes_to_store < bytes_read {
                truncated = true;
            }
        } else {
            truncated = true;
        }
    }

    CapturedOutput {
        content: String::from_utf8_lossy(&stored).to_string(),
        truncated,
    }
}

fn join_output(reader: thread::JoinHandle<CapturedOutput>) -> CapturedOutput {
    reader.join().unwrap_or_default()
}
