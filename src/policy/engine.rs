// ─── < Imports > ────────────────────────────────────────────────────

use crate::config::Config;
use crate::decision::Decision;
use crate::request::Request;

use super::input::PolicyInput;
use super::native::NativePolicyEngine;
use super::output::PolicyDecision;

// ─── < Traits > ─────────────────────────────────────────────────────

pub trait PolicyEngine {
    fn decide(&self, input: PolicyInput<'_>) -> PolicyDecision;
}

// ─── < Public Functions > ───────────────────────────────────────────

pub fn decide(request: &Request, config: &Config) -> Decision {
    match config.policy.engine.as_str() {
        "native" => NativePolicyEngine::new().decide(PolicyInput::new(request, config)).into_decision(),
        _ => NativePolicyEngine::new().decide(PolicyInput::new(request, config)).into_decision(),
    }
}
