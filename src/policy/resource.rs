// ─── < Imports > ────────────────────────────────────────────────────

use crate::decision::{Decision, DecisionReason, RiskLevel};
use crate::request::Request;

use super::rules::PolicyRules;

// ─── < Public Functions > ───────────────────────────────────────────

pub fn decide(request: &Request, rules: &impl PolicyRules) -> Option<Decision> {
    if !request.has_resource() {
        return None;
    }

    if rules.is_protected_resource(&request.resource) {
        return Some(Decision::deny_with_risk(DecisionReason::ResourceProtected, RiskLevel::Critical));
    }

    if rules.is_blocked_path(&request.resource) {
        return Some(Decision::deny_with_risk(DecisionReason::PathBlocked, RiskLevel::Critical));
    }

    None
}
