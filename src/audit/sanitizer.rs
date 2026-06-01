// ─── < Constants > ──────────────────────────────────────────────────

const MAX_AUDIT_FIELD_CHARS: usize = 512;
const TRUNCATED_SUFFIX: &str = "…[truncated]";

const SENSITIVE_MARKERS: &[&str] = &[
    "api-key",
    "api_key",
    "apikey",
    "authorization",
    "bearer",
    "credential",
    "passwd",
    "password",
    "private-key",
    "private_key",
    "secret",
    "token",
];

// ─── < Public Functions > ───────────────────────────────────────────

pub fn sanitize_field(value: &str) -> String {
    truncate_field(&redact_sensitive_tokens(value))
}

// ─── < Private Functions > ──────────────────────────────────────────

fn redact_sensitive_tokens(value: &str) -> String {
    value.split_whitespace().map(redact_token).collect::<Vec<String>>().join(" ")
}

fn redact_token(token: &str) -> String {
    let normalized_token = token.to_ascii_lowercase();

    if normalized_token == "bearer" {
        return "[redacted]".to_string();
    }

    if !contains_sensitive_marker(&normalized_token) {
        return token.to_string();
    }

    if let Some((key, _value)) = token.split_once('=') {
        return format!("{key}=[redacted]");
    }

    if let Some((key, _value)) = token.split_once(':') {
        return format!("{key}:[redacted]");
    }

    "[redacted]".to_string()
}

fn contains_sensitive_marker(value: &str) -> bool {
    SENSITIVE_MARKERS.iter().any(|marker| value.contains(marker))
}

fn truncate_field(value: &str) -> String {
    if value.chars().count() <= MAX_AUDIT_FIELD_CHARS {
        return value.to_string();
    }

    let retained_chars = MAX_AUDIT_FIELD_CHARS.saturating_sub(TRUNCATED_SUFFIX.chars().count());

    let mut truncated = value.chars().take(retained_chars).collect::<String>();

    truncated.push_str(TRUNCATED_SUFFIX);

    truncated
}
