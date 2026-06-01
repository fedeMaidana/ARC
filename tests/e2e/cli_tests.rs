// ─── < Imports > ────────────────────────────────────────────────────

use std::process::{Command, Output};

// ─── < Helpers > ────────────────────────────────────────────────────

fn run_arc(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_arc"))
        .args(args)
        .output()
        .expect("failed to execute arc binary")
}

// ─── < Tests > ──────────────────────────────────────────────────────

#[test]
fn help_command_prints_usage() {
    let output = run_arc(&["help"]);

    assert_eq!(output.status.code(), Some(2));

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("ARC"));
    assert!(stdout.contains("Usage"));
    assert!(stdout.contains("arc run <command> [args...]"));
}

#[test]
fn config_help_command_prints_config_usage() {
    let output = run_arc(&["config", "help"]);

    assert_eq!(output.status.code(), Some(2));

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("Config usage"));
    assert!(stdout.contains("arc config path"));
    assert!(stdout.contains("arc config show"));
}

#[test]
fn unknown_config_command_prints_cli_error() {
    let output = run_arc(&["config", "nope"]);

    assert_eq!(output.status.code(), Some(2));

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("CLI error"));
    assert!(stdout.contains("unknown config command 'nope'"));
    assert!(stdout.contains("Usage"));
}

#[test]
fn check_without_action_prints_cli_error() {
    let output = run_arc(&["check"]);

    assert_eq!(output.status.code(), Some(2));

    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("CLI error"));
    assert!(stdout.contains("missing action after 'check'"));
    assert!(stdout.contains("Usage"));
}
