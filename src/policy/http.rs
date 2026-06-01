// ─── < Imports > ────────────────────────────────────────────────────

use crate::config::Config;
use crate::decision::{Decision, DecisionReason, RiskLevel};
use crate::http_target;
use crate::request::Request;

// ─── < Public Functions > ───────────────────────────────────────────

pub fn decide(request: &Request, config: &Config) -> Option<Decision> {
    if !request.is_http_get() {
        return None;
    }

    if !http_target::is_valid_http_url(&request.resource) {
        return Some(Decision::deny_with_risk(DecisionReason::InvalidHttpUrl, RiskLevel::Medium));
    }

    if config.http.is_blocked_target(&request.resource) {
        return Some(Decision::deny_with_risk(DecisionReason::HttpTargetBlocked, RiskLevel::High));
    }

    None
}
