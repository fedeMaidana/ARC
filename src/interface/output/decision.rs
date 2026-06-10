// ─── < Imports > ────────────────────────────────────────────────────

use crate::decision::{Decision, DecisionStatus, RiskLevel};
use crate::request::Request;
use crate::ui;

// ─── < Public Functions > ───────────────────────────────────────────

pub fn print_decision(request: &Request, decision: &Decision) {
    let (decision_icon, decision_text) = match decision.status {
        DecisionStatus::Allow => ("✅", ui::green(decision.status.as_text())),
        DecisionStatus::Deny => ("⛔", ui::red(decision.status.as_text())),
        DecisionStatus::Ask => ("⚠️", ui::yellow(decision.status.as_text())),
    };

    println!("{}", ui::section("Decision"));
    println!("  {} {}", ui::dim("action:  "), ui::bold(&request.action));

    if request.has_resource() {
        println!("  {} {}", ui::dim("resource:"), request.resource);
    } else {
        println!("  {} {}", ui::dim("resource:"), ui::dim("none"));
    }

    println!("  {} {} {}", ui::dim("result:  "), decision_icon, decision_text);

    println!("  {} {}", ui::dim("risk:    "), format_risk_level(decision.risk));

    println!("  {} {}", ui::dim("reason:  "), decision.reason.as_text());
}

// ─── < Private Functions > ──────────────────────────────────────────

fn format_risk_level(risk: RiskLevel) -> String {
    match risk {
        RiskLevel::Low => ui::green(risk.as_text()),
        RiskLevel::Medium => ui::yellow(risk.as_text()),
        RiskLevel::High => ui::red(risk.as_text()),
        RiskLevel::Critical => ui::red(risk.as_text()),
    }
}
