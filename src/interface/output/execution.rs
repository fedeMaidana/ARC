// ─── < Imports > ────────────────────────────────────────────────────

use crate::executor::ExecutionReport;
use crate::ui;

// ─── < Public Functions > ───────────────────────────────────────────

pub fn print_execution_report(report: &ExecutionReport) {
    match report {
        ExecutionReport::CheckMode { .. } => print_check_mode(),
        ExecutionReport::SkippedDenied => print_execution_skipped(),
        ExecutionReport::AskRequired => print_ask_required(),
        ExecutionReport::AskDeclined => print_ask_declined(),
        ExecutionReport::NoExecutionNeeded => {}
        ExecutionReport::MissingCommand => print_missing_command_error(),
        ExecutionReport::CommandFinished(command_report) => {
            println!();
            println!("{}", ui::section("Execution"));
            println!("  {} {}", ui::dim("running:"), ui::bold(&command_report.command_line));

            let status = if command_report.success {
                ui::green(&command_report.status)
            } else {
                ui::red(&command_report.status)
            };

            println!("  {} {}", ui::dim("status: "), status);

            print_captured_output("Output", &command_report.stdout, command_report.stdout_truncated);

            print_captured_output("Error output", &command_report.stderr, command_report.stderr_truncated);
        }
        ExecutionReport::CommandTimedOut(timeout_report) => {
            println!();
            println!("{}", ui::red("⏱️  Execution timed out"));
            println!("  {} {}", ui::dim("command:"), ui::bold(&timeout_report.command_line));
            println!("  {} {}s", ui::dim("timeout:"), timeout_report.timeout_seconds);

            print_captured_output("Output", &timeout_report.stdout, timeout_report.stdout_truncated);

            print_captured_output("Error output", &timeout_report.stderr, timeout_report.stderr_truncated);
        }
        ExecutionReport::CommandFailed(error_report) => {
            println!();
            println!("{}", ui::red("⛔ Execution error"));
            println!("  {} {}", ui::dim("command:"), ui::bold(&error_report.command_line));
            println!("  {} failed to execute command", ui::dim("error:  "));
            println!("  {} {}", ui::dim("details:"), error_report.details);
        }
    }
}

// ─── < Private Functions > ──────────────────────────────────────────

fn print_check_mode() {
    println!();
    println!("{}", ui::yellow("🔎 Check mode"));
    println!("  {}", ui::dim("Execution skipped. No command was run."));
}

fn print_execution_skipped() {
    println!();
    println!("{}", ui::yellow("⏭️  Execution skipped"));
    println!("  {}", ui::dim("ARC denied this request."));
}

fn print_ask_required() {
    println!();
    println!("{}", ui::yellow("⏭️  Execution skipped"));
    println!("  {}", ui::dim("This request requires ask approval."));
}

fn print_ask_declined() {
    println!();
    println!("{}", ui::red("Operation cancelled"));
}

fn print_missing_command_error() {
    println!();
    println!("{}", ui::red("⛔ Execution error"));
    println!("  error: missing command");
}

fn print_captured_output(title: &str, content: &str, truncated: bool) {
    if content.trim().is_empty() && !truncated {
        return;
    }

    println!();
    println!("{}", ui::section(title));

    if !content.trim().is_empty() {
        println!("{}", ui::indent_lines(content, 4));
    }

    if truncated {
        println!("    {}", ui::yellow("[output truncated]"));
    }
}
