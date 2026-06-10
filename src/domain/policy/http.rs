// ─── < Imports > ────────────────────────────────────────────────────

use crate::decision::{Decision, DecisionReason, RiskLevel};
use crate::http_target;
use crate::request::Request;

use super::rules::PolicyRules;

// ─── < Public Functions > ───────────────────────────────────────────

pub fn decide(request: &Request, rules: &impl PolicyRules) -> Option<Decision> {
    if !request.is_http_get() {
        return None;
    }

    if !http_target::is_valid_http_url(&request.resource) {
        return Some(Decision::deny_with_risk(DecisionReason::InvalidHttpUrl, RiskLevel::Medium));
    }

    if rules.is_blocked_http_target(&request.resource) {
        return Some(Decision::deny_with_risk(DecisionReason::HttpTargetBlocked, RiskLevel::High));
    }

    None
}
