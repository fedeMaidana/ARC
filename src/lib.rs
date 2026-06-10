// ─── < Domain (core) > ──────────────────────────────────────────────

pub mod decision;
pub mod http_target;
pub mod policy;
pub mod request;
pub mod resource;

// ─── < Application > ────────────────────────────────────────────────

pub(crate) mod application;

// ─── < Infrastructure (adapters) > ──────────────────────────────────

pub mod agent;
pub mod audit;
pub mod config;
pub(crate) mod doctor;
pub mod executor;
pub mod json_api;
pub(crate) mod shims;

// ─── < Presentation > ───────────────────────────────────────────────

pub(crate) mod ask;
pub mod cli;
pub(crate) mod output;
pub(crate) mod tui;
pub(crate) mod ui;

// ─── < Composition root > ───────────────────────────────────────────

pub mod app;
