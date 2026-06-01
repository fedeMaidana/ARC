// ─── < Imports > ────────────────────────────────────────────────────

use crate::decision::RiskLevel;
use crate::request::Request;

// ─── < Public Functions > ───────────────────────────────────────────

pub fn for_allowed_request(request: &Request) -> RiskLevel {
    if request.is_http_get() {
        return RiskLevel::Medium;
    }

    RiskLevel::Low
}

pub fn for_allowed_console_command(command_name: &str) -> RiskLevel {
    match command_name {
        "echo" | "pwd" | "ls" => RiskLevel::Low,
        "cat" => RiskLevel::Medium,
        _ => RiskLevel::Medium,
    }
}
