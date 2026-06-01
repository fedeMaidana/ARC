// ─── < Imports > ────────────────────────────────────────────────────

use crate::ui;

// ─── < Public Functions > ───────────────────────────────────────────

pub fn print_usage() {
    println!("{}", ui::section("Usage"));
    println!("  {}", ui::bold("arc init"));
    println!("  {}", ui::bold("arc config path"));
    println!("  {}", ui::bold("arc config show"));
    println!("  {}", ui::bold("arc decide --json"));
    println!("  {}", ui::bold("arc run <command> [args...]"));
    println!("  {}", ui::bold("arc check run <command> [args...]"));
    println!();

    println!("{}", ui::section("Development"));
    println!("  cargo run -q -- init");
    println!("  cargo run -q -- config path");
    println!("  cargo run -q -- config show");
    println!("  echo '{{\"action\":\"run\",\"command\":[\"echo\",\"hola\"]}}' | cargo run -q -- decide --json");
    println!("  cargo run -q -- run ls -la");
    println!("  cargo run -q -- check run rm -rf /");
    println!();

    println!("{}", ui::dim("Tip: use arc.toml to configure the policy."));
}

pub fn print_config_usage() {
    println!("{}", ui::section("Config usage"));
    println!("  {}", ui::bold("arc config path"));
    println!("  {}", ui::bold("arc config show"));
}
