// ─── < Imports > ────────────────────────────────────────────────────

use crate::config::Config;
use crate::decision::{Decision, DecisionReason, RiskLevel};
use crate::request::Request;

use super::input::PolicyInput;
use super::native_engine::NativePolicyEngine;
use super::output::PolicyDecision;
use super::rego::RegoPolicyEngine;

// ─── < Traits > ─────────────────────────────────────────────────────

pub trait PolicyEngine {
    fn decide(&self, input: PolicyInput<'_>) -> PolicyDecision;
}

// ─── < Public Functions > ───────────────────────────────────────────

pub fn decide(request: &Request, config: &Config) -> Decision {
    let input = PolicyInput::new(request, config);

    match config.policy.engine.as_str() {
        "native" => NativePolicyEngine::new().decide(input).into_decision(),
        "rego" => RegoPolicyEngine::new().decide(input).into_decision(),
        _ => Decision::deny_with_risk(DecisionReason::PolicyEngineFailed, RiskLevel::Critical),
    }
}
