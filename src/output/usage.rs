// ─── < Imports > ────────────────────────────────────────────────────

use crate::ui;

// ─── < Public Functions > ───────────────────────────────────────────

pub fn print_usage() {
    println!("{}", ui::section("Setup"));
    println!("  {}", ui::bold("arc init"));
    println!("      {}", ui::dim("Create the default Rego policy file"));
    println!();

    println!("{}", ui::section("Runtime settings"));
    println!("  {}", ui::bold("arc config path"));
    println!("      {}", ui::dim("Show where runtime settings come from"));
    println!("  {}", ui::bold("arc config check"));
    println!("      {}", ui::dim("Validate runtime defaults and ARC_* environment"));
    println!("  {}", ui::bold("arc config show"));
    println!("      {}", ui::dim("Print the active runtime settings"));
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
    println!("  ARC_AUDIT_ENABLED=true");
    println!("  ARC_AUDIT_PATH=~/.local/share/arc/audit.log");
    println!();

    println!("{}", ui::section("Development"));
    println!("  cargo run -q -- init");
    println!("  cargo run -q -- config path");
    println!("  cargo run -q -- config check");
    println!("  cargo run -q -- config show");
    println!("  cargo run -q -- run ls -la");
    println!("  cargo run -q -- check run rm -rf /");
    println!("  echo '{{\"action\":\"run\",\"command\":[\"echo\",\"hola\"]}}' | cargo run -q -- decide --json");
    println!("  cargo run -q -- monitor");
    println!();

    println!("{}", ui::dim("Tip: ARC uses built-in runtime defaults, ARC_* environment variables, and optional Rego policies."));
}

pub fn print_config_usage() {
    println!("{}", ui::section("Runtime settings usage"));
    println!("  {}", ui::bold("arc config path"));
    println!("  {}", ui::bold("arc config check"));
    println!("  {}", ui::bold("arc config show"));
    println!();
    println!("{}", ui::dim("Runtime settings are built from safe defaults plus ARC_* environment variables."));
}
