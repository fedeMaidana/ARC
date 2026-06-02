// ─── < Imports > ────────────────────────────────────────────────────

use crate::agent::AgentDiscovery;
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

pub fn print_agent_scan_results(discoveries: &[AgentDiscovery]) {
    println!("{}", ui::section("Agent scan"));

    let detected_count = discoveries.iter().filter(|discovery| discovery.is_detected()).count();

    println!("  {} {detected_count}/{}", ui::bold("detected"), discoveries.len());
    println!();

    for discovery in discoveries {
        print_agent_discovery(discovery);
    }

    println!();
    println!("{}", ui::dim("Next: ARC will use these discoveries during init to register agents and install launcher shims."));
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

fn print_agent_discovery(discovery: &AgentDiscovery) {
    if let Some(path) = discovery.path() {
        println!("  {} {}", ui::green("✅"), ui::bold(discovery.id()));
        println!("      name: {}", discovery.display_name());
        println!("      command: {}", discovery.detected_command().unwrap_or("unknown"));
        println!("      path: {}", path.display());
    } else {
        println!("  {} {}", ui::yellow("⚠️ "), ui::bold(discovery.id()));
        println!("      name: {}", discovery.display_name());
        println!("      commands checked: {}", discovery.command_names().join(", "));
        println!("      status: not found");
    }
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
