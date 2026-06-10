// ─── < Imports > ────────────────────────────────────────────────────

use crate::decision::Decision;

// ─── < Structs > ────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PolicyDecision {
    decision: Decision,
}

// ─── < Implementations > ────────────────────────────────────────────

impl PolicyDecision {
    pub fn new(decision: Decision) -> Self {
        Self { decision }
    }

    pub fn decision(&self) -> Decision {
        self.decision
    }

    pub fn into_decision(self) -> Decision {
        self.decision
    }
}
