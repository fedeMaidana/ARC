// ─── < Enums > ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecisionStatus {
    Allow,
    Deny,
    Ask,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecisionReason {
    ActionAllowed,
    ActionBlocked,
    ActionRequiresApproval,
    ResourceRequired,
    ConsoleCommandRequired,
    ConsoleCommandBlocked,
    ConsoleCommandNotAllowed,
    ConsoleCommandRequiresApproval,
    ConsoleSubcommandRequired,
    ConsoleSubcommandBlocked,
    ConsoleSubcommandNotAllowed,
    ConsoleSubcommandRequiresApproval,
    ConsoleArgumentBlocked,
    ConsoleArgumentRequiresApproval,
    ResourceProtected,
    PathBlocked,
    InvalidHttpUrl,
    HttpTargetBlocked,
    ActionNotConfigured,
}

// ─── < Structs > ────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Decision {
    pub status: DecisionStatus,
    pub reason: DecisionReason,
    pub risk: RiskLevel,
}

// ─── < Implementations > ────────────────────────────────────────────

impl DecisionStatus {
    pub fn as_text(&self) -> &'static str {
        match self {
            DecisionStatus::Allow => "allow",
            DecisionStatus::Deny => "deny",
            DecisionStatus::Ask => "ask",
        }
    }
}

impl RiskLevel {
    pub fn as_text(&self) -> &'static str {
        match self {
            RiskLevel::Low => "low",
            RiskLevel::Medium => "medium",
            RiskLevel::High => "high",
            RiskLevel::Critical => "critical",
        }
    }
}

impl DecisionReason {
    pub fn as_text(&self) -> &'static str {
        match self {
            DecisionReason::ActionAllowed => "action is allowed",
            DecisionReason::ActionBlocked => "action is blocked",
            DecisionReason::ActionRequiresApproval => "action requires user approval",
            DecisionReason::ResourceRequired => "resource is required",
            DecisionReason::ConsoleCommandRequired => "console command is required",
            DecisionReason::ConsoleCommandBlocked => "console command is blocked",
            DecisionReason::ConsoleCommandNotAllowed => "console command is not allowed",
            DecisionReason::ConsoleCommandRequiresApproval => "console command requires user approval",
            DecisionReason::ConsoleSubcommandRequired => "console subcommand is required",
            DecisionReason::ConsoleSubcommandBlocked => "console subcommand is blocked",
            DecisionReason::ConsoleSubcommandNotAllowed => "console subcommand is not allowed",
            DecisionReason::ConsoleSubcommandRequiresApproval => "console subcommand requires user approval",
            DecisionReason::ConsoleArgumentBlocked => "console argument is blocked",
            DecisionReason::ConsoleArgumentRequiresApproval => "console argument requires user approval",
            DecisionReason::ResourceProtected => "resource is protected",
            DecisionReason::PathBlocked => "path is blocked",
            DecisionReason::InvalidHttpUrl => "invalid http url",
            DecisionReason::HttpTargetBlocked => "http target is blocked",
            DecisionReason::ActionNotConfigured => "action is not configured",
        }
    }

    pub fn as_code(&self) -> &'static str {
        match self {
            DecisionReason::ActionAllowed => "action_allowed",
            DecisionReason::ActionBlocked => "action_blocked",
            DecisionReason::ActionRequiresApproval => "action_requires_approval",
            DecisionReason::ResourceRequired => "resource_required",
            DecisionReason::ConsoleCommandRequired => "console_command_required",
            DecisionReason::ConsoleCommandBlocked => "console_command_blocked",
            DecisionReason::ConsoleCommandNotAllowed => "console_command_not_allowed",
            DecisionReason::ConsoleCommandRequiresApproval => "console_command_requires_approval",
            DecisionReason::ConsoleSubcommandRequired => "console_subcommand_required",
            DecisionReason::ConsoleSubcommandBlocked => "console_subcommand_blocked",
            DecisionReason::ConsoleSubcommandNotAllowed => "console_subcommand_not_allowed",
            DecisionReason::ConsoleSubcommandRequiresApproval => "console_subcommand_requires_approval",
            DecisionReason::ConsoleArgumentBlocked => "console_argument_blocked",
            DecisionReason::ConsoleArgumentRequiresApproval => "console_argument_requires_approval",
            DecisionReason::ResourceProtected => "resource_protected",
            DecisionReason::PathBlocked => "path_blocked",
            DecisionReason::InvalidHttpUrl => "invalid_http_url",
            DecisionReason::HttpTargetBlocked => "http_target_blocked",
            DecisionReason::ActionNotConfigured => "action_not_configured",
        }
    }
}

impl Decision {
    pub fn allow(reason: DecisionReason) -> Self {
        Self::allow_with_risk(reason, RiskLevel::Low)
    }

    pub fn deny(reason: DecisionReason) -> Self {
        Self::deny_with_risk(reason, RiskLevel::High)
    }

    pub fn ask(reason: DecisionReason) -> Self {
        Self::ask_with_risk(reason, RiskLevel::Medium)
    }

    pub fn allow_with_risk(reason: DecisionReason, risk: RiskLevel) -> Self {
        Self {
            status: DecisionStatus::Allow,
            reason,
            risk,
        }
    }

    pub fn deny_with_risk(reason: DecisionReason, risk: RiskLevel) -> Self {
        Self {
            status: DecisionStatus::Deny,
            reason,
            risk,
        }
    }

    pub fn ask_with_risk(reason: DecisionReason, risk: RiskLevel) -> Self {
        Self {
            status: DecisionStatus::Ask,
            reason,
            risk,
        }
    }

    pub fn is_allowed(&self) -> bool {
        matches!(self.status, DecisionStatus::Allow)
    }

    pub fn is_denied(&self) -> bool {
        matches!(self.status, DecisionStatus::Deny)
    }

    pub fn should_ask(&self) -> bool {
        matches!(self.status, DecisionStatus::Ask)
    }
}
