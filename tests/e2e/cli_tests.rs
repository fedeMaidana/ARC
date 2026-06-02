// ─── < Shared Test Helpers > ────────────────────────────────────────

#[path = "cli_tests/common.rs"]
mod common;

// ─── < E2E CLI Test Modules > ───────────────────────────────────────

#[path = "cli_tests/help_tests.rs"]
mod help_tests;

#[path = "cli_tests/config_tests.rs"]
mod config_tests;

#[path = "cli_tests/agent_tests.rs"]
mod agent_tests;

#[path = "cli_tests/human_flow_tests.rs"]
mod human_flow_tests;

#[path = "cli_tests/json_flow_tests.rs"]
mod json_flow_tests;
