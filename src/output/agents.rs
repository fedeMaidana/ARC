// ─── < Imports > ────────────────────────────────────────────────────

use crate::agent::{AgentCandidate, AgentDiscovery, AgentRegistrySyncReport, AgentScan, MissingKnownAgent};
use crate::config::{AgentSourceConfig, Config};
use crate::ui;

// ─── < Public Functions > ───────────────────────────────────────────

pub fn print_agents(config: &Config) {
    println!("{}", ui::section("Agents"));
    println!("  {} {}", ui::bold("allow unknown sources"), config.agents.allow_unknown_sources);
    println!("  {}", ui::bold("sources"));

    if config.agents.sources.is_empty() {
        println!("    {}", ui::dim("none"));
        println!();

        return;
    }

    for source in &config.agents.sources {
        let enabled = if source.enabled { "enabled" } else { "disabled" };

        println!("    - {}", ui::bold(&source.id));
        println!("      name: {}", source.display_name);
        println!("      kind: {}", source.kind);
        println!("      status: {enabled}");

        if let Some(description) = &source.description {
            println!("      description: {description}");
        }
    }

    println!();
    println!("{}", ui::dim("Tip: start an agent with ARC_SOURCE=<agent-id> so ARC can audit where requests came from."));
}

pub fn print_agent_scan_results(scan: &AgentScan) {
    println!("{}", ui::section("Agent scan"));
    println!("  {} {}", ui::bold("detected known agents"), scan.detected_agents().len());
    println!("  {} {}", ui::bold("possible agents"), scan.candidate_agents().len());

    if !scan.missing_known_agents().is_empty() {
        println!("  {} {}", ui::bold("missing known agents"), scan.missing_known_agents().len());
    }

    println!();

    print_detected_agents(scan.detected_agents());

    if !scan.missing_known_agents().is_empty() {
        println!();
        print_missing_known_agents(scan.missing_known_agents());
    }

    println!();
    print_candidate_agents(scan.candidate_agents());

    println!();
    println!("{}", ui::dim("Known detected agents can be registered automatically during arc init."));
    println!("{}", ui::dim("Possible agents require confirmation before ARC treats them as trusted agents."));
}

pub fn print_agent_sync_report(report: &AgentRegistrySyncReport) {
    println!("{}", ui::section("Agent registry synced"));
    println!("  {} {}", ui::bold("registry path"), report.registry_path());
    println!("  {} {}", ui::bold("detected"), report.detected_count());
    println!("  {} {}", ui::bold("registered"), report.registered_count());
    println!("  {} {}", ui::bold("added"), report.added_count());
    println!("  {} {}", ui::bold("updated"), report.updated_count());
    println!();
    println!("{}", ui::dim("Run `arc agents list` to see the active registered sources."));
}

pub fn print_agent_env_exports(source: &AgentSourceConfig) {
    let entry = format_agent_source_environment_entry(source);

    println!("{}", ui::section("Agent environment"));
    println!("  {}", ui::dim("Copy these exports before starting the agent:"));
    println!();
    println!("export ARC_AGENT_SOURCES={}", shell_single_quote(&entry));
    println!("export ARC_SOURCE={}", shell_single_quote(&source.id));
    println!();
    println!("{}", ui::dim("For multiple agents, join ARC_AGENT_SOURCES entries with ';'."));
}

// ─── < Private Functions > ──────────────────────────────────────────

fn print_detected_agents(agents: &[AgentDiscovery]) {
    println!("  {}", ui::bold("Detected agents"));

    if agents.is_empty() {
        println!("    {}", ui::dim("none"));
        return;
    }

    for agent in agents {
        print_detected_agent(agent);
    }
}

fn print_missing_known_agents(agents: &[MissingKnownAgent]) {
    println!("  {}", ui::bold("Known agents not found"));

    if agents.is_empty() {
        println!("    {}", ui::dim("none"));
        return;
    }

    for agent in agents {
        print_missing_known_agent(agent);
    }
}

fn print_candidate_agents(candidates: &[AgentCandidate]) {
    println!("  {}", ui::bold("Possible agents"));

    if candidates.is_empty() {
        println!("    {}", ui::dim("none"));
        return;
    }

    for candidate in candidates {
        print_candidate_agent(candidate);
    }
}

fn print_detected_agent(agent: &AgentDiscovery) {
    println!("    {} {}", ui::green("✅"), ui::bold(agent.id()));
    println!("      name: {}", agent.display_name());
    println!("      command: {}", agent.detected_command());
    println!("      path: {}", agent.path().display());
}

fn print_missing_known_agent(agent: &MissingKnownAgent) {
    println!("    {} {}", ui::yellow("⚠️ "), ui::bold(agent.id()));
    println!("      name: {}", agent.display_name());
    println!("      commands checked: {}", agent.command_names().join(", "));
    println!("      status: not found");
}

fn print_candidate_agent(candidate: &AgentCandidate) {
    println!("    {} {}", ui::yellow("?"), ui::bold(candidate.command_name()));
    println!("      path: {}", candidate.path().display());
    println!("      reason: {}", candidate.reason());
}

fn format_agent_source_environment_entry(source: &AgentSourceConfig) -> String {
    let mut parts = vec![
        source.id.clone(),
        source.display_name.clone(),
        source.kind.clone(),
        source.enabled.to_string(),
    ];

    if let Some(description) = source.description.as_deref().filter(|description| !description.is_empty()) {
        parts.push(description.to_string());
    }

    parts.join("|")
}

fn shell_single_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}
