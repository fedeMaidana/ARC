// ─── < Imports > ────────────────────────────────────────────────────

use crate::config::Config;
use crate::decision::{Decision, DecisionReason, RiskLevel};
use crate::request::Request;

// ─── < Public Functions > ───────────────────────────────────────────

pub fn decide(request: &Request, config: &Config) -> Option<Decision> {
    if !request.is_http_get() {
        return None;
    }

    if !is_valid_url(&request.resource) {
        return Some(Decision::deny_with_risk(DecisionReason::InvalidHttpUrl, RiskLevel::Medium));
    }

    if config.http.is_blocked_target(&request.resource) {
        return Some(Decision::deny_with_risk(DecisionReason::HttpTargetBlocked, RiskLevel::High));
    }

    None
}

// ─── < Private Functions > ──────────────────────────────────────────

fn is_valid_url(resource: &str) -> bool {
    resource.starts_with("https://") || resource.starts_with("http://")
}
