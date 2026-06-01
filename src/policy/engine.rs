// ─── < Imports > ────────────────────────────────────────────────────

use crate::config::Config;
use crate::decision::{Decision, DecisionReason, RiskLevel};
use crate::request::Request;

use super::{action, console, http, resource, risk};

// ─── < Public Functions > ───────────────────────────────────────────

pub fn decide(request: &Request, config: &Config) -> Decision {
    if let Some(decision) = action::decide(request, config) {
        return decision;
    }

    if let Some(decision) = console::decide(request, config) {
        return decision;
    }

    if let Some(decision) = resource::decide(request, config) {
        return decision;
    }

    if let Some(decision) = http::decide(request, config) {
        return decision;
    }

    decide_allowed_action(request, config)
}

// ─── < Private Functions > ──────────────────────────────────────────

fn decide_allowed_action(request: &Request, config: &Config) -> Decision {
    if !config.actions.is_allowed_action(&request.action) {
        return Decision::deny_with_risk(DecisionReason::ActionNotConfigured, RiskLevel::High);
    }

    let request_risk = risk::for_allowed_request(request);

    if config.actions.action_should_ask(&request.action) {
        return Decision::ask_with_risk(DecisionReason::ActionRequiresApproval, request_risk);
    }

    Decision::allow_with_risk(DecisionReason::ActionAllowed, request_risk)
}
