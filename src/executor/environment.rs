// ─── < Imports > ────────────────────────────────────────────────────

use std::process::Command;

use crate::config::ExecutionConfig;

// ─── < Public Functions > ───────────────────────────────────────────

pub(super) fn apply_execution_environment(command: &mut Command, execution_config: &ExecutionConfig) {
    if !execution_config.inherit_environment {
        command.env_clear();
    }

    for variable in &execution_config.environment {
        if variable.name.trim().is_empty() {
            continue;
        }

        command.env(&variable.name, &variable.value);
    }

    if let Some(working_directory) = execution_config.working_directory.as_deref()
        && !working_directory.trim().is_empty()
    {
        command.current_dir(working_directory);
    }
}
