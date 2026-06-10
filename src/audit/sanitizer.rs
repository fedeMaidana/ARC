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
    let mut output: Vec<String> = Vec::new();
    let mut redact_next = false;

    for token in value.split_whitespace() {
        if redact_next {
            output.push("[redacted]".to_string());
            redact_next = false;
            continue;
        }

        let (replacement, should_redact_next) = redact_token(token);

        output.push(replacement);
        redact_next = should_redact_next;
    }

    output.join(" ")
}

fn redact_token(token: &str) -> (String, bool) {
    let normalized_token = token.to_ascii_lowercase();

    if !contains_sensitive_marker(&normalized_token) {
        return (token.to_string(), false);
    }

    if let Some((key, _value)) = token.split_once('=') {
        return (format!("{key}=[redacted]"), false);
    }

    if let Some((key, _value)) = token.split_once(':') {
        return (format!("{key}:[redacted]"), false);
    }

    ("[redacted]".to_string(), true)
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
