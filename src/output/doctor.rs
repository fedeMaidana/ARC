// ─── < Imports > ────────────────────────────────────────────────────

use crate::doctor::{DoctorCheckStatus, DoctorReport};
use crate::ui;

// ─── < Public Functions > ───────────────────────────────────────────

pub fn print_doctor_report(report: &DoctorReport) {
    println!("{}", ui::section("ARC doctor"));

    for check in report.checks() {
        let status = match check.status() {
            DoctorCheckStatus::Pass => ui::green("✅"),
            DoctorCheckStatus::Warn => ui::yellow("⚠️ "),
            DoctorCheckStatus::Fail => ui::red("❌"),
        };

        println!("  {status} {}", ui::bold(check.name()));
        println!("      {}", check.message());
    }

    println!();

    if report.has_failures() {
        println!("{}", ui::red("ARC is not ready yet."));
        println!("{}", ui::dim("Fix the failed checks and run `arc doctor` again."));
    } else if report.has_warnings() {
        println!("{}", ui::yellow("ARC is usable, but there are warnings."));
    } else {
        println!("{}", ui::green("ARC is ready."));
    }
}
