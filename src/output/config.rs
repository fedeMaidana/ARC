// ─── < Imports > ────────────────────────────────────────────────────

use std::path::Path;

use crate::config::{Config, ConfigInitResult};
use crate::ui;

use super::shared::print_list;

// ─── < Public Functions > ───────────────────────────────────────────

pub fn print_config_init_result(result: &ConfigInitResult) {
    match result {
        ConfigInitResult::Created(path) => {
            println!("{}", ui::green("✅ Config created"));
            println!("  {} {}", ui::dim("path:"), path.display());
        }
        ConfigInitResult::AlreadyExists(path) => {
            println!("{}", ui::yellow("⚠️  Config already exists"));
            println!("  {} {}", ui::dim("path:"), path.display());
        }
    }
}

pub fn print_config_path(path: &Path) {
    println!("{}", ui::section("Config path"));
    println!("  {}", path.display());
}

pub fn print_config_path_missing(default_path: &Path) {
    println!("{}", ui::yellow("⚠️  Config not found"));
    println!("  {} {}", ui::dim("default path:"), default_path.display());
    println!();
    println!("{}", ui::dim("Run: arc init"));
}

pub fn print_config(config: &Config, path: &Path) {
    println!("{}", ui::section("Config"));
    println!("  {} {}", ui::dim("path:"), path.display());
    println!("  {} {}", ui::dim("version:"), config.config_version);
    println!();

    println!("{}", ui::section("Policy"));
    println!("  {} {}", ui::bold("default action"), config.policy.default_action);

    println!("{}", ui::section("Actions"));
    print_list("allowed", &config.actions.allowed);
    print_list("blocked", &config.actions.blocked);
    print_list("need resource", &config.actions.need_resource);
    print_list("ask", &config.actions.ask);

    println!("{}", ui::section("Resources"));
    print_list("protected", &config.resources.protected);
    print_list("blocked path prefixes", &config.resources.blocked_path_prefixes);

    println!("{}", ui::section("HTTP"));
    print_list("allowed schemes", &config.http.allowed_schemes);
    println!("  {} {}", ui::bold("block localhost"), config.http.block_localhost);
    println!("  {} {}", ui::bold("block private networks"), config.http.block_private_networks);
    println!("  {} {}", ui::bold("block link local"), config.http.block_link_local);
    println!("  {} {}", ui::bold("block metadata services"), config.http.block_metadata_services);
    print_list("blocked hosts", &config.http.blocked_hosts);
    print_list("blocked CIDRs", &config.http.blocked_cidrs);
    print_list("legacy blocked targets", &config.http.blocked_targets);

    println!("{}", ui::section("Console"));
    println!("  {} {}", ui::bold("default command policy"), config.console.default_command_policy);
    println!("  {} {}", ui::bold("allow path resolution"), config.console.allow_path_resolution);
    print_list("allowed commands", &config.console.allowed_commands);
    print_list("blocked commands", &config.console.blocked_commands);
    print_list("blocked arguments", &config.console.blocked_arguments);
    print_list("ask commands", &config.console.ask_commands);
    print_console_command_rules(config);

    println!("{}", ui::section("Audit"));
    println!("  {} {}", ui::bold("enabled"), config.audit.enabled);
    println!("  {} {}", ui::bold("path"), config.audit.path);

    println!("{}", ui::section("Execution"));
    println!("  {} {}", ui::bold("timeout seconds"), config.execution.timeout_seconds);
    println!("  {} {}", ui::bold("max output bytes"), config.execution.max_output_bytes);
    println!("  {} {}", ui::bold("inherit environment"), config.execution.inherit_environment);
    println!("  {} {}", ui::bold("working directory"), config.execution.working_directory.as_deref().unwrap_or("(current process)"));

    print_execution_environment_variables(config);
}

// ─── < Private Functions > ──────────────────────────────────────────

fn print_console_command_rules(config: &Config) {
    println!("  {}", ui::bold("command rules"));

    if config.console.command_rules.is_empty() {
        println!("    {}", ui::dim("none"));
        println!();

        return;
    }

    for rule in &config.console.command_rules {
        println!("    - {}", ui::bold(&rule.name));
        println!("      mode: {}", rule.mode);

        if let Some(risk) = &rule.risk {
            println!("      risk: {risk}");
        }

        if !rule.allowed_paths.is_empty() {
            println!("      allowed paths: {}", rule.allowed_paths.join(", "));
        }

        if !rule.allowed_subcommands.is_empty() {
            println!("      allowed subcommands: {}", rule.allowed_subcommands.join(", "));
        }

        if !rule.ask_subcommands.is_empty() {
            println!("      ask subcommands: {}", rule.ask_subcommands.join(", "));
        }

        if !rule.blocked_subcommands.is_empty() {
            println!("      blocked subcommands: {}", rule.blocked_subcommands.join(", "));
        }

        if !rule.blocked_arguments.is_empty() {
            println!("      blocked arguments: {}", rule.blocked_arguments.join(", "));
        }

        if !rule.ask_arguments.is_empty() {
            println!("      ask arguments: {}", rule.ask_arguments.join(", "));
        }
    }

    println!();
}

fn print_execution_environment_variables(config: &Config) {
    println!("  {}", ui::bold("environment"));

    if config.execution.environment.is_empty() {
        println!("    {}", ui::dim("none"));
        println!();

        return;
    }

    for variable in &config.execution.environment {
        println!("    - {}={}", variable.name, ui::dim("[hidden]"));
    }

    println!();
}
