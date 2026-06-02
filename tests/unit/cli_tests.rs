// ─── < Imports > ────────────────────────────────────────────────────

use arc::cli::{CliCommand, CliError};

// ─── < Tests > ──────────────────────────────────────────────────────

#[test]
fn parses_help_command() {
    let args = vec!["arc".to_string(), "help".to_string()];

    let result = CliCommand::from_args(&args);

    assert!(matches!(result, Ok(CliCommand::Help)));
}

#[test]
fn parses_init_command() {
    let args = vec!["arc".to_string(), "init".to_string()];

    let result = CliCommand::from_args(&args);

    assert!(matches!(result, Ok(CliCommand::Init)));
}

#[test]
fn parses_settings_path_command() {
    let args = vec!["arc".to_string(), "settings".to_string(), "path".to_string()];

    let result = CliCommand::from_args(&args);

    assert!(matches!(result, Ok(CliCommand::ConfigPath)));
}

#[test]
fn parses_settings_check_command() {
    let args = vec!["arc".to_string(), "settings".to_string(), "check".to_string()];

    let result = CliCommand::from_args(&args);

    assert!(matches!(result, Ok(CliCommand::ConfigCheck)));
}

#[test]
fn parses_settings_show_command() {
    let args = vec!["arc".to_string(), "settings".to_string(), "show".to_string()];

    let result = CliCommand::from_args(&args);

    assert!(matches!(result, Ok(CliCommand::ConfigShow)));
}

#[test]
fn parses_config_path_compatibility_alias() {
    let args = vec!["arc".to_string(), "config".to_string(), "path".to_string()];

    let result = CliCommand::from_args(&args);

    assert!(matches!(result, Ok(CliCommand::ConfigPath)));
}

#[test]
fn parses_config_check_compatibility_alias() {
    let args = vec!["arc".to_string(), "config".to_string(), "check".to_string()];

    let result = CliCommand::from_args(&args);

    assert!(matches!(result, Ok(CliCommand::ConfigCheck)));
}

#[test]
fn parses_monitor_command() {
    let args = vec!["arc".to_string(), "monitor".to_string()];

    let result = CliCommand::from_args(&args);

    assert!(matches!(result, Ok(CliCommand::Tui)));
}

#[test]
fn parses_tui_command() {
    let args = vec!["arc".to_string(), "tui".to_string()];

    let result = CliCommand::from_args(&args);

    assert!(matches!(result, Ok(CliCommand::Tui)));
}

#[test]
fn parses_check_policy_request() {
    let args = vec![
        "arc".to_string(),
        "check".to_string(),
        "run".to_string(),
        "ls".to_string(),
        "-la".to_string(),
    ];

    let result = CliCommand::from_args(&args);

    match result {
        Ok(CliCommand::PolicyRequest(request)) => {
            assert!(request.is_check_mode());
            assert_eq!(request.action, "run");
            assert_eq!(request.resource, "ls -la");
        }
        _ => panic!("expected policy request"),
    }
}

#[test]
fn returns_error_when_check_has_no_action() {
    let args = vec!["arc".to_string(), "check".to_string()];

    let result = CliCommand::from_args(&args);

    assert!(matches!(result, Err(CliError::MissingActionAfterCheck)));
}

#[test]
fn returns_error_when_settings_has_no_subcommand() {
    let args = vec!["arc".to_string(), "settings".to_string()];

    let result = CliCommand::from_args(&args);

    assert!(matches!(result, Err(CliError::MissingRuntimeSettingsCommand)));
}

#[test]
fn returns_error_for_unknown_settings_command() {
    let args = vec!["arc".to_string(), "settings".to_string(), "nope".to_string()];

    let result = CliCommand::from_args(&args);

    match result {
        Err(CliError::UnknownRuntimeSettingsCommand { command }) => {
            assert_eq!(command, "nope");
        }
        _ => panic!("expected unknown runtime settings command error"),
    }
}

#[test]
fn returns_error_for_unknown_config_alias_command() {
    let args = vec!["arc".to_string(), "config".to_string(), "nope".to_string()];

    let result = CliCommand::from_args(&args);

    match result {
        Err(CliError::UnknownRuntimeSettingsCommand { command }) => {
            assert_eq!(command, "nope");
        }
        _ => panic!("expected unknown runtime settings command error"),
    }
}

#[test]
fn parses_decide_json_command() {
    let args = vec!["arc".to_string(), "decide".to_string(), "--json".to_string()];

    let result = CliCommand::from_args(&args);

    assert!(matches!(result, Ok(CliCommand::DecideJson)));
}
