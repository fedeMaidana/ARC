// ─── < Capabilities > ───────────────────────────────────────────────

pub mod agent;
pub mod app;
pub mod audit;
pub mod config;
pub(crate) mod doctor;
pub mod executor;
pub(crate) mod interface;
pub(crate) mod matching;
pub mod policy;
pub(crate) mod reviewing;
pub(crate) mod shims;

// ─── < Stable Module Paths > ────────────────────────────────────────

pub(crate) use reviewing::application;
pub use reviewing::{decision, request};

pub use matching::{http_target, resource};

pub(crate) use interface::{ask, output, tui, ui};
pub use interface::{cli, json_api};
