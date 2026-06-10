// ─── < Imports > ────────────────────────────────────────────────────

use anyhow::Error;

use crate::ui;

// ─── < Public Functions > ───────────────────────────────────────────

pub fn print_app_error(error: &Error) {
    println!("{}", ui::red("⛔ Error"));
    println!("  {} {}", ui::dim("details:"), error);

    for cause in error.chain().skip(1) {
        println!("  {} {}", ui::dim("caused by:"), cause);
    }
}

pub fn print_cli_error(error: &impl std::fmt::Display) {
    println!("{}", ui::red("⛔ CLI error"));
    println!("  {} {}", ui::dim("details:"), error);
    println!();
}
