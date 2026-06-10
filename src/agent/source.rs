// ─── < Structs > ────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentSource {
    id: String,
    status: AgentSourceStatus,
}

// ─── < Enums > ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentSourceStatus {
    Builtin,
    Registered,
    Unknown,
}

// ─── < Implementations > ────────────────────────────────────────────

impl AgentSource {
    pub fn builtin(id: &str) -> Self {
        Self {
            id: id.to_string(),
            status: AgentSourceStatus::Builtin,
        }
    }

    pub fn registered(id: &str) -> Self {
        Self {
            id: id.to_string(),
            status: AgentSourceStatus::Registered,
        }
    }

    pub fn unknown(id: &str) -> Self {
        Self {
            id: id.to_string(),
            status: AgentSourceStatus::Unknown,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn status(&self) -> AgentSourceStatus {
        self.status
    }

    pub fn status_text(&self) -> &'static str {
        self.status.as_text()
    }
}

impl AgentSourceStatus {
    pub fn as_text(self) -> &'static str {
        match self {
            Self::Builtin => "builtin",
            Self::Registered => "registered",
            Self::Unknown => "unknown",
        }
    }
}
