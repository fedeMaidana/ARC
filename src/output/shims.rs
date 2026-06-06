// ─── < Imports > ────────────────────────────────────────────────────

use std::path::Path;

use crate::shims::{ShimInstallReport, ShimListReport};
use crate::ui;

// ─── < Public Functions > ───────────────────────────────────────────

pub fn print_shims_path(launcher_dir: &Path, runtime_shims_dir: &Path) {
    println!("{}", ui::section("ARC shims paths"));
    println!("  {} {}", ui::bold("launchers"), launcher_dir.display());
    println!("  {} {}", ui::bold("runtime shims"), runtime_shims_dir.display());
    println!();
    println!("{}", ui::dim("Only the launcher directory should be added to your shell PATH."));
}

pub fn print_shims_install_report(report: &ShimInstallReport) {
    println!("{}", ui::green("✅ ARC launcher shims installed"));
    println!("  {} {}", ui::bold("launcher dir"), report.launcher_dir().display());
    println!();

    println!("{}", ui::section("Launchers"));

    if report.launchers().is_empty() {
        println!("  {}", ui::dim("none"));
        println!();
        println!("{}", ui::dim("Run `arc init` or `arc agents sync` first to register detected agents."));
        return;
    }

    for launcher in report.launchers() {
        println!("  - {}", ui::bold(launcher.command()));
        println!("    agent: {}", launcher.agent_id());
        println!("    shim: {}", launcher.shim_path().display());
        println!("    real: {}", launcher.real_path());
    }

    println!();
    println!("{}", ui::dim("Add this to your shell PATH if it is not already there:"));
    println!("  export PATH=\"{}:$PATH\"", report.launcher_dir().display());
}

pub fn print_shims_list_report(report: &ShimListReport) {
    println!("{}", ui::section("ARC launcher shims"));
    println!("  {} {}", ui::bold("launcher dir"), report.launcher_dir().display());
    println!();

    if report.launchers().is_empty() {
        println!("  {}", ui::dim("none"));
        println!();
        println!("{}", ui::dim("Run `arc init` or `arc agents sync` first to register detected agents."));
        return;
    }

    for launcher in report.launchers() {
        let status = if launcher.installed() {
            ui::green("installed")
        } else {
            ui::yellow("missing")
        };

        println!("  - {}", ui::bold(launcher.command()));
        println!("    agent: {}", launcher.agent_id());
        println!("    status: {status}");
        println!("    shim: {}", launcher.shim_path().display());
        println!("    real: {}", launcher.real_path());
    }
}
