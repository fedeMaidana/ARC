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
        let resource = join_command_parts(&command_parts);

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

// ─── < Private Functions > ──────────────────────────────────────────

fn join_command_parts(command_parts: &[String]) -> String {
    if command_parts.len() <= 1 {
        return command_parts.join(" ");
    }

    command_parts
        .iter()
        .map(|part| quote_part_if_needed(part))
        .collect::<Vec<String>>()
        .join(" ")
}

fn quote_part_if_needed(part: &str) -> String {
    let needs_quoting = part.is_empty()
        || part
            .chars()
            .any(|character| character.is_whitespace() || character == '\'' || character == '"');

    if !needs_quoting {
        return part.to_string();
    }

    format!("'{}'", part.replace('\'', "'\\''"))
}
