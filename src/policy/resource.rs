// ─── < Imports > ────────────────────────────────────────────────────

use crate::config::Config;
use crate::decision::{Decision, DecisionReason, RiskLevel};
use crate::request::Request;

// ─── < Public Functions > ───────────────────────────────────────────

pub fn decide(request: &Request, config: &Config) -> Option<Decision> {
    if !request.has_resource() {
        return None;
    }

    if config.resources.is_protected_resource(&request.resource) {
        return Some(Decision::deny_with_risk(DecisionReason::ResourceProtected, RiskLevel::Critical));
    }

    if config.resources.is_blocked_path(&request.resource) {
        return Some(Decision::deny_with_risk(DecisionReason::PathBlocked, RiskLevel::Critical));
    }

    None
}
