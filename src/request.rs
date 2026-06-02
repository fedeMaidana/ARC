// ─── < Enums > ──────────────────────────────────────────────────────

#[derive(Debug)]
pub enum RequestMode {
    Execute,
    Check,
}

// ─── < Structs > ────────────────────────────────────────────────────

#[derive(Debug)]
pub struct Request {
    pub mode: RequestMode,
    pub action: String,
    pub resource: String,
    pub command_parts: Vec<String>,
}

// ─── < Implementations > ────────────────────────────────────────────

impl Request {
    pub fn new(mode: RequestMode, action: String, command_parts: Vec<String>) -> Self {
        let resource = command_parts.join(" ");

        Self {
            mode,
            action,
            resource,
            command_parts,
        }
    }

    pub fn has_resource(&self) -> bool {
        !self.resource.is_empty()
    }

    pub fn is_http_get(&self) -> bool {
        self.action == "http_get"
    }

    pub fn is_run_command(&self) -> bool {
        self.action == "run"
    }

    pub fn is_check_mode(&self) -> bool {
        matches!(self.mode, RequestMode::Check)
    }

    pub fn command_name(&self) -> Option<&str> {
        self.command_parts.first().map(|command| command.as_str())
    }

    pub fn command_args(&self) -> &[String] {
        if self.command_parts.len() <= 1 {
            &[]
        } else {
            &self.command_parts[1..]
        }
    }
}
