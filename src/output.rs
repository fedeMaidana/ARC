// ─── < Modules > ────────────────────────────────────────────────────

mod agents;
mod banner;
mod config;
mod decision;
mod doctor;
mod error;
mod execution;
mod shared;
mod shims;
mod usage;

// ─── < Public Exports > ─────────────────────────────────────────────

pub use self::agents::{print_agent_env_exports, print_agent_scan_results, print_agent_sync_report, print_agents};
pub use self::banner::print_banner;
pub use self::config::{
    print_init_result, print_settings, print_settings_check_error, print_settings_check_success, print_settings_source_path,
};
pub use self::decision::print_decision;
pub use self::doctor::print_doctor_report;
pub use self::error::{print_app_error, print_cli_error};
pub use self::execution::print_execution_report;
pub use self::shims::{print_shims_install_report, print_shims_list_report, print_shims_path};
pub use self::usage::{print_agents_usage, print_settings_usage, print_shims_usage, print_usage};
