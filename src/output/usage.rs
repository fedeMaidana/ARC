// ─── < Imports > ────────────────────────────────────────────────────

use crate::ui;

// ─── < Public Functions > ───────────────────────────────────────────

pub fn print_usage() {
    println!("{}", ui::section("Setup"));
    println!("  {}", ui::bold("arc init"));
    println!();

    println!("{}", ui::section("Config"));
    println!("  {}", ui::bold("arc config path"));
    println!("  {}", ui::bold("arc config check"));
    println!("  {}", ui::bold("arc config show"));
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

    println!("{}", ui::dim("Tip: use arc.toml to configure the policy."));
}

pub fn print_config_usage() {
    println!("{}", ui::section("Config usage"));
    println!("  {}", ui::bold("arc config path"));
    println!("  {}", ui::bold("arc config check"));
    println!("  {}", ui::bold("arc config show"));
}
