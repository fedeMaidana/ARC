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
    ActionAllowedByDefault,
    ActionRequiresApprovalByDefault,
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
    PolicyEngineFailed,
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

    pub fn from_text(value: &str) -> Option<Self> {
        match value {
            "allow" => Some(Self::Allow),
            "deny" => Some(Self::Deny),
            "ask" => Some(Self::Ask),
            _ => None,
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

    pub fn from_text(value: &str) -> Option<Self> {
        match value {
            "low" => Some(Self::Low),
            "medium" => Some(Self::Medium),
            "high" => Some(Self::High),
            "critical" => Some(Self::Critical),
            _ => None,
        }
    }
}

impl DecisionReason {
    pub fn as_text(&self) -> &'static str {
        match self {
            DecisionReason::ActionAllowed => "request matches an allowed policy",
            DecisionReason::ActionBlocked => "action is explicitly blocked by policy",
            DecisionReason::ActionRequiresApproval => "action requires manual approval",
            DecisionReason::ActionAllowedByDefault => "action is allowed by default policy",
            DecisionReason::ActionRequiresApprovalByDefault => "action requires manual approval by default policy",
            DecisionReason::ResourceRequired => "action requires a resource",
            DecisionReason::ConsoleCommandRequired => "run action requires a command",
            DecisionReason::ConsoleCommandBlocked => "command is explicitly blocked by console policy",
            DecisionReason::ConsoleCommandNotAllowed => "command is not in the console allowlist",
            DecisionReason::ConsoleCommandRequiresApproval => "command requires manual approval",
            DecisionReason::ConsoleSubcommandRequired => "command requires a subcommand",
            DecisionReason::ConsoleSubcommandBlocked => "subcommand is explicitly blocked by command policy",
            DecisionReason::ConsoleSubcommandNotAllowed => "subcommand is not allowed for this command",
            DecisionReason::ConsoleSubcommandRequiresApproval => "subcommand requires manual approval",
            DecisionReason::ConsoleArgumentBlocked => "argument is blocked by policy",
            DecisionReason::ConsoleArgumentRequiresApproval => "argument requires manual approval",
            DecisionReason::ResourceProtected => "resource is protected by policy",
            DecisionReason::PathBlocked => "path is blocked by policy",
            DecisionReason::InvalidHttpUrl => "HTTP URL is invalid or unsupported",
            DecisionReason::HttpTargetBlocked => "HTTP target is blocked by policy",
            DecisionReason::ActionNotConfigured => "action is not configured in policy",
            DecisionReason::PolicyEngineFailed => "policy engine failed",
        }
    }

    pub fn as_code(&self) -> &'static str {
        match self {
            DecisionReason::ActionAllowed => "action_allowed",
            DecisionReason::ActionBlocked => "action_blocked",
            DecisionReason::ActionRequiresApproval => "action_requires_approval",
            DecisionReason::ActionAllowedByDefault => "action_allowed_by_default",
            DecisionReason::ActionRequiresApprovalByDefault => "action_requires_approval_by_default",
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
            DecisionReason::PolicyEngineFailed => "policy_engine_failed",
        }
    }

    pub fn from_code(value: &str) -> Option<Self> {
        match value {
            "action_allowed" => Some(Self::ActionAllowed),
            "action_blocked" => Some(Self::ActionBlocked),
            "action_requires_approval" => Some(Self::ActionRequiresApproval),
            "action_allowed_by_default" => Some(Self::ActionAllowedByDefault),
            "action_requires_approval_by_default" => Some(Self::ActionRequiresApprovalByDefault),
            "resource_required" => Some(Self::ResourceRequired),
            "console_command_required" => Some(Self::ConsoleCommandRequired),
            "console_command_blocked" => Some(Self::ConsoleCommandBlocked),
            "console_command_not_allowed" => Some(Self::ConsoleCommandNotAllowed),
            "console_command_requires_approval" => Some(Self::ConsoleCommandRequiresApproval),
            "console_subcommand_required" => Some(Self::ConsoleSubcommandRequired),
            "console_subcommand_blocked" => Some(Self::ConsoleSubcommandBlocked),
            "console_subcommand_not_allowed" => Some(Self::ConsoleSubcommandNotAllowed),
            "console_subcommand_requires_approval" => Some(Self::ConsoleSubcommandRequiresApproval),
            "console_argument_blocked" => Some(Self::ConsoleArgumentBlocked),
            "console_argument_requires_approval" => Some(Self::ConsoleArgumentRequiresApproval),
            "resource_protected" => Some(Self::ResourceProtected),
            "path_blocked" => Some(Self::PathBlocked),
            "invalid_http_url" => Some(Self::InvalidHttpUrl),
            "http_target_blocked" => Some(Self::HttpTargetBlocked),
            "action_not_configured" => Some(Self::ActionNotConfigured),
            "policy_engine_failed" => Some(Self::PolicyEngineFailed),
            _ => None,
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
