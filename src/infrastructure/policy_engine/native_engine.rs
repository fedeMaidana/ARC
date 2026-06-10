// ─── < Imports > ────────────────────────────────────────────────────

use crate::domain::policy::native;

use super::engine::PolicyEngine;
use super::input::PolicyInput;
use super::output::PolicyDecision;

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
        PolicyDecision::new(native::decide(input.request(), input.config()))
    }
}
