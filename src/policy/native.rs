// ─── < Imports > ────────────────────────────────────────────────────

use crate::decision::{Decision, DecisionReason, RiskLevel};
use crate::request::Request;

use super::engine::PolicyEngine;
use super::input::PolicyInput;
use super::output::PolicyDecision;
use super::rules::{DefaultPolicyAction, PolicyRules};
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

fn decide_native(request: &Request, rules: &impl PolicyRules) -> Decision {
    if let Some(decision) = action::decide(request, rules) {
        return decision;
    }

    let action_decision = decide_action_policy(request, rules);

    if let Some(decision) = console::decide(request, rules) {
        if decision.is_denied() {
            return decision;
        }

        if decision.should_ask() && !action_decision.is_denied() {
            return decision;
        }
    }

    if let Some(decision) = resource::decide(request, rules) {
        return decision;
    }

    if let Some(decision) = http::decide(request, rules) {
        return decision;
    }

    action_decision
}

fn decide_action_policy(request: &Request, rules: &impl PolicyRules) -> Decision {
    let request_risk = risk::for_allowed_request(request);

    if rules.is_allowed_action(&request.action) {
        if rules.action_should_ask(&request.action) {
            return Decision::ask_with_risk(DecisionReason::ActionRequiresApproval, request_risk);
        }

        return Decision::allow_with_risk(DecisionReason::ActionAllowed, request_risk);
    }

    match rules.default_action() {
        DefaultPolicyAction::Allow => Decision::allow_with_risk(DecisionReason::ActionAllowedByDefault, request_risk),
        DefaultPolicyAction::Ask => Decision::ask_with_risk(DecisionReason::ActionRequiresApprovalByDefault, request_risk),
        DefaultPolicyAction::Deny => Decision::deny_with_risk(DecisionReason::ActionNotConfigured, RiskLevel::High),
    }
}
