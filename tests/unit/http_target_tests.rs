// ─── < Imports > ────────────────────────────────────────────────────

use arc::http_target;

// ─── < Tests: URL Validation > ──────────────────────────────────────

#[test]
fn accepts_valid_http_url() {
    assert!(http_target::is_valid_http_url("http://example.com"));
}

#[test]
fn accepts_valid_https_url() {
    assert!(http_target::is_valid_http_url("https://example.com/docs"));
}

#[test]
fn rejects_url_without_scheme() {
    assert!(!http_target::is_valid_http_url("example.com"));
}

#[test]
fn rejects_unsupported_scheme() {
    assert!(!http_target::is_valid_http_url("file:///etc/passwd"));
    assert!(!http_target::is_valid_http_url("ftp://example.com/file.txt"));
}

#[test]
fn rejects_malformed_url() {
    assert!(!http_target::is_valid_http_url("http:///missing-host"));
}

// ─── < Tests: Blocked Target Matching > ─────────────────────────────

#[test]
fn matches_exact_blocked_target() {
    assert!(http_target::matches_blocked_target("http://localhost", "http://localhost"));
}

#[test]
fn matches_blocked_target_with_any_port_when_blocked_target_has_no_port() {
    assert!(http_target::matches_blocked_target("http://localhost:3000", "http://localhost"));
}

#[test]
fn matches_blocked_target_with_default_port() {
    assert!(http_target::matches_blocked_target("http://localhost:80", "http://localhost"));
}

#[test]
fn matches_blocked_target_case_insensitively() {
    assert!(http_target::matches_blocked_target("HTTP://LOCALHOST:3000", "http://localhost"));
}

#[test]
fn matches_blocked_ipv6_loopback_target() {
    assert!(http_target::matches_blocked_target("http://[::1]:8080", "http://[::1]"));
}

#[test]
fn matches_blocked_unspecified_ipv4_target() {
    assert!(http_target::matches_blocked_target("http://0.0.0.0:8080", "http://0.0.0.0"));
}

#[test]
fn does_not_match_different_scheme() {
    assert!(!http_target::matches_blocked_target("https://localhost", "http://localhost"));
}

#[test]
fn does_not_match_similar_domain() {
    assert!(!http_target::matches_blocked_target("http://localhost.evil.com", "http://localhost"));
}

#[test]
fn does_not_match_different_explicit_port() {
    assert!(!http_target::matches_blocked_target("http://localhost:3000", "http://localhost:8080"));
}

#[test]
fn matches_blocked_path_prefix() {
    assert!(http_target::matches_blocked_target("https://example.com/admin/users", "https://example.com/admin"));
}

#[test]
fn does_not_match_similar_path_prefix() {
    assert!(!http_target::matches_blocked_target("https://example.com/administrator", "https://example.com/admin"));
}
