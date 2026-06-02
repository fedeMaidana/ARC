// ─── < Imports > ────────────────────────────────────────────────────

use arc::agent::{AgentSource, AgentSourceError, AgentSourceStatus, classify_source};
use arc::config::{AgentSourceConfig, AgentsConfig};

// ─── < Tests > ──────────────────────────────────────────────────────

#[test]
fn builtin_source_has_builtin_status() {
    let source = AgentSource::builtin("cli");

    assert_eq!(source.id(), "cli");
    assert_eq!(source.status(), AgentSourceStatus::Builtin);
    assert_eq!(source.status_text(), "builtin");
}

#[test]
fn classifies_enabled_registered_source() {
    let config = agents_config(false, vec![agent_source("opencode", true)]);

    let source = classify_source("opencode", &config).expect("source should be allowed");

    assert_eq!(source.id(), "opencode");
    assert_eq!(source.status(), AgentSourceStatus::Registered);
    assert_eq!(source.status_text(), "registered");
}

#[test]
fn rejects_disabled_registered_source() {
    let config = agents_config(true, vec![agent_source("opencode", false)]);

    let error = classify_source("opencode", &config).expect_err("source should be rejected");

    match error {
        AgentSourceError::Disabled { source_id } => {
            assert_eq!(source_id, "opencode");
        }
        _ => panic!("expected disabled source error"),
    }
}

#[test]
fn allows_unknown_source_when_unknown_sources_are_allowed() {
    let config = agents_config(true, vec![agent_source("opencode", true)]);

    let source = classify_source("custom-agent", &config).expect("source should be allowed");

    assert_eq!(source.id(), "custom-agent");
    assert_eq!(source.status(), AgentSourceStatus::Unknown);
    assert_eq!(source.status_text(), "unknown");
}

#[test]
fn rejects_unknown_source_when_unknown_sources_are_disabled() {
    let config = agents_config(false, vec![agent_source("opencode", true)]);

    let error = classify_source("custom-agent", &config).expect_err("source should be rejected");

    match error {
        AgentSourceError::Unknown { source_id } => {
            assert_eq!(source_id, "custom-agent");
        }
        _ => panic!("expected unknown source error"),
    }
}

#[test]
fn trims_source_before_classification() {
    let config = agents_config(false, vec![agent_source("opencode", true)]);

    let source = classify_source(" opencode ", &config).expect("source should be allowed");

    assert_eq!(source.id(), "opencode");
    assert_eq!(source.status(), AgentSourceStatus::Registered);
}

// ─── < Helpers > ────────────────────────────────────────────────────

fn agents_config(allow_unknown_sources: bool, sources: Vec<AgentSourceConfig>) -> AgentsConfig {
    AgentsConfig {
        allow_unknown_sources,
        sources,
    }
}

fn agent_source(id: &str, enabled: bool) -> AgentSourceConfig {
    AgentSourceConfig {
        id: id.to_string(),
        display_name: id.to_string(),
        enabled,
        kind: "local_agent".to_string(),
        description: None,
    }
}
