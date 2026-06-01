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
