// ─── < Imports > ────────────────────────────────────────────────────

use crate::ui;

// ─── < Public Functions > ───────────────────────────────────────────

pub fn print_usage() {
    println!("{}", ui::section("Setup"));
    println!("  {}", ui::bold("arc init"));
    println!("      {}", ui::dim("Create the default Rego policy and sync detected agents"));
    println!("  {}", ui::bold("arc doctor"));
    println!("      {}", ui::dim("Check whether ARC is ready to protect registered agents"));
    println!();

    println!("{}", ui::section("Runtime settings"));
    println!("  {}", ui::bold("arc settings path"));
    println!("      {}", ui::dim("Show where runtime settings come from"));
    println!("  {}", ui::bold("arc settings check"));
    println!("      {}", ui::dim("Validate runtime defaults and ARC_* environment"));
    println!("  {}", ui::bold("arc settings show"));
    println!("      {}", ui::dim("Print the active runtime settings"));
    println!("  {}", ui::dim("arc config ... is kept as a compatibility alias"));
    println!();

    println!("{}", ui::section("Agents"));
    println!("  {}", ui::bold("arc agents scan"));
    println!("      {}", ui::dim("Detect installed known agents and possible agent candidates"));
    println!("  {}", ui::bold("arc agents scan --known"));
    println!("      {}", ui::dim("Also show known agents that were not found"));
    println!("  {}", ui::bold("arc agents sync"));
    println!("      {}", ui::dim("Persist detected known agents in the internal registry"));
    println!("  {}", ui::bold("arc agents list"));
    println!("      {}", ui::dim("Print configured agent sources"));
    println!("  {}", ui::bold("arc agents env <id> [--name <name>]"));
    println!("      {}", ui::dim("Print ARC_* exports for an agent integration"));
    println!("  {}", ui::bold("arc agents help"));
    println!("      {}", ui::dim("Show agent command usage"));
    println!();

    println!("{}", ui::section("Shims"));
    println!("  {}", ui::bold("arc shims path"));
    println!("      {}", ui::dim("Show ARC launcher and runtime shim directories"));
    println!("  {}", ui::bold("arc shims install"));
    println!("      {}", ui::dim("Install launcher shims for registered agents"));
    println!("  {}", ui::bold("arc shims list"));
    println!("      {}", ui::dim("Show registered launcher shim status"));
    println!("  {}", ui::bold("arc shims activate"));
    println!("      {}", ui::dim("Add ARC launchers to your shell profile"));
    println!("  {}", ui::bold("arc shims help"));
    println!("      {}", ui::dim("Show shim command usage"));
    println!();

    println!("{}", ui::section("Policy"));
    println!("  {}", ui::bold("arc run <command> [args...]"));
    println!("  {}", ui::bold("arc check run <command> [args...]"));
    println!("  {}", ui::bold("arc decide --json"));
    println!();

    println!("{}", ui::section("Interactive"));
    println!("  {}", ui::bold("arc monitor"));
    println!("  {}", ui::dim("arc tui"));
    println!();

    println!("{}", ui::section("Environment"));
    println!("  ARC_POLICY_ENGINE=native");
    println!("  ARC_POLICY_ENGINE=rego");
    println!("  ARC_REGO_POLICY_PATH=~/.config/arc/policies.d");
    println!("  ARC_REGO_ENTRYPOINT=data.arc.decision");
    println!("  ARC_AGENT_REGISTRY_PATH=~/.local/share/arc/agents.json");
    println!("  ARC_LAUNCHER_DIR=~/.local/share/arc/launchers");
    println!("  ARC_RUNTIME_SHIMS_DIR=~/.local/share/arc/runtime-shims");
    println!("  ARC_AGENT_SOURCES=claude-code|Claude Code|local_agent|true");
    println!("  ARC_SOURCE=claude-code");
    println!("  ARC_AUDIT_ENABLED=true");
    println!("  ARC_AUDIT_PATH=~/.local/share/arc/audit.log");
    println!();

    println!("{}", ui::section("Development"));
    println!("  cargo run -q -- init");
    println!("  cargo run -q -- doctor");
    println!("  cargo run -q -- settings path");
    println!("  cargo run -q -- settings check");
    println!("  cargo run -q -- settings show");
    println!("  cargo run -q -- agents scan");
    println!("  cargo run -q -- agents scan --known");
    println!("  cargo run -q -- agents sync");
    println!("  cargo run -q -- agents list");
    println!("  cargo run -q -- shims path");
    println!("  cargo run -q -- shims install");
    println!("  cargo run -q -- shims list");
    println!("  cargo run -q -- shims activate");
    println!("  cargo run -q -- agents env claude-code --name \"Claude Code\"");
    println!("  cargo run -q -- run ls -la");
    println!("  cargo run -q -- check run rm -rf /");
    println!("  echo '{{\"action\":\"run\",\"command\":[\"echo\",\"hola\"]}}' | cargo run -q -- decide --json");
    println!("  cargo run -q -- monitor");
    println!();

    println!(
        "{}",
        ui::dim(
            "Tip: ARC uses built-in runtime defaults, an internal agent registry, ARC_* environment variables, and optional Rego policies."
        )
    );
}

