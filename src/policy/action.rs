// ─── < Imports > ────────────────────────────────────────────────────

use crate::config::Config;
use crate::decision::{Decision, DecisionReason, RiskLevel};
use crate::request::Request;

// ─── < Public Functions > ───────────────────────────────────────────

pub fn decide(request: &Request, config: &Config) -> Option<Decision> {
    if config.actions.is_blocked_action(&request.action) {
        return Some(Decision::deny_with_risk(DecisionReason::ActionBlocked, RiskLevel::Critical));
    }

    if config.actions.action_needs_resource(&request.action) && !request.has_resource() {
        return Some(Decision::deny_with_risk(DecisionReason::ResourceRequired, RiskLevel::Medium));
    }

    None
}
