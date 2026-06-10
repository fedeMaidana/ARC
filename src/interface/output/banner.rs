// ─── < Imports > ────────────────────────────────────────────────────

use crate::ui;

// ─── < Public Functions > ───────────────────────────────────────────

pub fn print_banner() {
    println!();
    println!("{}", ui::cyan(&format!("🛡️  ARC v{}", env!("CARGO_PKG_VERSION"))));
    println!("{}", ui::dim("────────────────"));
    println!();
}