pub fn print_settings_usage() {
    println!("{}", ui::section("Runtime settings usage"));
    println!("  {}", ui::bold("arc settings path"));
    println!("  {}", ui::bold("arc settings check"));
    println!("  {}", ui::bold("arc settings show"));
    println!();
    println!("  {}", ui::dim("Compatibility aliases:"));
    println!("  {}", ui::dim("arc config path"));
    println!("  {}", ui::dim("arc config check"));
    println!("  {}", ui::dim("arc config show"));
    println!();
    println!("{}", ui::dim("Runtime settings are built from safe defaults plus ARC_* environment variables."));
}

pub fn print_agents_usage() {
    println!("{}", ui::section("Agent usage"));
    println!("  {}", ui::bold("arc agents scan"));
    println!("      {}", ui::dim("Detect installed known agents and possible agent candidates"));
    println!();
    println!("  {}", ui::bold("arc agents scan --known"));
    println!("      {}", ui::dim("Also show known agents that were not found"));
    println!();
    println!("  {}", ui::bold("arc agents sync"));
    println!("      {}", ui::dim("Persist detected known agents in the internal registry"));
    println!();
    println!("  {}", ui::bold("arc agents list"));
    println!("      {}", ui::dim("Show built-in and configured agent sources"));
    println!();
    println!("  {}", ui::bold("arc agents env <id> [options]"));
    println!("      {}", ui::dim("Print shell exports for an agent source"));
    println!();
    println!("  {}", ui::dim("Options for arc agents env:"));
    println!("  --name <name>");
    println!("      {}", ui::dim("Human-friendly agent name"));
    println!("  --kind <kind>");
    println!("      {}", ui::dim("One of: local_cli, local_agent, custom"));
    println!("  --description <text>");
    println!("      {}", ui::dim("Optional description for humans"));
    println!("  --disabled");
    println!("      {}", ui::dim("Generate the agent as disabled"));
    println!();
    println!("  {}", ui::dim("Examples:"));
    println!("  arc agents scan");
    println!("  arc agents scan --known");
    println!("  arc agents sync");
    println!("  arc agents env claude-code --name \"Claude Code\"");
    println!("  arc agents env codex --name \"Codex\"");
    println!("  arc agents env personal-agent --name \"Personal Agent\" --kind custom");
}

pub fn print_shims_usage() {
    println!("{}", ui::section("Shim usage"));
    println!("  {}", ui::bold("arc shims path"));
    println!("      {}", ui::dim("Show ARC launcher and runtime shim directories"));
    println!();
    println!("  {}", ui::bold("arc shims install"));
    println!("      {}", ui::dim("Install launcher shims for registered agents"));
    println!();
    println!("  {}", ui::bold("arc shims list"));
    println!("      {}", ui::dim("Show registered launcher shim status"));
    println!();
    println!("  {}", ui::bold("arc shims activate"));
    println!("      {}", ui::dim("Add ARC launchers to your shell profile"));
    println!();
    println!("  {}", ui::dim("Examples:"));
    println!("  arc init");
    println!("  arc shims install");
    println!("  arc shims list");
    println!("  arc shims activate");
}
