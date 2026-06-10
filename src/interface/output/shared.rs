// ─── < Imports > ────────────────────────────────────────────────────

use crate::ui;

// ─── < Public Functions > ───────────────────────────────────────────

pub fn print_list(title: &str, items: &[String]) {
    println!("  {}", ui::bold(title));

    for item in items {
        println!("    - {item}");
    }

    println!();
}
