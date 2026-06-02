// ─── < Imports > ────────────────────────────────────────────────────

use crate::config::{Config, DefaultPolicyAction};
use crate::decision::{Decision, DecisionReason, RiskLevel};
use crate::request::Request;

use super::engine::PolicyEngine;
use super::input::PolicyInput;
use super::output::PolicyDecision;
use super::{action, console, http, resource, risk};

// ─── < Structs > ────────────────────────────────────────────────────

#[derive(Debug, Default, Clone, Copy)]
pub struct NativePolicyEngine;

// ─── < Implementations > ────────────────────────────────────────────

impl NativePolicyEngine {
    pub fn new() -> Self {
        Self
    }
}

impl PolicyEngine for NativePolicyEngine {
    fn decide(&self, input: PolicyInput<'_>) -> PolicyDecision {
        PolicyDecision::new(decide_native(input.request(), input.config()))
    }
}

// ─── < Private Functions > ──────────────────────────────────────────

fn decide_native(request: &Request, config: &Config) -> Decision {
    if let Some(decision) = action::decide(request, config) {
        return decision;
    }

    let action_decision = decide_action_policy(request, config);

    if let Some(decision) = console::decide(request, config) {
        if decision.is_denied() {
            return decision;
        }

        if decision.should_ask() && !action_decision.is_denied() {
            return decision;
        }
    }

    if let Some(decision) = resource::decide(request, config) {
        return decision;
    }

    if let Some(decision) = http::decide(request, config) {
        return decision;
    }

    action_decision
}

fn decide_action_policy(request: &Request, config: &Config) -> Decision {
    let request_risk = risk::for_allowed_request(request);

    if config.actions.is_allowed_action(&request.action) {
        if config.actions.action_should_ask(&request.action) {
            return Decision::ask_with_risk(DecisionReason::ActionRequiresApproval, request_risk);
        }

        return Decision::allow_with_risk(DecisionReason::ActionAllowed, request_risk);
    }

    match config.policy.default_action_policy() {
        DefaultPolicyAction::Allow => Decision::allow_with_risk(DecisionReason::ActionAllowedByDefault, request_risk),
        DefaultPolicyAction::Ask => Decision::ask_with_risk(DecisionReason::ActionRequiresApprovalByDefault, request_risk),
        DefaultPolicyAction::Deny => Decision::deny_with_risk(DecisionReason::ActionNotConfigured, RiskLevel::High),
    }
}
