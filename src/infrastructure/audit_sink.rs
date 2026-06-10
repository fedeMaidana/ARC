// ─── < Modules > ────────────────────────────────────────────────────

mod error;
mod path;
mod writer;

// ─── < Public Exports > ─────────────────────────────────────────────

pub use self::error::AuditError;
pub use self::writer::{ensure_audit_log_is_writable, record_event};
