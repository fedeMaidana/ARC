// ─── < Imports > ────────────────────────────────────────────────────

use crate::decision::{Decision, DecisionReason, RiskLevel};
use crate::request::Request;

use super::rules::PolicyRules;

// ─── < Public Functions > ───────────────────────────────────────────

pub fn decide(request: &Request, rules: &impl PolicyRules) -> Option<Decision> {
    if rules.is_blocked_action(&request.action) {
        return Some(Decision::deny_with_risk(DecisionReason::ActionBlocked, RiskLevel::Critical));
    }

    if rules.action_needs_resource(&request.action) && !request.has_resource() {
        return Some(Decision::deny_with_risk(DecisionReason::ResourceRequired, RiskLevel::Medium));
    }

    None
}
