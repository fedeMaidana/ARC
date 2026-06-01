// ─── < Imports > ────────────────────────────────────────────────────

use arc::resource;

// ─── < Tests: Normalization > ───────────────────────────────────────

#[test]
fn normalizes_current_dir_segments() {
    let normalized = resource::normalize_path_like_resource("./.env");

    assert_eq!(normalized, ".env");
}

#[test]
fn normalizes_parent_segments_inside_relative_path() {
    let normalized = resource::normalize_path_like_resource("config/../.env");

    assert_eq!(normalized, ".env");
}

#[test]
fn preserves_leading_parent_segments() {
    let normalized = resource::normalize_path_like_resource("../secrets.txt");

    assert_eq!(normalized, "../secrets.txt");
}

#[test]
fn normalizes_parent_segments_inside_absolute_path() {
    let normalized = resource::normalize_path_like_resource("/tmp/../etc/passwd");

    assert_eq!(normalized, "/etc/passwd");
}

#[test]
fn does_not_escape_above_root() {
    let normalized = resource::normalize_path_like_resource("/../etc/passwd");

    assert_eq!(normalized, "/etc/passwd");
}

// ─── < Tests: Protected Resources > ─────────────────────────────────

#[test]
fn matches_exact_protected_resource_after_normalization() {
    assert!(resource::matches_resource_name("./.env", ".env"));
    assert!(resource::matches_resource_name("config/../.env", ".env"));
}

#[test]
fn matches_nested_protected_resource() {
    assert!(resource::matches_resource_name("config/.env", ".env"));
}

#[test]
fn does_not_match_similar_resource_name() {
    assert!(!resource::matches_resource_name(".env.example", ".env"));
    assert!(!resource::matches_resource_name("config/.env.example", ".env"));
}

// ─── < Tests: Blocked Prefixes > ────────────────────────────────────

#[test]
fn matches_blocked_prefix_after_absolute_path_normalization() {
    assert!(resource::matches_path_prefix("/tmp/../etc/passwd", "/etc/"));
}

#[test]
fn matches_blocked_parent_prefix_after_relative_path_normalization() {
    assert!(resource::matches_path_prefix("config/../../secrets.txt", "../"));
}

#[test]
fn matches_exact_blocked_prefix_without_trailing_separator() {
    assert!(resource::matches_path_prefix("/etc", "/etc/"));
}

#[test]
fn does_not_match_similar_blocked_prefix() {
    assert!(!resource::matches_path_prefix("/etcpasswd", "/etc/"));
}
